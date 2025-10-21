use crate::{
    D, W,
    template::{
        SpeechCharacter, SpeechDetails, SpeechEmotion, footer, header_extra, navbar, page,
        render_speech,
    },
};
use chrono::{DateTime, Datelike, FixedOffset, Local, NaiveDateTime, Utc};
use lol_html::{RewriteStrSettings, element, html_content::ContentType, rewrite_str};
use maud::{Markup, PreEscaped, html};
use poem::{
    IntoResponse, Response, handler,
    http::StatusCode,
    web::{
        Data, Form, Json, Path, Redirect,
        cookie::{Cookie, CookieJar},
    },
};
use serde::Deserialize;
use std::fs::read_to_string;

const ISO8601_DATE: &str = "%Y-%m-%dT%H:%M";

fn clock_icon() -> Markup {
    html! {
        span.emoji-icon {( "🕒" )}
    }
}

#[derive(Deserialize, Debug)]
pub struct Post {
    pub title: String,
    pub contents: String,
    pub slug: String,
    pub subtitle: Option<String>,
    pub category: Option<String>,
    pub bsky_uri: Option<String>,
    pub creation_datetime: DateTime<Local>,
}

pub fn fetch_all_posts(conn: &W) -> Vec<Post> {
    let conn = conn.lock().unwrap();

    let mut stmt = conn.prepare("SELECT title, contents, slug, subtitle, category, bsky_uri, creation_datetime FROM post").unwrap();
    let post_iter = stmt
        .query_map([], |row| {
            Ok(Post {
                title: row.get(0).unwrap(),
                contents: row.get(1).unwrap(),
                slug: row.get(2).unwrap(),
                subtitle: row.get(3).unwrap(),
                category: row.get(4).unwrap(),
                bsky_uri: row.get(5).unwrap(),
                creation_datetime: row.get(6).unwrap(),
            })
        })
        .unwrap()
        .flatten();

    let mut posts = post_iter.collect::<Vec<_>>();
    posts.sort_by(|a, b| {
        a.creation_datetime
            .partial_cmp(&b.creation_datetime)
            .unwrap()
    });
    posts.reverse(); // *reverse chronological*

    posts
}

#[handler]
pub fn list_posts(Data(conn): D<&W>) -> Markup {
    let posts = fetch_all_posts(conn);
    page(
        html! {
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
        },
        "/blog",
    )
}

