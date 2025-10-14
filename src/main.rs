mod db;
mod post;
pub mod template;

use crate::post::{list_posts, new_post, submit_new_post};
use maud::{Markup, html};
use poem::{
    EndpointExt, Route, Server, endpoint::StaticFilesEndpoint, get, handler, listener::TcpListener,
    web::Data,
};
use rusqlite::Connection;
use std::sync::{Arc, Mutex};

type WrappedConnection = Arc<Mutex<Connection>>;
type W = WrappedConnection;
type D<T> = Data<T>;

#[handler]
fn hello_world() -> Markup {
    html! {
        h1 { "Hello, World!" }
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let conn = Arc::new(Mutex::new(db::connect()));

    let app = Route::new()
        .at("/hello", get(hello_world))
        .at("/posts", get(list_posts))
        .at("/posts/new", get(new_post).post(submit_new_post))
        .nest("/static", StaticFilesEndpoint::new("./static/"))
        .data(conn.clone());

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .name("hello-world")
        .run(app)
        .await
}
