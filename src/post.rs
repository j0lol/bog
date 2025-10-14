use crate::{
    D, W,
    template::{header, page},
};
use maud::{Markup, PreEscaped, html};
use poem::{
    IntoResponse, handler,
    web::{Data, Form, Redirect},
};
use serde::Deserialize;

struct Post {
    title: String,
    contents: String,
}

#[handler]
pub fn list_posts(Data(conn): D<&W>) -> Markup {
    let conn = conn.lock().unwrap();

    let mut stmt = conn.prepare("SELECT title, contents FROM post").unwrap();
    let post_iter = stmt
        .query_map([], |row| {
            Ok(Post {
                title: row.get(0).unwrap(),
                contents: row.get(1).unwrap(),
            })
        })
        .unwrap();

    page(
        html! {
            h1 { "Post list" }
            ol {
                @for post in post_iter.flatten() {
                    li  {
                        h2 { (post.title) }
                        ( PreEscaped (post.contents) )
                    }

                }
            }
        },
        "/blog",
    )
}

#[handler]
pub fn new_post() -> Markup {
    html! {
        ( header() )
        body {
            h1 { "Make a new post" }
            form method="POST" {
                label {
                    "Title"
                    input name="title" {}
                }
                label {
                    "Contents"
                    textarea name="contents" {}
                }
                button type="submit" { "Submit" }
            }
        }
    }
}

#[derive(Deserialize)]
struct SubmitNewPostForm {
    title: String,
    contents: String,
}
#[handler]
pub fn submit_new_post(
    Form(form): Form<SubmitNewPostForm>,
    Data(conn): D<&W>,
) -> impl IntoResponse {
    let conn = conn.lock().unwrap();

    let mut stmt = conn
        .prepare("INSERT INTO post (title, contents) VALUES (?1, ?2)")
        .unwrap();
    stmt.execute((form.title, form.contents)).unwrap();

    Redirect::see_other("/posts")
}
