mod db;
mod post;
pub mod template;

use crate::{
    post::{list_posts, new_post, submit_new_post, view_post},
    template::{SpeechCharacter, SpeechEmotion, header, navbar, speech},
};
use maud::{Markup, PreEscaped, html};
use poem::{
    EndpointExt, Route, Server, endpoint::StaticFilesEndpoint, get, handler, listener::TcpListener,
    web::Data,
};
use rusqlite::Connection;
use std::{
    fs::read_to_string,
    sync::{Arc, Mutex},
};

type WrappedConnection = Arc<Mutex<Connection>>;
type W = WrappedConnection;
type D<T> = Data<T>;

#[handler]
fn index() -> Markup {
    let pronouns = ["she/her", "they/them", "it/its"];
    let genders = ["𐂂", "\u{2400}", "\u{2205}", "girl?", "stolen", "not"];

    let select_1 = html! {};
    let select_2 = html! {};

    html! {
        (header() )
        div.wrapper {
            ( navbar("/") )
            main {
                h1.fancy.page-head { "Hi!" }
                ( speech( SpeechCharacter::Deer, SpeechEmotion::Neutral, html! {
                    p {
                        "I'm Jo. "
                        label #my-pronouns { "My Pronouns are " (select_1) }
                        "and"
                        label { " my gender is" (select_2) }
                    }

                    p {
                        "I'm a CompSci graduate from the University of Sussex."
                        small { a href="/contact" { "(Hire me!)"} }
                    }
                }))
                "details"
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let conn = Arc::new(Mutex::new(db::connect()));

    let app = Route::new()
        .at("/", get(index))
        .at("/blog", get(list_posts))
        .at("/blog/new", get(new_post).post(submit_new_post))
        .at("/blog/:slug", get(view_post))
        .nest("/static", StaticFilesEndpoint::new("./static/"))
        .data(conn.clone());

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .name("hello-world")
        .run(app)
        .await
}
