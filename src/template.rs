use maud::{DOCTYPE, Markup, PreEscaped, html};

pub fn header() -> Markup {
    html! {
        (DOCTYPE)
        head {
            meta charset="utf-8";
            meta name="viewport" content="width=device-width";

            link rel="stylesheet" type="text/css" href="/static/css/normalize.css";
            link rel="stylesheet" type="text/css" href="/static/css/style.css";
            link rel="stylesheet" type="text/css" href="/static/css/nav.css";
            link rel="stylesheet" type="text/css" href="/static/css/dialog.css";

            meta name="theme-color" content="#b497ee";
            meta name="apple-mobile-web-app-status-bar-style" content="#b497ee";
        }
    }
}

pub fn navbar(current_endpoint: &str) -> Markup {
    let items = vec![
        ("index", "/", "j0.lol"),
        ("projects", "/projects", "Projects"),
        ("blog-index", "/blog", "Blog"),
        ("contact", "/contact", "Contact"),
    ];

    html! {
        nav.bar {
            ul {
                @for (name, endpoint, label) in items {
                    @let is_index = name == "index";
                    @let active = endpoint == current_endpoint;

                    li {
                        a #{ "nav-" (name) } href=(endpoint) .active[active] { (label) }
                    }
                    @if is_index {
                        li .navbreak-show style="flex: 1 0 100%;";
                    }
                }
            }
        }
    }
}

pub fn page(markup: Markup, endpoint: &str) -> Markup {
    html! {
        ( header() )
        div.wrapper {
            (navbar(endpoint))
            main {
                (markup)
            }
        }
    }
}

pub enum SpeechEmotion {
    Neutral,
    Worried,
    Shocked,
    Happy,
}

pub enum SpeechCharacter {
    Deer,
    You,
}

pub fn speech(char: SpeechCharacter, emotion: SpeechEmotion, content: Markup) -> Markup {
    let src = match char {
        SpeechCharacter::Deer => match emotion {
            SpeechEmotion::Neutral => "/static/speech/deer/neutral.png",
            SpeechEmotion::Worried => "/static/speech/deer/sad.png",
            SpeechEmotion::Happy => "/static/speech/deer/happy.png",
            SpeechEmotion::Shocked => "/static/speech/deer/shock.png",
        },
        SpeechCharacter::You => "/static/speech/you.png",
    };
    let alt = match char {
        SpeechCharacter::Deer => match emotion {
            SpeechEmotion::Neutral => "drawing of a deer, talking to you.",
            SpeechEmotion::Worried => "drawing of a sad or worried deer, talking to you.",
            SpeechEmotion::Happy => "drawing of a happy deer, talking to you.",
            SpeechEmotion::Shocked => "drawing of a shocked deer, talking to you.",
        },
        SpeechCharacter::You => "drawing of you, smiling.",
    };
    let class = match char {
        SpeechCharacter::Deer => "deer",
        SpeechCharacter::You => "you",
    };

    html! {
        div.dialog-box {
            img.raw.dialog.profile width="120" height="120" src=(src) alt=(alt);
            div.dialog.speech.{(class)} {
                (content)
            }
        }
    }
}
