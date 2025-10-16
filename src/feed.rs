use crate::{D, W, post::fetch_all_posts};
use atom_syndication::{Content, Entry, EntryBuilder, FeedBuilder, Link, Person};
use chrono::Utc;
use poem::{IntoResponse, handler, web::Data};

const HOST: &str = "https://j0.lol";

fn entries(conn: &W) -> Vec<Entry> {
    let posts = fetch_all_posts(conn);

    let entries: Vec<_> = posts
        .iter()
        .map(|post| {
            let mut entry = EntryBuilder::default();
            let author: Person = Person {
                name: "Jo Null".to_owned(),
                ..Default::default()
            };

            entry
                .link(Link {
                    href: format!("{HOST}/blog/{}", post.slug),
                    rel: "alternate".to_owned(),
                    ..Link::default()
                })
                .title(post.title.clone())
                .published(post.creation_datetime.fixed_offset())
                .author(author)
                .content(Content {
                    base: Some(HOST.to_owned()),
                    src: Some(post.contents.clone()),
                    content_type: Some("html".to_owned()),
                    ..Content::default()
                });
            entry.build()
        })
        .collect();

    entries
}

#[handler]
pub fn feed(Data(conn): D<&W>) -> impl IntoResponse {
    let feed = FeedBuilder::default()
        .title("Jo's blog")
        .links(vec![
            Link {
                href: format!("{HOST}/blog"),
                rel: "alternate".to_string(),
                ..Default::default()
            },
            Link {
                href: format!("{HOST}/feed"),
                rel: "self".to_string(),
                ..Default::default()
            },
        ])
        .id(format!("{HOST}/blog"))
        .author(Person {
            name: "Jo Null".to_owned(),
            ..Default::default()
        })
        .base(HOST.to_string())
        .icon(format!("{HOST}/static/favicon.ico"))
        .logo(format!("{HOST}/static/j0site-banner.png"))
        .updated(Utc::now())
        .entries(entries(conn))
        .build();

    feed.to_string().with_content_type("application/atom+xml")
}
