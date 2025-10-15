use maud::{DOCTYPE, Markup, PreEscaped, html};

pub fn header() -> Markup {
    header_extra(html! {})
}

pub fn header_extra(markup: Markup) -> Markup {
    html! {
        (DOCTYPE)
        head {
            meta charset="utf-8";
            meta name="viewport" content="width=device-width";

            link rel="stylesheet" type="text/css" href="/static/css/normalize.css";
            link rel="stylesheet" type="text/css" href="/static/css/style.css";
            link rel="stylesheet" type="text/css" href="/static/css/nav.css";
            link rel="stylesheet" type="text/css" href="/static/css/dialog.css";

            script type="module" src="/static/js/login.js" defer {}

            meta name="theme-color" content="#b497ee";
            meta name="apple-mobile-web-app-status-bar-style" content="#b497ee";

            script src="/static/js/prism.js" defer {}
            link rel="stylesheet" type="text/css" href="/static/css/prism-theme-mocha.css" defer;

            link rel="icon" href="/static/favicon.ico" sizes="any";
            link rel="apple-touch-icon" href="/static/j0site-pfp.png";
            meta property="og:image" content="/static/j0site-banner.png";

            (markup)
        }
    }
}

pub fn footer() -> Markup {
    html! {
        footer #page-footer {
            div {
                span {
                    "This website is running on "
                    a href="https://tangled.org/@j0.lol/bog" style="color: var(--fg-header)"
                    { samp { (env!("CARGO_CRATE_NAME")) } }
                    " "
                    a href={"https://tangled.org/@j0.lol/bog/commit/" (env!("VERGEN_GIT_SHA"))} style="color: var(--fg-header); font-size: 0.8rem;"
                    { samp { (env!("VERGEN_GIT_SHA")[..8]) } " (" (env!("VERGEN_GIT_COMMIT_DATE")) ")" }
                    "."
                }
                br;
                button .subtle.clickme #login-button { "auth" }
                span .subtle {" • "}
                a .subtle.clickme href="/blog/new" { "new" }
                span .subtle {" • "}
                button .subtle.clickme #edit-button { "edit" }
            }

            (PreEscaped("
            <div class=\"_88x31s\">
                <span class=\"sr-only\">Miscellaneous links:</span>
                <a href=\"/\">
                    <img class=\"raw\" width=88 height=31 src=\"/static/badges/j0.gif\"
                         alt=\"Logo: j0, with subtitle 'deer thing'. To the side, there is a purple deer with yellow features. Various elements flicker.\">
                </a>

                <a href=\"https://maud.lambda.xyz\">
                    <img class=\"raw\" width=88 height=31 src=\"/static/badges/maud.png\" alt=\"Powered by Maud\">
                </a>
                <a href=\"https://brainmade.org\">
                    <div style=\"display: flex; padding: 3px; background-color: #000\">
                        <img class=\"raw\" alt=\"The Brainmade Mark\" src=\"/static/badges/brainmade.svg\" width=82 height=25>
                    </div>
                </a>
            </div>
            "))
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
            ( footer() )
        }
    }
}

pub fn page_article(markup: Markup, endpoint: &str) -> Markup {
    html! {
        ( header() )
        div.wrapper {
            (navbar(endpoint))
            article {
                (markup)
            }
            ( footer() )
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

pub struct SpeechDetails {
    pub class: String,
    pub alt: String,
    pub src: String,
}
pub fn render_speech(char: SpeechCharacter, emotion: SpeechEmotion) -> SpeechDetails {
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

    SpeechDetails {
        class: class.to_owned(),
        alt: alt.to_owned(),
        src: src.to_owned(),
    }
}

pub fn speech(char: SpeechCharacter, emotion: SpeechEmotion, content: Markup) -> Markup {
    let SpeechDetails { class, alt, src } = render_speech(char, emotion);
    html! {
        div.dialog-box {
            img.raw.dialog.profile width="120" height="120" src=(src) alt=(alt);
            div.dialog.speech.{(class)} {
                (content)
            }
        }
    }
}
