// Modified from: https://github.com/rust-lang/crates_io_og_image
// Licensed under the MIT license

use crate::env::var;
use crate::post::{Post, format_date};
use crate::{D, W};
use poem::http::StatusCode;
use poem::web::headers::ContentType;
use poem::{Body, IntoResponse, Response, handler};
use serde::Serialize;
use std::borrow::Cow;
use std::collections::HashMap;
use std::error::Error;
use std::io::Read;
use std::path::{Path, PathBuf};
use tempfile::NamedTempFile;
use tokio::fs;
use tokio::process::Command;

/// Data structure containing information needed to generate an OpenGraph image
#[derive(Debug, Clone, Serialize)]
pub struct OgImageData<'a> {
    title: &'a str,
    subtitle: Option<&'a str>,
    datestring: &'a str,
}

/// Generator for creating OpenGraph images using the Typst typesetting system.
///
/// This struct manages the path to the Typst binary and provides methods for
/// generating PNG images from a Typst template.
pub struct OgImageGenerator {
    typst_binary_path: PathBuf,
    typst_font_path: Option<PathBuf>,
    oxipng_binary_path: PathBuf,
}

impl OgImageGenerator {
    /// Creates a new `OgImageGenerator` with default binary paths.
    ///
    /// Uses "typst" and "oxipng" as default binary paths, assuming they are
    /// available in PATH. Use [`with_typst_path()`](Self::with_typst_path) and
    /// [`with_oxipng_path()`](Self::with_oxipng_path) to customize the
    /// binary paths.
    ///
    /// # Examples
    ///
    /// ```
    /// use crates_io_og_image::OgImageGenerator;
    ///
    /// let generator = OgImageGenerator::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Detects the image format from the first few bytes using magic numbers.
    ///
    /// Returns the appropriate file extension for supported formats:
    /// - PNG: returns "png"
    /// - JPEG: returns "jpg"
    /// - Unsupported formats: returns None
    fn detect_image_format(bytes: &[u8]) -> Option<&'static str> {
        // PNG magic number: 89 50 4E 47 0D 0A 1A 0A
        if bytes.starts_with(&[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]) {
            return Some("png");
        }

        // JPEG magic number: FF D8 FF
        if bytes.starts_with(&[0xFF, 0xD8, 0xFF]) {
            return Some("jpg");
        }

