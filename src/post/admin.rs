use super::{
    ISO8601_DATE, Post, clean_empty_string,
    fetch::{fetch_draft, fetch_post},
    parse_date, read_secret,
};
use crate::{
    D, W,
    error::{AppError, Result},
    template::header_extra,
};
use chrono::DateTime;
use maud::{Markup, PreEscaped, html};
use poem::{
    IntoResponse, Response, handler,
    web::{Data, Form, Json, Redirect, cookie::CookieJar},
};
use serde::Deserialize;

struct PostData {
    title: String,
    contents: String,
    slug: String,
    subtitle: Option<String>,
    category: Option<String>,
    bsky_uri: Option<String>,
    creation_datetime: DateTime<chrono::Local>,
}

#[derive(Copy, Clone)]
enum PostOperation {
    Insert,
    Update,
    UpdateDraft,
}

#[derive(Deserialize)]
pub struct SubmitNewPostForm {
    pub title: String,
    pub contents: String,
    pub slug: String,
    pub subtitle: Option<String>,
    pub category: Option<String>,
    pub bsky_uri: Option<String>,
    pub creation_datetime: String,
}

#[derive(Deserialize)]
pub struct SubmitEditedPostForm {
    pub title: String,
    pub contents: String,
    pub slug: String,
    pub subtitle: Option<String>,
    pub category: Option<String>,
    pub bsky_uri: Option<String>,
    pub creation_datetime: String,
}

fn check_auth(cookie_jar: &CookieJar) -> Result<()> {
    let secret = read_secret();
    match cookie_jar.get("secret_pass") {
        Some(cookie) if cookie.value_str() == secret => Ok(()),
        _ => Err(AppError::Unauthorized),
    }
}

#[handler]
pub fn login(body: String, cookie_jar: &CookieJar) -> impl IntoResponse {
    cookie_jar.add(poem::web::cookie::Cookie::new_with_str("secret_pass", body));
    #[allow(clippy::expect_used)]
    let secret_value = cookie_jar
        .get("secret_pass")
        .expect("Cookie should exist after being set");
    html! {(secret_value)}.into_response()
}

#[handler]
pub fn new_post(cookie_jar: &CookieJar, Data(conn): D<&W>) -> impl IntoResponse {
    match new_post_inner(cookie_jar, Data(conn)) {
        Ok(response) => response,
        Err(e) => e.into_response(),
    }
}

fn new_post_inner(cookie_jar: &CookieJar, Data(conn): D<&W>) -> Result<Response> {
    check_auth(cookie_jar)?;

    let post = fetch_draft(conn)
        .map_err(|_| AppError::internal_server_error("Could not fetch draft".to_string()))?;

    let response = render_post_form(&post, "Make a new post", "POST", "/blog/new", "new");
    Ok(response.into_response())
}

#[handler]
pub fn edit_post(
    cookie_jar: &CookieJar,
    poem::web::Path(slug): poem::web::Path<String>,
    Data(conn): D<&W>,
) -> impl IntoResponse {
    match edit_post_inner(cookie_jar, &slug, Data(conn)) {
        Ok(response) => response,
        Err(e) => e.into_response(),
    }
}

fn edit_post_inner(cookie_jar: &CookieJar, slug: &str, Data(conn): D<&W>) -> Result<Response> {
    check_auth(cookie_jar)?;

    let post = fetch_post(slug.to_string(), conn)?;
    let response = render_post_form(
        &post,
        "Edit post",
        "POST",
        &format!("/blog/edit/{slug}"),
        "edit",
    );
    Ok(response.into_response())
}

