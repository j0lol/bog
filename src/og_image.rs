// Modified from: https://github.com/rust-lang/crates_io_og_image
// Licensed under the MIT license

use crate::post::{Post, format_date};
use crate::{D, W};
use poem::http::StatusCode;
use poem::{Body, IntoResponse, Response, handler};
use serde::Serialize;
use std::collections::HashMap;
use std::error::Error;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use tempfile::NamedTempFile;
use tokio::fs;
use tokio::process::Command;

#[derive(Debug, Clone, Serialize, Hash, PartialEq, Eq)]
pub struct OgImageData<'a> {
    title: &'a str,
    subtitle: Option<&'a str>,
    datestring: &'a str,
}

// Simple in-memory cache for rendered images
static IMAGE_CACHE: std::sync::OnceLock<Mutex<HashMap<String, Vec<u8>>>> =
    std::sync::OnceLock::new();

fn get_cache() -> &'static Mutex<HashMap<String, Vec<u8>>> {
    IMAGE_CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

fn cache_key(data: &OgImageData<'_>) -> String {
    format!(
        "{}|{}|{}",
        data.title,
        data.subtitle.unwrap_or(""),
        data.datestring
    )
}

pub struct OgImageGenerator {
    typst_binary_path: PathBuf,
    oxipng_binary_path: PathBuf,
}

impl OgImageGenerator {
    pub async fn generate(&self, data: OgImageData<'_>) -> Result<Vec<u8>, Box<dyn Error>> {
        // Check cache first
        let key = cache_key(&data);
        {
            let cache = get_cache().lock().unwrap();
            if let Some(cached_image) = cache.get(&key) {
                return Ok(cached_image.clone());
            }
        }

        let temp_dir = tempfile::tempdir()?;
        let assets_dir = temp_dir.path().join("assets");
        fs::create_dir(&assets_dir).await?;

        // Copy font assets
        fs::write(
            assets_dir.join("DMSans-Regular.ttf"),
            include_bytes!("../template/assets/DMSans-Regular.ttf"),
        )
        .await?;

        fs::write(
            assets_dir.join("DMSans-Italic.ttf"),
            include_bytes!("../template/assets/DMSans-Italic.ttf"),
        )
        .await?;

        fs::write(
            assets_dir.join("HeptaSlab-Regular.ttf"),
            include_bytes!("../template/assets/HeptaSlab-Regular.ttf"),
        )
        .await?;

        fs::write(
            assets_dir.join("HeptaSlab-Medium.ttf"),
            include_bytes!("../template/assets/HeptaSlab-Medium.ttf"),
        )
        .await?;

        fs::write(
            assets_dir.join("MapleMono-Medium.ttf"),
            include_bytes!("../template/assets/MapleMono-Medium.ttf"),
        )
        .await?;

        // Copy the Typst template
        let template_content = include_str!("../template/og_image.typ");
        let typ_file_path = temp_dir.path().join("og-image.typ");
        fs::write(&typ_file_path, template_content).await?;

        let output_file = NamedTempFile::new()?;
        let json_data = serde_json::to_string(&data)?;

        // Build typst command
        let mut command = Command::new(&self.typst_binary_path);
        command
            .arg("compile")
            .arg("--format")
            .arg("png")
            .arg("--input")
            .arg(format!("data={json_data}"))
            .arg("--font-path")
            .arg(&assets_dir)
            .arg(&typ_file_path)
            .arg(output_file.path());

        // Clear environment and preserve necessary vars
        command.env_clear();
        if let Ok(path) = std::env::var("PATH") {
            command.env("PATH", path);
        }
        if let Ok(home) = std::env::var("HOME") {
            command.env("HOME", home);
        }

        let output = command.output().await?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Typst compilation failed: {}", stderr).into());
        }

        // Optimize the PNG
        self.optimize_png(output_file.path()).await;

        // Read the generated image
        let mut buf = vec![];
        {
            let mut file = std::fs::File::open(output_file.path())?;
            file.read_to_end(&mut buf)?;
        }

        // Cache the result
        {
            let mut cache = get_cache().lock().unwrap();
            cache.insert(key, buf.clone());
        }

        Ok(buf)
    }

    async fn optimize_png(&self, png_file: &Path) {
        let mut command = Command::new(&self.oxipng_binary_path);
        command
            .arg("--opt")
            .arg("2")
            .arg("--strip")
            .arg("safe")
            .arg(png_file);

        command.env_clear();
        if let Ok(path) = std::env::var("PATH") {
            command.env("PATH", path);
        }

        let _ = command.output().await;
    }
}

impl Default for OgImageGenerator {
    fn default() -> Self {
        Self {
            typst_binary_path: PathBuf::from("typst"),
            oxipng_binary_path: PathBuf::from("oxipng"),
        }
    }
}

#[handler]
pub async fn og_image_handler(
    poem::web::Path(slug): poem::web::Path<String>,
    poem::web::Data(conn): D<&W>,
) -> Response {
    let post = {
        let conn = conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT title, contents, slug, subtitle, category, bsky_uri, creation_datetime FROM post WHERE slug = ?1").unwrap();

        match stmt.query_one([slug], |row| {
            Ok(Post {
                title: row.get(0).unwrap(),
                contents: row.get(1).unwrap(),
                slug: row.get(2).unwrap(),
                subtitle: row.get(3).unwrap(),
                category: row.get(4).unwrap(),
                bsky_uri: row.get(5).unwrap(),
                creation_datetime: row.get(6).unwrap(),
            })
        }) {
            Ok(v) => v,
            Err(rusqlite::Error::QueryReturnedNoRows) => {
                return (StatusCode::NOT_FOUND, "404 Not Found").into_response();
            }
            Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
        }
    };

    let og_image_data = OgImageData {
        title: &post.title,
        subtitle: post.subtitle.as_deref(),
        datestring: &format_date(post.creation_datetime),
    };

    match OgImageGenerator::default().generate(og_image_data).await {
        Ok(image_bytes) => Response::builder()
            .content_type("image/png")
            .body(Body::from_vec(image_bytes))
            .into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to generate image",
        )
            .into_response(),
    }
}
