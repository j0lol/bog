use maud::{PreEscaped, html};
use poem::{IntoResponse, Response, handler, web::Data};

use super::{clock_icon, fetch::fetch_all_posts, format_date};
use crate::{
    D, W,
    error::Result,
    template::{footer, header_extra, navbar},
};

#[handler]
pub fn list_posts(Data(conn): D<&W>) -> Result<Response> {
    let posts = fetch_all_posts(conn)?;

    let markup = html! {
        ( header_extra(&html! {
            meta property="og:title" content="Jo's Blog";
        }) )
        div.wrapper {
            (navbar("/blog"))
            main {
                h1 { "Post list" }
                p {
                    "All posts in reverse chronological order. " a href="/feed" { "Atom/RSS feed" } "."
                }
                ul {
                    @for post in posts.iter().filter(|p| p.category.is_none()) {
                    li  {
                        a href={"/blog/" (post.slug) } { (PreEscaped(post.title.clone())) }
                        br;
                        span {
                            (clock_icon())
                        }
                        time datetime=(post.creation_datetime) { (format_date(post.creation_datetime)) }
                    }
                    }
                }

                h2 { "Trash" }
                ul {
                    @for post in posts.iter().filter(|p| p.category == Some("trash".to_string())) {
                    li  {
                        a href={"/blog/" (post.slug) } { (PreEscaped(post.title.clone())) }
                        br;
                        span {
                            (clock_icon())
                        }
                        time datetime=(post.creation_datetime) { (format_date(post.creation_datetime)) }
                    }
                    }

                }
            }
            ( footer() )
        }
    };

    Ok(markup.into_response())
}