fn render_post_form(post: &Post, title: &str, method: &str, action: &str, js_mode: &str) -> Markup {
    html! {
        ( header_extra(&html! {
            script defer src="/static/js/footnotes.js" {}
        }) )
        body {
            h1 { (title) }
            form method=(method) action=(action) {
                label {
                    "Title: "
                    input name="title" value=(post.title.clone()) {}
                }
                label {
                    "Slug: "
                    input name="slug" value=(post.slug.clone()) {}
                }
                label {
                    "dtl: "
                    input name="creation_datetime" type="datetime-local" value=(post.creation_datetime.format(ISO8601_DATE)) {}
                }
                label {
                    "Subtitle: "
                    input name="subtitle" value=[post.subtitle.clone()] {}
                }
                label {
                    "Category: "
                    input name="category" value=[post.category.clone()] {}
                }
                label {
                    "bsky_uri:  "
                    input name="bsky_uri" value=[post.bsky_uri.clone()] {}
                    a href="https://pdsls.dev" {"pdsls"}
                }
                br;
                br;
                div #editorWrapper {
                    textarea #editor name="contents" { (post.contents.clone()) }
                    div #editorPreview { "hii :3" }
                }
                br;
                span { "Your draft is auto saved..." }

                button type="submit" disabled style="display: none" aria-hidden="true";
                button type="submit" { "Publish" }
            }

            script type="module" {
                (PreEscaped(format!(r#"
                    import {{ initPostEditor }} from "/static/js/post-editor.js";
                    initPostEditor("{js_mode}");
                "#)))
            }
        }
    }
}

#[handler]
pub fn submit_new_post(
    cookie_jar: &CookieJar,
    Form(form): Form<SubmitNewPostForm>,
    Data(conn): D<&W>,
) -> impl IntoResponse {
    match submit_new_post_inner(cookie_jar, form, Data(conn)) {
        Ok(response) => response,
        Err(e) => e.into_response(),
    }
}

fn submit_new_post_inner(
    cookie_jar: &CookieJar,
    form: SubmitNewPostForm,
    Data(conn): D<&W>,
) -> Result<Response> {
    check_auth(cookie_jar)?;

    let creation_datetime = parse_date(&form.creation_datetime).with_timezone(&chrono::Local);

    let post_data = PostData {
        title: form.title,
        contents: form.contents,
        slug: form.slug.clone(),
        subtitle: form.subtitle,
        category: form.category,
        bsky_uri: form.bsky_uri,
        creation_datetime,
    };

    save_post(conn, &post_data, PostOperation::Insert)?;

    Ok(Redirect::see_other(format!("/blog/{}", form.slug)).into_response())
}

#[handler]
pub fn submit_edited_post(
    cookie_jar: &CookieJar,
    Form(form): Form<SubmitEditedPostForm>,
    Data(conn): D<&W>,
) -> impl IntoResponse {
    match submit_edited_post_inner(cookie_jar, form, Data(conn)) {
        Ok(response) => response,
        Err(e) => e.into_response(),
    }
}

fn submit_edited_post_inner(
    cookie_jar: &CookieJar,
    form: SubmitEditedPostForm,
    Data(conn): D<&W>,
) -> Result<Response> {
    check_auth(cookie_jar)?;

    let creation_datetime = parse_date(&form.creation_datetime).with_timezone(&chrono::Local);

    let post_data = PostData {
        title: form.title,
        contents: form.contents,
        slug: form.slug.clone(),
        subtitle: form.subtitle,
        category: form.category,
        bsky_uri: form.bsky_uri,
        creation_datetime,
    };

    save_post(conn, &post_data, PostOperation::Update)?;

    Ok(Redirect::see_other(format!("/blog/{}", form.slug)).into_response())
}

#[handler]
pub fn update_draft(
    cookie_jar: &CookieJar,
    Json(form): Json<SubmitNewPostForm>,
    Data(conn): D<&W>,
) -> impl IntoResponse {
    match update_draft_inner(cookie_jar, form, Data(conn)) {
        Ok(response) => response,
        Err(e) => e.into_response(),
    }
}

fn update_draft_inner(
    cookie_jar: &CookieJar,
    form: SubmitNewPostForm,
    Data(conn): D<&W>,
) -> Result<Response> {
    check_auth(cookie_jar)?;

    let creation_datetime = parse_date(&form.creation_datetime).with_timezone(&chrono::Local);

    let post_data = PostData {
        title: form.title,
        contents: form.contents,
        slug: form.slug,
        subtitle: form.subtitle,
        category: form.category,
        bsky_uri: form.bsky_uri,
        creation_datetime,
    };

    save_post(conn, &post_data, PostOperation::UpdateDraft)?;

    Ok("Draft saved".into_response())
}

fn save_post(conn: &W, post_data: &PostData, operation: PostOperation) -> Result<()> {
    let conn = conn.lock()?;

    let query = match operation {
        PostOperation::Insert => {
            "INSERT INTO post (title, contents, slug, subtitle, category, bsky_uri, creation_datetime) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)"
        }
        PostOperation::Update => {
            "UPDATE post SET title = ?1, contents = ?2, slug = ?3, subtitle = ?4, category = ?5, bsky_uri = ?6, creation_datetime = ?7 WHERE slug = ?3"
        }
        PostOperation::UpdateDraft => {
            "UPDATE draft SET title = ?1, contents = ?2, slug = ?3, subtitle = ?4, category = ?5, bsky_uri = ?6, creation_datetime = ?7"
        }
    };

    let mut stmt = conn.prepare(query).map_err(AppError::from)?;

    stmt.execute((
        &post_data.title,
        &post_data.contents,
        &post_data.slug,
        clean_empty_string(post_data.subtitle.clone()),
        clean_empty_string(post_data.category.clone()),
        clean_empty_string(post_data.bsky_uri.clone()),
        post_data.creation_datetime,
    ))
    .map_err(AppError::from)?;

    Ok(())
}
