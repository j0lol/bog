// i'm a pedant, sorry.
#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::allow_attributes,
    clippy::empty_enum_variants_with_brackets,
    clippy::empty_structs_with_brackets,
    clippy::error_impl_error,
    clippy::if_then_some_else_none,
    clippy::impl_trait_in_params,
    clippy::indexing_slicing,
    clippy::map_err_ignore,
    clippy::mod_module_files,
    clippy::mutex_atomic,
    clippy::mutex_integer,
    clippy::needless_raw_strings,
    clippy::str_to_string,
    clippy::try_err,
    clippy::unnecessary_self_imports,
    clippy::unused_trait_names,
    clippy::pub_use
)]
#![warn(
    clippy::pedantic,
    clippy::arbitrary_source_item_ordering,
    clippy::module_name_repetitions,
    clippy::pathbuf_init_then_push
)]
#![allow(
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::too_many_lines
)]

mod db;
mod error;
mod feed;
mod og_image;
mod other_pages;
pub mod post;
pub mod template;

use crate::{
    feed::feed as feed_handler,
    other_pages::{contact, index, projects},
};
use poem::{
    EndpointExt as _, Route, Server, endpoint::StaticFilesEndpoint, get, listener::TcpListener,
    middleware::CookieJarManager, web::Data,
};
use rusqlite::Connection;
use std::{
    env,
    sync::{Arc, Mutex},
};

type WrappedConnection = Arc<Mutex<Connection>>;
type W = WrappedConnection;
type D<T> = Data<T>;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::init();

    let conn = Arc::new(Mutex::new(db::connect()));

    let app = Route::new()
        .at("/", get(index))
        .at("/contact", get(contact))
        .at("/projects", get(projects))
        .at("/blog", get(post::list::list))
        .at(
            "/blog/new",
            get(post::admin::new_post).post(post::admin::submit_new_post),
        )
        .at("/blog/new/sync", poem::post(post::admin::update_draft))
        .at("/blog/edit/render", poem::post(post::admin::render_draft))
        .at(
            "/blog/edit/:slug",
            get(post::admin::edit_post).post(post::admin::submit_edited_post),
        )
        .at("/blog/:slug", get(post::view::view))
        .at("/og-image/:slug", get(og_image::og_image_handler))
        .at("/login", poem::post(post::admin::login))
        .at("/feed", get(feed_handler))
        .nest("/static", StaticFilesEndpoint::new("./static/"))
        .nest("/dist", StaticFilesEndpoint::new(env!("OUT_DIR")))
        .with(CookieJarManager::new())
        .data(conn.clone());

    let port = env::var("PORT").unwrap_or("3000".to_owned());

    println!("Listening on https://localhost:{port}");
    Server::new(TcpListener::bind(format!("0.0.0.0:{port}")))
        .name("hello-world")
        .run(app)
        .await
}