#[handler]
pub fn view_post(Path(slug): Path<String>, Data(conn): D<&W>) -> Response {
    let conn = conn.lock().unwrap();

    let mut stmt = conn.prepare("SELECT title, contents, slug, subtitle, category, bsky_uri, creation_datetime FROM post WHERE slug = ?1").unwrap();
    let post = match stmt.query_one([slug], |row| {
        Ok(Post {
            title: row.get(0).unwrap(),
            contents: row.get(1).unwrap(),
            slug: row.get(2).unwrap(),
            subtitle: row.get(3).unwrap(),
            category: row.get(4).unwrap(),
            bsky_uri: row.get(5).unwrap(),
            creation_datetime: row.get(6).unwrap(),
        })
    }) {
        Ok(v) => v,
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            return (StatusCode::NOT_FOUND, "404 Not Found").into_response();
        }
        Err(e) => Err(e).unwrap(),
    };

    let contents = rewrite_str(&post.contents, RewriteStrSettings {

        element_content_handlers: vec![element!("speech-box", |el| {
            let char = el.get_attribute("character").unwrap_or("deer".to_string());
            let emotion = el.get_attribute("emotion").unwrap_or("neutral".to_string());

            let SpeechDetails { class, alt, src } = render_speech(match char.as_ref() {
                "you" => SpeechCharacter::You,
                "deer" => SpeechCharacter::Deer,
                _ => SpeechCharacter::Deer,
            }, match emotion.as_ref() {
                "worried" => SpeechEmotion::Worried,
                "shocked" => SpeechEmotion::Shocked,
                "happy" => SpeechEmotion::Happy,
                "neutral" => SpeechEmotion::Neutral,
                _ => SpeechEmotion::Neutral,
            });

            el.set_tag_name("div")?;
            el.set_attribute("class", &format!("dialog speech {class}"))?;
            el.before(&format!(r#"<div class="dialog-box"> <img class="raw dialog profile" width="120" height="120" src="{src}" alt="{alt}"> "#), ContentType::Html);
            el.after("</div>", ContentType::Html);

            Ok(())
        })],
        ..RewriteStrSettings::new()
    }).unwrap();

    {
        let markup = html! {
            // samp {( format!("{post:#?}") )}
            h1.blog-head { (PreEscaped(post.title.clone() ))}
            @let subtitle = post.subtitle.clone().unwrap_or("".to_string());
            span.blog-subhead { em { (subtitle) }}
            hr.frontmatter;
            p.blog-publish {
                ( clock_icon() )
                time datetime=(post.creation_datetime) { (format_date(post.creation_datetime)) }
            }

            (PreEscaped(contents))
        };
        let endpoint = &format!("/blog/{}", post.slug);
        html! {
            ( header_extra(html! {
                script defer src="/static/js/bsky-comments.js" {}
                script defer src="/static/js/footnotes.js" {}

                meta property="og:title" content={(post.title) " — Jo's Blog"};

                meta property="og:image" content={"/og-image/" (post.slug)};
            }) )
            div.wrapper {
                (navbar(endpoint))
                article {
                    (markup)
                }

                @if post.bsky_uri.is_some() {
                    section.page {
                        noop {}
                        bsky-comments post=[post.bsky_uri] {}
                    }
                }

                ( footer() )
            }
        }
        .into_response()
    }
}

fn read_secret() -> String {
    read_to_string("./.secret").unwrap().trim().to_string()
}

#[handler]
pub async fn login(body: String, cookie_jar: &CookieJar) -> impl IntoResponse {
    cookie_jar.add(Cookie::new_with_str("secret_pass", body));

    html! {(cookie_jar.get("secret_pass").unwrap())}.into_response()
}

#[handler]
pub fn new_post(cookie_jar: &CookieJar, Data(conn): D<&W>) -> impl IntoResponse {
    // Auth check!
    let secret = read_secret();
    match cookie_jar.get("secret_pass") {
        Some(cookie) if cookie.value_str() == secret => {}
        _ => return (StatusCode::UNAUTHORIZED, "Unauthorized.").into_response(),
    }

    let conn = conn.lock().unwrap();

    let mut stmt = conn.prepare("SELECT title, contents, slug, subtitle, category, bsky_uri, creation_datetime FROM draft").unwrap();
    let post: Post = stmt
        .query_one([], |row| {
            Ok(Post {
                title: row.get(0).unwrap(),
                contents: row.get(1).unwrap(),
                slug: row.get(2).unwrap(),
                subtitle: row.get(3).unwrap(),
                category: row.get(4).unwrap(),
                bsky_uri: row.get(5).unwrap(),
                creation_datetime: row.get(6).unwrap(),
            })
        })
        .unwrap();

    html! {
        ( header_extra(html! {
            script defer src="/static/js/footnotes.js" {}
        }) )
        body {
            h1 { "Make a new post" }
            form method="POST" {
                label {
                    "Title: "
                    input name="title" value=(post.title) {}
                }
                label {
                    "Slug: "
                    input name="slug" value=(post.slug) {}
                }
                label {
                    "dtl: "
                    input name="creation_datetime" type="datetime-local" value=(post.creation_datetime.format(ISO8601_DATE)) {}
                }
                label {
                    "Subtitle: "
                    input name="subtitle" value=[post.subtitle] {}
                }
                label {
                    "Category: "
                    input name="category" value=[post.category] {}
                }
                label {
                    "bsky_uri:  "
                    input name="bsky_uri" value=[post.bsky_uri] {}
                    a href="https://pdsls.dev" {"pdsls"}
                }
                br;
                br;
                div #editorWrapper {
                    textarea #editor name="contents" { (post.contents) }
                    div #editorPreview { "hii :3" }
                }
                br;
                span { "Your draft is auto saved..." }

                // Prevent implicit submission of the form (with enter)
                button type="submit" disabled style="display: none" aria-hidden="true";

                button type="submit" { "Publish" }
            }

            script type="module" src="/static/js/post-new.js" {}
            script type="module" src="/static/js/post-preview.js" {}
        }
    }.into_response()
}

