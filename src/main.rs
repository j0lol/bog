mod db;
mod other_pages;
mod post;
pub mod template;

use crate::{
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
use std::sync::{Arc, Mutex};

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
        .nest("/static", StaticFilesEndpoint::new("./static/"))
        .with(CookieJarManager::new())
        .data(conn.clone());

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .name("hello-world")
        .run(app)
        .await
}
