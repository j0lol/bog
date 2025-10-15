mod db;
mod post;
pub mod template;

use crate::{
    post::{list_posts, new_post, submit_new_post, update_draft, view_post},
    template::{SpeechCharacter, SpeechEmotion, footer, header, navbar, speech},
};
use maud::{Markup, PreEscaped, html};
use poem::{
    EndpointExt, Route, Server, endpoint::StaticFilesEndpoint, get, handler, listener::TcpListener,
    web::Data,
};
use rand::seq::SliceRandom;
use rusqlite::Connection;
use std::{
    fs::read_to_string,
    sync::{Arc, Mutex},
};

type WrappedConnection = Arc<Mutex<Connection>>;
type W = WrappedConnection;
type D<T> = Data<T>;

#[handler]
fn index() -> Markup {
    let mut rng = rand::rng();

    let mut pronouns = ["she/her", "they/them", "it/its"];
    let mut genders = ["𐂂", "\u{2400}", "\u{2205}", "girl?", "stolen", "not"];

    pronouns.shuffle(&mut rng);
    genders.shuffle(&mut rng);

    let select_1 = html! {
        select { @for thing in pronouns {
                option { (thing) }
        } }
    };
    let select_2 = html! {
        select { @for thing in genders {
                option { (thing) }
        } }
    };

    html! {
        (header() )
        div.wrapper {
            ( navbar("/") )
            main {
                h1.fancy.page-head { "Hi!" }
                ( speech( SpeechCharacter::Deer, SpeechEmotion::Neutral, html! {
                    p {
                        "I'm Jo. "
                        label #my-pronouns { "My pronouns are " (select_1) }
                        " and "
                        label { "my gender is" (select_2) }
                    }

                    p {
                        "I'm a CompSci graduate from the University of Sussex. "
                        small { a href="/contact" { "(Hire me!)"} }
                    }
                }))

                details style="margin-top: 1rem;" {
                    summary { "What's your pronouns?" }

                    label #pronouns-blurb {
                        input #pronouns-choice;
                    }
                    button #pronouns-submit type="button" { "Submit" }
                    // Ugly please kill
                    script { ( PreEscaped("
                        function steal_pronouns() {
                            let input_field = document.querySelector(\"#pronouns-choice\");
                            let pronouns = input_field.value;
                            input_field.value = \"\";

                            document.querySelector(\"#pronouns-blurb\").innerHTML = \"haha! mine now!\";
                            document.querySelector(\"#my-pronouns\").innerHTML = \"My pronouns are <b>\" + pronouns + \"</b>\";
                            document.querySelector(\"#pronouns-submit\").remove();
                        }

                        document.querySelector(\"#pronouns-submit\").addEventListener(\"click\", steal_pronouns);
                        "))

                    }
                }

                h2 { "What do you do?" }
                p { "I mainly write software, and study in the art of writing software.
                    My specialities lie in writing correct, robust code.
                    I love to read and write documentation, and to double-check my work.
                    Outside of development, I like to draw and write,
                    and cook meals with a good splash of umami.
                    I am conversational in a constructed language, toki pona.
                    " }
                p { "Here's what I'm interested in right now!" }

                ul style="margin-top: -0.5rem;" {
                    li { "Rust" }
                    li { "Making small stuff with PHP" }
                    li { "WebGPU for fast, write-once-run-anywhere rendering" }
                    li { "Small websites, small communities" }
                    li { "Idiomatic vanilla Javascript, modern CSS" }
                    li { "Clean design" }
                }

                h2 { "More info" }
                ul {
                    li {
                        "If you want to see what I've done before, see "
                        a href="/projects" { "my projects"} "!"
                    }
                    li {
                        "If you want to see how I did my projects, see "
                        a href="/blog" { "my blog"} "!"
                    }
                }

                p { "I'm always working on something! Feel free to "
                    a href="https://github.com/j0lol" { "check out my git repos" }
                    " or just "
                    a href="/contact" { "say hi" }
                    " :)" }
            }

            div.tablet-show style="background: var(--bg-surface1); padding: 0 1rem;" {
                hr.frontmatter.pilcrow;
            }
        }

        div.page {
            ( speech( SpeechCharacter::Deer, SpeechEmotion::Happy, html! {
                p {
                    (PreEscaped("Thanks for visiting my website! Here's some friend's sites.
                    These are the 88&times;31 buttons that my friends made to link to their websites.
                    Click them to go to them!"))
                }
            }))

            details {
                summary { "Embed code for my website" }

                pre {
                    code.language-html {
                        (PreEscaped("&#x3C;a rel=&#x22;noreferrer&#x22; href=&#x22;https://j0.lol&#x22;&#x3E;
    &#x3C;img src=&#x22;https://j0.lol/static/badges/j0.gif&#x22; alt=&#x22;Logo: j0, with subtitle &#x27;deer thing&#x27;. To the side, there is a purple deer with yellow features. Various elements flicker.&#x22;&#x3E;
&#x3C;/a&#x3E;"))
                    }
                }
            }

            @let btns = vec![
                ("meihapps.gif", "Logo: mei happs. Monospaced text. Pink border.", "https://meihapps.gay", ),
                ("finn.gif", "Logo: cr0wbar. Handwritten text. The 0 has what appears to be a shiny gem inside of it.", "https://cr0wbar.dev", ),
                ("cadence_now.png", "Logo: cadence Now!. The first part of the text is in an 'outrun' vaporwave-type style. The final part is handwritten. There is what appears to be a purple eye with a yellow iris to the left of the text. Enclosing the eye is a white letter c. There is a star banner on the bottom right.", "https://cadence.moe", ),
                ("barrow.png", "Green text says 'Join Barr0wnet'. Enclosing the text is a scientific diagram of the chemical alpha-methyl.", "http://alphamethyl.barr0w.net", ),
                ("swiftys.gif", "Swifty's HQ!", "https://swiftyshq.neocities.org", ),
                ("kett.png", "racc.at 88x31", "https://racc.at/", ),
                ("mae.png", "mae", "https://mae.wtf/", ),
            ];

            @for (file, alt, link) in btns {
                a href=(link) {
                    img.raw src={"/static/badges/" (file)} alt=(alt);
                }
            }
        }
        (footer() )
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let conn = Arc::new(Mutex::new(db::connect()));

    let app = Route::new()
        .at("/", get(index))
        .at("/blog", get(list_posts))
        .at("/blog/new", get(new_post).post(submit_new_post))
        .at("/blog/new/sync", poem::post(update_draft))
        .at("/blog/:slug", get(view_post))
        .nest("/static", StaticFilesEndpoint::new("./static/"))
        .data(conn.clone());

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .name("hello-world")
        .run(app)
        .await
}
