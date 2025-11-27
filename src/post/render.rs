#![allow(clippy::needless_pass_by_value)]

use crate::template::{SpeechDetails, render_speech, speech};
use crate::{
    error::{AppError, Result},
    template::{SpeechCharacter, SpeechEmotion},
};
use autumnus::{formatter::Formatter, languages::Language};
use makup::Rewriter;
use maud::{PreEscaped, html};
use std::collections::HashMap;

fn speech_box(contents: &str, attrs: HashMap<&str, &str>) -> String {
    let char = attrs.get("character").unwrap_or(&"deer");
    let emotion = attrs.get("emotion").unwrap_or(&"neutral");

    speech(
        &match *char {
            "you" => SpeechCharacter::You,
            _ => SpeechCharacter::Deer,
        },
        &match *emotion {
            "worried" => SpeechEmotion::Worried,
            "shocked" => SpeechEmotion::Shocked,
            "happy" => SpeechEmotion::Happy,
            _ => SpeechEmotion::Neutral,
        },
        &html! { (PreEscaped(contents)) },
    )
    .into_string()
}

fn speech_box_nocss(contents: &str, attrs: HashMap<&str, &str>) -> String {
    let char = attrs.get("character").unwrap_or(&"deer");
    let emotion = attrs.get("emotion").unwrap_or(&"neutral");

    let SpeechDetails { class: _, alt, src } = render_speech(
        &match *char {
            "you" => SpeechCharacter::You,
            _ => SpeechCharacter::Deer,
        },
        &match *emotion {
            "worried" => SpeechEmotion::Worried,
            "shocked" => SpeechEmotion::Shocked,
            "happy" => SpeechEmotion::Happy,
            _ => SpeechEmotion::Neutral,
        },
    );

    html! {
        table {
            tbody {
                td {
                    img width="120" height="120" src=(src) alt=(alt);
                }
                td {
                    span { (PreEscaped(contents))}
                }
            }
        }
    }
    .into_string()
}
// Safety: function is not allowed to error. Annoyingly.
#[allow(clippy::expect_used, clippy::unwrap_used)]
fn code_block(contents: &str, attrs: HashMap<&str, &str>) -> String {
    let lang = attrs
        .get("lang")
        .or_else(|| attrs.get("language"))
        .unwrap_or(&"plain");

    let formatter = autumnus::HtmlInlineBuilder::new()
        .lang(Language::guess(lang, contents))
        .source(contents)
        .theme(Some(
            autumnus::themes::get("catppuccin_mocha").expect("Built in!"),
        ))
        .build()
        .expect("i hope this doesnt crash!");

    let mut output = Vec::new();
    formatter.format(&mut output).unwrap();

    let output = String::from_utf8(output).unwrap();
    html! {
        .code-block {
            .code-language { (lang) }
            (PreEscaped(output))
        }
    }
    .into_string()
}

fn code_block_nocss(contents: &str, attrs: HashMap<&str, &str>) -> String {
    let lang = attrs
        .get("lang")
        .or_else(|| attrs.get("language"))
        .unwrap_or(&"plain");

    html! {
        figure {
            pre {
                code {
                    (contents)
                }
            }
            figcaption {
                "Code block in " (lang) " language"
            }
        }
    }
    .into_string()
}

pub fn render_post(markup: &str) -> Result<String> {
    let mut rewriter = Rewriter::default();
    rewriter.add_component("speech-box", speech_box);
    rewriter.add_component("code-block", code_block);

    let contents = rewriter
        .rewrite(markup)
        .map_err(|e| AppError::internal_server_error(e.to_string()))?;

    Ok(contents)
}

pub fn render_post_nocss(markup: &str) -> Result<String> {
    let mut rewriter = Rewriter::default();
    rewriter.add_component("speech-box", speech_box_nocss);
    rewriter.add_component("code-block", code_block_nocss);

    let contents = rewriter
        .rewrite(markup)
        .map_err(|e| AppError::internal_server_error(e.to_string()))?;

    Ok(contents)
}
