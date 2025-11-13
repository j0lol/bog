use crate::{
    D, W,
    error::{AppError, Result},
    post::{fetch::fetch_post, format_date},
};
use derive_typst_intoval::{IntoDict, IntoValue};
use poem::{Body, IntoResponse, Response, handler};
use serde::Serialize;
use std::{
    collections::HashMap,
    sync::{Mutex, MutexGuard},
};
use typst::{
    foundations::{Dict, IntoValue},
    layout::PagedDocument,
};
use typst_as_lib::TypstEngine;

const TEMPLATE_FILE: &str = include_str!("../template/og_image.typ");
const FONTS: [&[u8]; 5] = [
    include_bytes!("../template/assets/DMSans-Regular.ttf"),
    include_bytes!("../template/assets/DMSans-Italic.ttf"),
    include_bytes!("../template/assets/HeptaSlab-Medium.ttf"),
    include_bytes!("../template/assets/HeptaSlab-Regular.ttf"),
    include_bytes!("../template/assets/MapleMono-Medium.ttf"),
];

#[derive(Debug, Clone, Serialize, Hash, PartialEq, Eq, IntoDict, IntoValue)]
pub struct OgImageData {
    title: String,
    subtitle: Option<String>,
    datestring: String,
}

impl From<OgImageData> for Dict {
    fn from(value: OgImageData) -> Self {
        value.into_dict()
    }
}

pub fn generate(data: &OgImageData) -> Result<Vec<u8>> {
    // Check cache first
    {
        let cache = cache::acquire().lock()?;
        if let Some(cached_image) = cache::get(&cache, data) {
            return Ok(cached_image);
        }
        // drop lock
    }

    let template = TypstEngine::builder()
        .main_file(TEMPLATE_FILE)
        .fonts(FONTS)
        .build();

    let document: PagedDocument = template.compile_with_input(data.clone()).output?;
    assert!(document.pages.len() == 1);

    let png = typst_render::render(&document.pages[0], 4.0)
        .encode_png()
        .map_err(|e| AppError::internal_server_error(format!("Typst compile error: {e}")))?;

    // Cache the result
    {
        let mut cache = cache::acquire().lock()?;
        cache::insert(&mut cache, data, png.clone());
        // drop lock
    }

    Ok(png)
}

#[handler]
pub fn og_image_handler(
    poem::web::Path(slug): poem::web::Path<String>,
    poem::web::Data(conn): D<&W>,
) -> Result<Response> {
    let post = fetch_post(slug, conn)?;

    let og_image_data = OgImageData {
        title: post.title,
        subtitle: post.subtitle,
        datestring: format_date(post.creation_datetime),
    };

    let image_bytes = generate(&og_image_data)?;

    Ok(Response::builder()
        .content_type("image/png")
        .body(Body::from_vec(image_bytes))
        .into_response())
}

mod cache {
    use super::{HashMap, Mutex, MutexGuard, OgImageData};

    type Cache = HashMap<String, Vec<u8>>;

    // Simple in-memory cache for rendered images
    static IMAGE_CACHE: std::sync::OnceLock<Mutex<Cache>> = std::sync::OnceLock::new();

    pub(super) fn acquire() -> &'static Mutex<Cache> {
        IMAGE_CACHE.get_or_init(|| Mutex::new(HashMap::new()))
    }

    pub(super) fn get(cache: &MutexGuard<'_, Cache>, key: &OgImageData) -> Option<Vec<u8>> {
        cache.get(&cache_key(key)).cloned()
    }

    pub(super) fn insert(cache: &mut MutexGuard<'_, Cache>, key: &OgImageData, val: Vec<u8>) {
        cache.insert(cache_key(key), val);
    }

    fn cache_key(data: &OgImageData) -> String {
        format!(
            "{}|{}|{}",
            data.title,
            data.subtitle.clone().unwrap_or_default(),
            data.datestring
        )
    }
}