        None
    }

    /// Creates a new `OgImageGenerator` using the `TYPST_PATH` environment variable.
    ///
    /// If the `TYPST_PATH` environment variable is set, uses that path.
    /// Otherwise, falls back to the default behavior (assumes "typst" is in PATH).
    ///
    /// # Examples
    ///
    /// ```
    /// use crates_io_og_image::OgImageGenerator;
    ///
    /// let generator = OgImageGenerator::from_environment()?;
    /// # Ok::<(), crates_io_og_image::OgImageError>(())
    /// ```
    pub fn from_environment() -> Result<Self, Box<dyn Error>> {
        // im lazy
        Ok(OgImageGenerator::default())
    }

    /// Sets the Typst binary path for the generator.
    ///
    /// This allows specifying a custom path to the Typst binary.
    /// If not set, defaults to "typst" which assumes the binary is available in PATH.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::PathBuf;
    /// use crates_io_og_image::OgImageGenerator;
    ///
    /// let generator = OgImageGenerator::default()
    ///     .with_typst_path(PathBuf::from("/usr/local/bin/typst"));
    /// ```
    pub fn with_typst_path(mut self, typst_path: PathBuf) -> Self {
        self.typst_binary_path = typst_path;
        self
    }

    /// Sets the font path for the Typst compiler.
    ///
    /// This allows specifying a custom directory where Typst will look for fonts
    /// during compilation in addition to the default system font discovery.
    /// If not set, Typst will use only its default font discovery.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::PathBuf;
    /// use crates_io_og_image::OgImageGenerator;
    ///
    /// let generator = OgImageGenerator::default()
    ///     .with_font_path(PathBuf::from("/usr/share/fonts"));
    /// ```
    pub fn with_font_path(mut self, font_path: PathBuf) -> Self {
        self.typst_font_path = Some(font_path);
        self
    }

    /// Sets the oxipng binary path for PNG optimization.
    ///
    /// This allows specifying a custom path to the oxipng binary for PNG optimization.
    /// If not set, defaults to "oxipng" which assumes the binary is available in PATH.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::PathBuf;
    /// use crates_io_og_image::OgImageGenerator;
    ///
    /// let generator = OgImageGenerator::default()
    ///     .with_oxipng_path(PathBuf::from("/usr/local/bin/oxipng"));
    /// ```
    pub fn with_oxipng_path(mut self, oxipng_path: PathBuf) -> Self {
        self.oxipng_binary_path = oxipng_path;
        self
    }

    /// Generates an OpenGraph image using the provided data.
    ///
    /// This method creates a temporary directory with all the necessary files
    /// to create the OpenGraph image, compiles it to PNG using the Typst
    /// binary, and returns the resulting image as a `NamedTempFile`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use crates_io_og_image::{OgImageGenerator, OgImageData, OgImageAuthorData, OgImageError};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), OgImageError> {
    /// let generator = OgImageGenerator::default();
    /// let data = OgImageData {
    ///     name: "my-crate",
    ///     version: "1.0.0",
    ///     description: Some("A sample crate"),
    ///     license: Some("MIT"),
    ///     tags: &["web", "api"],
    ///     authors: &[OgImageAuthorData { name: "user", avatar: None }],
    ///     lines_of_code: Some(5000),
    ///     crate_size: 100,
    ///     releases: 10,
    /// };
    /// let image_file = generator.generate(data).await?;
    /// println!("Generated image at: {:?}", image_file.path());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn generate(&self, data: OgImageData<'_>) -> Result<Vec<u8>, Box<dyn Error>> {
        // Create a temporary folder
        println!("Creating temporary folder...");
        let temp_dir = tempfile::tempdir()?;

        // Create assets directory
        println!("Creating assets directory...");
        let assets_dir = temp_dir.path().join("assets");
        fs::create_dir(&assets_dir).await?;

        // Copy assets
        println!("Copying assets...");
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

        // Copy the static Typst template file
        println!("Copying Typst template...");
        let template_content = include_str!("../template/og_image.typ");
        let typ_file_path = temp_dir.path().join("og-image.typ");
        fs::write(&typ_file_path, template_content).await?;

        // Create a named temp file for the output PNG
        println!("Creating output file...");
        let output_file = NamedTempFile::new()?;

        // Serialize data to JSON
        println!("Serializing data to JSON...");
        let json_data = serde_json::to_string(&data)?;

        // Run typst compile command with input data
        println!("Running typst compile command...");
        let mut command = Command::new(&self.typst_binary_path);
        command.arg("compile").arg("--format").arg("png");

        // Pass in the data and avatar map as JSON inputs
        println!("Passing in data and avatar map as JSON inputs...");
        let input = format!("data={json_data}");
        command.arg("--input").arg(input);
        // let input = format!("avatar_map={json_avatar_map}");
        // command.arg("--input").arg(input);

        // // Pass in the font path if specified
        // if let Some(font_path) = &self.typst_font_path {
        //     debug!(font_path = %font_path.display(), "Using custom font path");
        command.arg("--font-path").arg(&assets_dir);
        // } else {
        //     debug!("Using only system fonts");
        // }

        // Pass input and output file paths
        command.arg(&typ_file_path).arg(output_file.path());

        // Clear environment variables to avoid leaking sensitive data
        command.env_clear();

        // Preserve environment variables needed for font discovery
        if let Ok(path) = std::env::var("PATH") {
            command.env("PATH", path);
        }
        if let Ok(home) = std::env::var("HOME") {
            command.env("HOME", home);
        }

        println!("Running typst compile command...");
        let compilation_start_time = std::time::Instant::now();
        let output = command.output().await;
        let output = output.unwrap();
        let compilation_duration = compilation_start_time.elapsed();

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            println!("Typst compilation failed");
            return Err("Typst compilation failed".into());
        }

        // let output_size_bytes = fs::metadata(output_file.path()).await;
        // let output_size_bytes = output_size_bytes.map(|m| m.len()).unwrap_or(0);

        // After successful Typst compilation, optimize the PNG
        self.optimize_png(output_file.path()).await;

        let mut buf = vec![];
        {
            let mut file = std::fs::File::open(output_file.path())?;
            file.read_to_end(&mut buf)?;
        }
        Ok(buf)
    }

    /// Optimizes a PNG file using oxipng.
    ///
    /// This method attempts to reduce the file size of a PNG using lossless compression.
    /// All errors are handled internally and logged as warnings. The method never fails
    /// to ensure PNG optimization is truly optional.
    async fn optimize_png(&self, png_file: &Path) {
        let mut command = Command::new(&self.oxipng_binary_path);

        // Default optimization level for speed/compression balance
        command.arg("--opt").arg("2");

        // Remove safe-to-remove metadata
        command.arg("--strip").arg("safe");

        // Overwrite the input PNG file
        command.arg(png_file);

        // Clear environment variables to avoid leaking sensitive data
        command.env_clear();

        // Preserve environment variables needed for running oxipng
        if let Ok(path) = std::env::var("PATH") {
            command.env("PATH", path);
        }

        let output = command.output().await;

        match output {
            Ok(output) if output.status.success() => {}
            Ok(output) => {
                println!("PNG optimization failed, continuing with unoptimized image");
            }
            Err(err) => {
                println!("Failed to execute oxipng, continuing with unoptimized image");
            }
        }
    }
}

impl Default for OgImageGenerator {
    /// Creates a default `OgImageGenerator` with default binary paths.
    ///
    /// Uses "typst" and "oxipng" as default binary paths, assuming they are available in PATH.
    fn default() -> Self {
        Self {
            typst_binary_path: PathBuf::from("typst"),
            typst_font_path: None,
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
        // Contain mutex guard to avoid later await point
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
            Err(e) => Err(e).unwrap(),
        }
    };

    let og_image_data = OgImageData {
        title: &post.title,
        subtitle: post.subtitle.as_deref(),
        datestring: &format_date(post.creation_datetime),
    };
    let og_image_file: NamedTempFile = OgImageGenerator::new()
        .generate(og_image_data)
        .await
        .unwrap();

    let mut inner_file = og_image_file.as_file();
    let mut buf = vec![];
    inner_file.read_to_end(&mut buf).unwrap();

    let body = Body::from_vec(buf);

    Response::builder()
        .content_type("image/png")
        .body(body)
        .into_response()
}
