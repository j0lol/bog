use crate::error::{AppError, Result};
use crate::post::{fetch::fetch_post, format_date};
use crate::{D, W};
use poem::{Body, IntoResponse, Response, handler};
use serde::Serialize;
use std::collections::HashMap;

use std::io::Read;
use std::path::PathBuf;
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
        data.subtitle.unwrap_or_default(),
        data.datestring
    )
}

pub struct OgImageGenerator {
    typst_binary_path: PathBuf,
}

impl OgImageGenerator {
    pub async fn generate(&self, data: OgImageData<'_>) -> Result<Vec<u8>> {
        // Check cache first
        let key = cache_key(&data);
        {
            let cache = get_cache().lock()?;
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
            return Err(AppError::internal_server_error(format!(
                "Typst compilation failed: {stderr}"
            )));
        }

        // Optimize the PNG
        if let Err(e) = self.optimize_png(output_file.path()).await {
            log::warn!("PNG optimization failed, using unoptimized version: {e}");
        }

        // Read the generated image
        let mut buf = vec![];
        {
            let mut file = std::fs::File::open(output_file.path())?;
            file.read_to_end(&mut buf)?;
        }

        // Cache the result
        {
            let mut cache = get_cache().lock()?;
            cache.insert(key, buf.clone());
        }

        Ok(buf)
    }

    async fn optimize_png(&self, png_file: &std::path::Path) -> Result<()> {
        let png_data = tokio::fs::read(png_file).await?;
        let mut options = oxipng::Options::from_preset(2);
        options.optimize_alpha = true;
        options.strip = oxipng::StripChunks::Safe;

        let optimized = oxipng::optimize_from_memory(&png_data, &options).map_err(|e| {
            AppError::internal_server_error(format!("PNG optimization failed: {e}"))
        })?;

        tokio::fs::write(png_file, optimized).await?;
        Ok(())
    }
}

impl Default for OgImageGenerator {
    fn default() -> Self {
        Self {
            typst_binary_path: PathBuf::from("typst"),
        }
    }
}

#[handler]
pub async fn og_image_handler(
    poem::web::Path(slug): poem::web::Path<String>,
    poem::web::Data(conn): D<&W>,
) -> Result<Response> {
    let post = fetch_post(slug, conn)?;

    let og_image_data = OgImageData {
        title: &post.title,
        subtitle: post.subtitle.as_deref(),
        datestring: &format_date(post.creation_datetime),
    };

    let image_bytes = OgImageGenerator::default().generate(og_image_data).await?;

    Ok(Response::builder()
        .content_type("image/png")
        .body(Body::from_vec(image_bytes))
        .into_response())
}
