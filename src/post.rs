use crate::{
    D, W,
    template::{header, navbar, page},
};
use chrono::{DateTime, Local};
use maud::{Markup, PreEscaped, html};
use poem::{
    IntoResponse, handler,
    web::{Data, Form, Json, Path, Redirect},
};
use serde::Deserialize;

const ISO8601_DATE: &'static str = "%Y-%m-%dT%H:%M";

fn clock_icon() -> Markup {
    html! {
        ( "🕒" )
    }
}

#[derive(Deserialize)]
struct Post {
    title: String,
    contents: String,
    slug: String,
    subtitle: Option<String>,
    category: Option<String>,
    bsky_uri: Option<String>,
    creation_datetime: DateTime<Local>,
}

#[handler]
pub fn list_posts(Data(conn): D<&W>) -> Markup {
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
        .unwrap();

    page(
        html! {
            h1 { "Post list" }
            ol {
                @for post in post_iter.flatten() {
                li  {
                    a href={"/blog/" (post.slug) } { (post.title) }
                    br;
                    span {
                        (clock_icon())
                    }
                    time datetime=(post.creation_datetime) { (post.creation_datetime) }
                }
                }
            }
        },
        "/blog",
    )
}

#[handler]
pub fn view_post(Path(slug): Path<String>, Data(conn): D<&W>) -> Markup {
    let conn = conn.lock().unwrap();

    let mut stmt = conn.prepare("SELECT title, contents, slug, subtitle, category, bsky_uri, creation_datetime FROM post WHERE slug = ?1").unwrap();
    let post = stmt
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
        ( header() )
        div.wrapper {
            (navbar(&format!("/blog/{}", post.slug)))
            article {
                h1.blog-head { (post.title)}
                span.blog-subhead { em { "subtitle" }}
                hr.frontmatter;
                p.blog-publish {
                    ( clock_icon() )
                    time datetime=(post.creation_datetime) { (post.creation_datetime) }
                }

                (PreEscaped(post.contents))
            }
        }
    }
}

#[handler]
pub fn new_post(Data(conn): D<&W>) -> Markup {
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
        ( header() )
        body {
            h1 { "Make a new post" }
            form method="POST" {
                label {
                    "Title"
                    input name="title" value=(post.title) {}
                }
                br;
                label {
                    "Slug"
                    input name="slug" value=(post.slug) {}
                }
                br;
                label {
                    "dtl"
                    input name="creation_datetime" type="datetime-local" value=(post.creation_datetime.format(ISO8601_DATE)) {}
                }
                br;
                div style="display: flex; flex-direction: row; gap: 0.5rem; height: 100%; width: 100%; " {

                    textarea #editor name="contents" style="width: 100%; height: 50ch" { (post.contents) }
                    div #editorPreview style="border: 1px solid red; padding: 0 0.5rem; background-color: var(--bg-surface1); width: 100%" { "hii :3" }
                }

                br;
                span { "Your draft is auto saved..." }
                button type="submit" { "Publish" }
            }

            script type="module" src="/static/js/post-new.js" {}
        }
    }
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
    Form(form): Form<SubmitNewPostForm>,
    Data(conn): D<&W>,
) -> impl IntoResponse {
    let conn = conn.lock().unwrap();

    let creation_datetime =
        chrono::NaiveDateTime::parse_from_str(&form.creation_datetime, "%Y-%m-%dT%H:%M")
            .expect("bad datetime");

    let mut stmt = conn
        .prepare("INSERT INTO post (title, contents, slug, subtitle, category, bsky_uri, creation_datetime) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)")
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

    Redirect::see_other(format!("/blog/{}", form.slug))
}

#[handler]
pub fn update_draft(Json(form): Json<SubmitNewPostForm>, Data(conn): D<&W>) {
    let conn = conn.lock().unwrap();

    let creation_datetime =
        chrono::NaiveDateTime::parse_from_str(&form.creation_datetime, "%Y-%m-%dT%H:%M")
            .expect("bad datetime");

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
}
