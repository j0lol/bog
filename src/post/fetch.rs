use super::Post;
use crate::{
    W,
    error::{AppError, Result},
};

pub enum PostSource {
    Draft,
    Post(String),
}

pub fn all(conn: &W) -> Result<Vec<Post>> {
    let conn = conn.lock()?;

    let mut stmt = conn.prepare("SELECT title, contents, slug, subtitle, category, bsky_uri, creation_datetime FROM post")
        .map_err(AppError::from)?;

    let posts_result: rusqlite::Result<Vec<Post>> = stmt
        .query_map([], |row| {
            Ok(Post {
                title: row.get(0)?,
                contents: row.get(1)?,
                slug: row.get(2)?,
                subtitle: row.get(3)?,
                category: row.get(4)?,
                bsky_uri: row.get(5)?,
                creation_datetime: row.get(6)?,
            })
        })?
        .collect();

    let mut posts = posts_result.map_err(AppError::from)?;
    posts.sort_by(|a, b| {
        a.creation_datetime
            .partial_cmp(&b.creation_datetime)
            .unwrap_or(std::cmp::Ordering::Equal)
            .reverse()
    });
    Ok(posts)
}

pub fn one(slug: String, conn: &W) -> Result<Post> {
    fetch_post_inner(&PostSource::Post(slug), conn)
}

pub fn one_draft(conn: &W) -> Result<Post> {
    fetch_post_inner(&PostSource::Draft, conn)
}

fn fetch_post_inner(source: &PostSource, conn: &W) -> Result<Post> {
    let conn = conn.lock()?;

    let (query, params): (&str, Vec<&dyn rusqlite::ToSql>) = match source {
        PostSource::Post(slug) => (
            "SELECT title, contents, slug, subtitle, category, bsky_uri, creation_datetime FROM post WHERE slug = ?1",
            vec![slug],
        ),
        PostSource::Draft => (
            "SELECT title, contents, slug, subtitle, category, bsky_uri, creation_datetime FROM draft",
            vec![],
        ),
    };

    let mut stmt = conn.prepare(query).map_err(AppError::from)?;

    let post = stmt
        .query_one(params.as_slice(), |row| {
            Ok(Post {
                title: row.get(0)?,
                contents: row.get(1)?,
                slug: row.get(2)?,
                subtitle: row.get(3)?,
                category: row.get(4)?,
                bsky_uri: row.get(5)?,
                creation_datetime: row.get(6)?,
            })
        })
        .map_err(AppError::from)?;

    Ok(post)
}