#[derive(Deserialize)]
struct SubmitNewPostForm {
    title: String,
    contents: String,
    slug: String,
    subtitle: Option<String>,
    category: Option<String>,
    bsky_uri: Option<String>,
    creation_datetime: String,
}

#[handler]
pub fn submit_new_post(
    cookie_jar: &CookieJar,
    Form(form): Form<SubmitNewPostForm>,
    Data(conn): D<&W>,
) -> impl IntoResponse {
    // Auth check!
    let secret = read_secret();
    match cookie_jar.get("secret_pass") {
        Some(cookie) if cookie.value_str() == secret => {}
        _ => return (StatusCode::UNAUTHORIZED, "Unauthorized.").into_response(),
    }

    let conn = conn.lock().unwrap();

    let creation_datetime = parse_date(form.creation_datetime);

    let mut stmt = conn
        .prepare("INSERT INTO post (title, contents, slug, subtitle, category, bsky_uri, creation_datetime) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)")
        .unwrap();
    stmt.execute((
        form.title,
        form.contents,
        form.slug.clone(),
        clean(form.subtitle),
        clean(form.category),
        clean(form.bsky_uri),
        creation_datetime,
    ))
    .expect("failed query");

    Redirect::see_other(format!("/blog/{}", form.slug)).into_response()
}

fn eng_ordinal_suffix(n: usize) -> String {
    // https://stackoverflow.com/a/31615643
    const S: [&str; 4] = ["th", "st", "nd", "rd"];
    let v: usize = n % 100;

    let a = S.get((v.overflowing_sub(20).0) % 10);
    let b = S.get(v);
    let c = S.first();

    a.or(b).or(c).unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use crate::post::eng_ordinal_suffix;

    #[test]
    fn testsuffix() {
        let nums = [0, 1, 2, 3, 4, 10, 11, 12, 13, 14, 20, 21, 22, 100, 101, 111];
        let nums_ordinal = [
            "0th", "1st", "2nd", "3rd", "4th", "10th", "11th", "12th", "13th", "14th", "20th",
            "21st", "22nd", "100th", "101st", "111th",
        ];

        for (x, y) in nums.iter().zip(nums_ordinal) {
            let x_ = format!("{x}{}", eng_ordinal_suffix(*x));
            assert_eq!(x_, y)
        }
    }
}

pub fn format_date(date: chrono::DateTime<Local>) -> String {
    //let sfx = eng_ordinal_suffix(date.day() as usize);
    date.format(&format!("%B %d, %Y")).to_string()
}

fn parse_date(datestring: String) -> chrono::DateTime<FixedOffset> {
    match chrono::DateTime::parse_from_rfc3339(&datestring) {
        Ok(dt) => dt,
        Err(e) => {
            // println!(
            //     "Dt parse error: {e}. String: {datestring}. Attempting fallback no-tz ISO8601 parsing."
            // );
            match NaiveDateTime::parse_from_str(&datestring, ISO8601_DATE)
                .map(|ndt| DateTime::<Utc>::from_naive_utc_and_offset(ndt, Utc))
            {
                Ok(dt) => {
                    // println!("Success parsing ISO8601 date.");

                    dt.into()
                }
                Err(e) => {
                    println!(
                        "Dt parse error: {e}. String: {datestring}. Falling back to UNIX_EPOCH."
                    );

                    chrono::DateTime::UNIX_EPOCH.into()
                }
            }
        }
    }
}

#[handler]
pub fn update_draft(
    cookie_jar: &CookieJar,
    Json(form): Json<SubmitNewPostForm>,
    Data(conn): D<&W>,
) -> impl IntoResponse {
    // Auth check!
    let secret = read_secret();
    match cookie_jar.get("secret_pass") {
        Some(cookie) if cookie.value_str() == secret => {}
        _ => return (StatusCode::UNAUTHORIZED, "Unauthorized.").into_response(),
    }

    let conn = conn.lock().unwrap();

    let creation_datetime = parse_date(form.creation_datetime);

    let mut stmt = conn
        .prepare(
            "
            UPDATE draft
            SET
                title = ?1,
                contents = ?2,
                slug = ?3,
                subtitle = ?4,
                category = ?5,
                bsky_uri = ?6,
                creation_datetime = ?7
            ",
        )
        .unwrap();
    stmt.execute((
        form.title,
        form.contents,
        form.slug.clone(),
        form.subtitle,
        form.category,
        form.bsky_uri,
        creation_datetime,
    ))
    .expect("failed query");

    StatusCode::OK.into_response()
}

