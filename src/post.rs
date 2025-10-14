use crate::WrappedConnection;
use maud::{Markup, html};
use poem::{handler, web::Data};

struct Post {
    title: String,
    contents: String,
}

#[handler]
pub async fn list_posts(Data(conn): Data<&WrappedConnection>) -> Markup {
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

    html! {
        h1 { "Post list" }
        ol {
            @for post in post_iter.flatten() {
                li  {
                    h2 { (post.title) }
                    pre { (post.contents) }
                }

            }
        }
    }
}
