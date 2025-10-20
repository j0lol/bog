use crate::{
    D, W,
    post::fetch_all_posts,
    template::{SpeechCharacter, SpeechDetails, SpeechEmotion, render_speech},
};
use atom_syndication::{Content, Entry, EntryBuilder, FeedBuilder, Link, Person};
use chrono::Utc;
use lol_html::{
    RewriteStrSettings, comments, element, html_content::ContentType, rewrite_str, text,
};
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
            let contents = rewrite_str(
                &post.contents,
                RewriteStrSettings {
                    element_content_handlers: vec![
                        element!("speech-box", |el| {
                            let char = el.get_attribute("character").unwrap_or("deer".to_string());
                            let emotion =
                                el.get_attribute("emotion").unwrap_or("neutral".to_string());

                            let SpeechDetails { class: _, alt, src } = render_speech(
                                match char.as_ref() {
                                    "you" => SpeechCharacter::You,
                                    "deer" => SpeechCharacter::Deer,
                                    _ => SpeechCharacter::Deer,
                                },
                                match emotion.as_ref() {
                                    "worried" => SpeechEmotion::Worried,
                                    "shocked" => SpeechEmotion::Shocked,
                                    "happy" => SpeechEmotion::Happy,
                                    "neutral" => SpeechEmotion::Neutral,
                                    _ => SpeechEmotion::Neutral,
                                },
                            );

                            el.set_tag_name("div")?;
                            el.before(
                                &format!(
                                    r#"
                        <table>
                        <tbody>
                            <td>
                                <img width="120" height="120" src="{src}" alt="{alt}">
                            </td>
                            <td>
                        "#
                                ),
                                ContentType::Html,
                            );
                            el.after("</td></tbody></table>", ContentType::Html);

                            Ok(())
                        }),
                        comments!("pre > code", |c| {
                            // for prism.js html-in-comments
                            c.replace(&c.text().trim(), ContentType::Text);

                            Ok(())
                        }),
                    ],
                    ..RewriteStrSettings::new()
                },
            )
            .unwrap();

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
                    value: Some(contents),
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