#[handler]
pub fn edit_post(
    Path(slug): Path<String>,
    cookie_jar: &CookieJar,
    Data(conn): D<&W>,
) -> impl IntoResponse {
    // Auth check!
    let secret = read_secret();
    match cookie_jar.get("secret_pass") {
        Some(cookie) if cookie.value_str() == secret => {}
        _ => return (StatusCode::UNAUTHORIZED, "Unauthorized.").into_response(),
    }

    let conn = conn.lock().unwrap();

    let mut stmt = conn.prepare("SELECT title, contents, slug, subtitle, category, bsky_uri, creation_datetime FROM post WHERE slug = ?1").unwrap();
    let post: Post = stmt
        .query_one([slug], |row| {
            Ok(Post {
                title: row.get(0).unwrap(),
                contents: row.get(1).unwrap(),
                slug: row.get(2).unwrap(),
                subtitle: row.get(3).unwrap(),
                category: row.get(4).unwrap(),
                bsky_uri: row.get(5).unwrap(),
                creation_datetime: row.get(6).unwrap(),
            })
        })
        .unwrap();

    html! {
        ( header_extra(html! {
            script defer src="/static/js/footnotes.js" {}
        }) )
        body {
            h1 { "Edit this post" }
            form method="POST" {
                label {
                    "Title: "
                    input name="title" value=(post.title) {}
                }
                label {
                    "Slug: "
                    input name="slug" value=(post.slug) {}
                }
                label {
                    "dtl: "
                    input name="creation_datetime" type="datetime-local" value=(post.creation_datetime.format(ISO8601_DATE)) {}
                }
                label {
                    "Subtitle: "
                    input name="subtitle" value=[post.subtitle] {}
                }
                label {
                    "Category: "
                    input name="category" value=[post.category] {}
                }
                label {
                    "bsky_uri:  "
                    input name="bsky_uri" value=[post.bsky_uri] {}
                    a href="https://pdsls.dev" {"pdsls"}
                }
                br;
                br;
                div #editorWrapper {
                    textarea #editor name="contents" { (post.contents) }
                    div #editorPreview { "hii :3" }
                }
                br;
                span { "Your edits are not saved automatically." }

                // Prevent implicit submission of the form (with enter)
                button type="submit" disabled style="display: none" aria-hidden="true";

                button type="submit" { "Update" }
            }

            script type="module" src="/static/js/post-edit.js" {}
            script type="module" src="/static/js/post-preview.js" {}
        }
    }.into_response()
}

fn clean(a: Option<String>) -> Option<String> {
    match a {
        Some(x) if x.is_empty() => None,
        x => x,
    }
}
#[handler]
pub fn submit_edited_post(
    Path(old_slug): Path<String>,
    cookie_jar: &CookieJar,
    Form(form): Form<SubmitNewPostForm>,
    Data(conn): D<&W>,
) -> impl IntoResponse {
    // Auth check!
    let secret = read_secret();
    match cookie_jar.get("secret_pass") {
        Some(cookie) if cookie.value_str() == secret => {}
        _ => return (StatusCode::UNAUTHORIZED, "Unauthorized.").into_response(),
    }

    let conn = conn.lock().unwrap();

    let creation_datetime = parse_date(form.creation_datetime);

    let mut stmt = conn
        .prepare(
            "
            UPDATE post
            SET
                title = ?1,
                contents = ?2,
                slug = ?3,
                subtitle = ?4,
                category = ?5,
                bsky_uri = ?6,
                creation_datetime = ?7
            WHERE
                slug = ?8",
        )
        .unwrap();
    stmt.execute((
        form.title,
        form.contents,
        form.slug.clone(),
        clean(form.subtitle),
        clean(form.category),
        clean(form.bsky_uri),
        creation_datetime,
        old_slug,
    ))
    .expect("failed query");

    Redirect::see_other(format!("/blog/{}", form.slug)).into_response()
}
