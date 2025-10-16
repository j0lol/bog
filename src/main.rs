mod db;
mod feed;
mod other_pages;
pub mod post;
pub mod template;

use crate::{
    feed::feed as feed_handler,
    other_pages::{contact, index, projects},
    post::{
        edit_post, list_posts, login, new_post, submit_edited_post, submit_new_post, update_draft,
        view_post,
    },
};
use poem::{
    EndpointExt, Route, Server, endpoint::StaticFilesEndpoint, get, listener::TcpListener,
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
    let conn = Arc::new(Mutex::new(db::connect()));

    let app = Route::new()
        .at("/", get(index))
        .at("/contact", get(contact))
        .at("/projects", get(projects))
        .at("/blog", get(list_posts))
        .at("/blog/new", get(new_post).post(submit_new_post))
        .at("/blog/new/sync", poem::post(update_draft))
        .at("/blog/edit/:slug", get(edit_post).post(submit_edited_post))
        .at("/blog/:slug", get(view_post))
        .at("/login", poem::post(login))
        .at("/feed", get(feed_handler))
        .nest("/static", StaticFilesEndpoint::new("./static/"))
        .with(CookieJarManager::new())
        .data(conn.clone());

    let port = env::var("PORT").unwrap_or("3000".to_string());

    Server::new(TcpListener::bind(format!("0.0.0.0:{port}")))
        .name("hello-world")
        .run(app)
        .await
}
