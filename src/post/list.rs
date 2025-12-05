use super::clock_icon;
use crate::{
    D, W,
    error::Result,
    other_pages::job_callout,
    post,
    template::{footer, header_extra, navbar},
};
use maud::{PreEscaped, html};
use poem::{IntoResponse as _, Response, handler, web::Data};

#[handler]
pub fn list(Data(conn): D<&W>) -> Result<Response> {
    let posts = post::fetch::all(conn)?;

    let markup = html! {
        ( header_extra(&html! {
            meta property="og:title" content="Jo's Blog";
        }) )
        div.wrapper {
            (navbar("/blog"))
            ( job_callout() )
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
                        time datetime=(post.creation_datetime) { (post::format_date(post.creation_datetime)) }
                    }
                    }
                }

                h2 { "Trash" }
                ul {
                    @for post in posts.iter().filter(|p| p.category == Some("trash".to_owned())) {
                    li  {
                        a href={"/blog/" (post.slug) } { (PreEscaped(post.title.clone())) }
                        br;
                        span {
                            (clock_icon())
                        }
                        time datetime=(post.creation_datetime) { (post::format_date(post.creation_datetime)) }
                    }
                    }

                }
            }
            ( footer() )
        }
    };

    Ok(markup.into_response())
}
