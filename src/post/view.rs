use super::{clock_icon, fetch::fetch_post};
use crate::{
    D, W,
    error::{AppError, Result},
    template::{
        SpeechCharacter, SpeechDetails, SpeechEmotion, footer, header_extra, navbar, render_speech,
    },
};
use lol_html::{RewriteStrSettings, element, html_content::ContentType, rewrite_str};
use maud::{PreEscaped, html};
use poem::{
    IntoResponse, Response, handler,
    web::{Data, Path},
};

use super::format_date;

#[handler]
pub fn view_post(Path(slug): Path<String>, Data(conn): D<&W>) -> Response {
    match view_post_inner(slug, Data(conn)) {
        Ok(response) => response,
        Err(e) => e.into_response(),
    }
}

fn view_post_inner(slug: String, Data(conn): D<&W>) -> Result<Response> {
    let post = fetch_post(slug, conn)?;

    let contents = rewrite_str(
        &post.contents,
        RewriteStrSettings {
            element_content_handlers: vec![element!("speech-box", |el| {
                let char = el.get_attribute("character").unwrap_or("deer".to_string());
                let emotion = el.get_attribute("emotion").unwrap_or("neutral".to_string());

                let SpeechDetails { class, alt, src } = render_speech(
                    &match char.as_ref() {
                        "you" => SpeechCharacter::You,
                        _ => SpeechCharacter::Deer,
                    },
                    &match emotion.as_ref() {
                        "worried" => SpeechEmotion::Worried,
                        "shocked" => SpeechEmotion::Shocked,
                        "happy" => SpeechEmotion::Happy,
                        _ => SpeechEmotion::Neutral,
                    },
                );

                el.set_tag_name("div")?;
                el.set_attribute("class", &format!("dialog speech {class}"))?;
                el.before(&format!(r#"<div class="dialog-box"> <img class="raw dialog profile" width="120" height="120" src="{src}" alt="{alt}"> "#), ContentType::Html);
                el.after("</div>", ContentType::Html);

                Ok(())
            })],
            ..RewriteStrSettings::new()
        },
    )
    .map_err(|e| AppError::internal_server_error(e.to_string()))?;

    let datestring = format_date(post.creation_datetime);

    let endpoint = &format!("/blog/{}", post.slug);
    let response = html! {
        ( header_extra(&html! {
            script defer src="/static/js/bsky-comments.js" {}
            script defer src="/static/js/footnotes.js" {}

            meta property="og:title" content={(post.title) " — Jo's Blog"};

            meta property="og:image" content={"/og-image/" (post.slug)};
            meta property="og:image:type" content="image/png";
            meta property="og:image:width" content="1200";
            meta property="og:image:height" content="630";
            meta property="og:image:alt" content={"A banner describing a blog post. The title is " (post.title) " and the publish time is " (datestring) ". The banner has a purple strip on the bottom with the website URL."};

            meta name="twitter:card" content="summary_large_image";
        }) )
        div.wrapper {
            (navbar(endpoint))
            article {
                h1.blog-head { (PreEscaped(post.title.clone() ))}
                @let subtitle = post.subtitle.clone().unwrap_or_default();
                span.blog-subhead { em { (subtitle) }}
                hr.frontmatter;
                p.blog-publish {
                    ( clock_icon() )
                    time datetime=(post.creation_datetime) { (datestring.clone()) }
                }

                (PreEscaped(contents))
            }

            @if post.bsky_uri.is_some() {
                section.page {
                    noop {}
                    bsky-comments post=[post.bsky_uri] {}
                }
            }

            ( footer() )
        }
    };

    Ok(response.into_response())
}
