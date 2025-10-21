use super::format_date;
use super::{clock_icon, fetch::fetch_all_posts};
use crate::{
    D, W,
    error::Result,
    template::{footer, header_extra, navbar},
};
use maud::{PreEscaped, html};
use poem::{IntoResponse, handler, web::Data};

#[handler]
pub fn list_posts(Data(conn): D<&W>) -> impl IntoResponse {
    match list_posts_inner(Data(conn)) {
        Ok(response) => response.into_response(),
        Err(e) => e.into_response(),
    }
}

fn list_posts_inner(Data(conn): D<&W>) -> Result<maud::Markup> {
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

    Ok(markup)
}
