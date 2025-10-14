mod db;
mod post;

use crate::post::list_posts;
use maud::{Markup, html};
use poem::{EndpointExt, Route, Server, get, handler, listener::TcpListener};
use rusqlite::Connection;
use std::sync::{Arc, Mutex};

type WrappedConnection = Arc<Mutex<Connection>>;

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
        .data(conn.clone());

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .name("hello-world")
        .run(app)
        .await
}
