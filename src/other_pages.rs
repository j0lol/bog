use crate::template::{SpeechCharacter, SpeechEmotion, footer, header, navbar, page, speech};
use maud::{Markup, PreEscaped, html};
use poem::handler;
use rand::seq::SliceRandom;

#[handler]
pub fn index() -> Markup {
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
                ( speech( &SpeechCharacter::Deer, &SpeechEmotion::Neutral, &html! {
                    p {
                        "I'm Jo. "
                        label #my-pronouns { "My pronouns are " (select_1) }
                        " and "
                        label { "my gender is " (select_2) }
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

            div.page {
                ( speech( &SpeechCharacter::Deer, &SpeechEmotion::Happy, &html! {
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

                ._88x31s {
                    @for (file, alt, link) in btns {
                        a href=(link) {
                            img.raw src={"/static/badges/" (file)} alt=(alt);
                        }
                    }
                }
            }
        }

        (footer() )
    }
}

#[handler]
pub fn contact() -> Markup {
    page(
        &html! {
            ( PreEscaped(r#"

     <h1 class="page-head" id="title">Contact me</h1>

        <p>The best place to contact me is my email: <a href="mailto:me@j0.lol">me@j0.lol</a></p>

        <h2>On the internet</h2>
        <ul>
            <li><a href="https://github.com/j0lol">Github</a></li>
            <li><a href="https://bsky.app/profile/j0.lol">Bluesky</a></li>
            <li><a href="https://wetdry.world/@j0">Mastodon</a></li>
            <li><a href="https://tangled.sh/@j0.lol">Tangled</a></li>
        </ul>
<!--        -->
<!--        <ul>-->
<!--            <li>Bluesky: <a href="https://bsky.app/profile/j0.lol">@j0.lol</a></li>-->
<!--            <li>Mastodon: <a href="https://wetdry.world/@j0">@j0@wetdry.world</a></li>-->
<!--        </ul>-->

        <h2>Chat with me</h2>
        <ul>
            <li>Discord: <samp>@nixpkgs</samp></li>
            <li>IRC: <samp>j0lol</samp> on libera</li>
                <details>
                    <summary>Those other chat apps nobody uses</summary>
                    <ul>
                        <li> <b>Seriously, like nobody uses these! I really only have accounts on these because they might somehow be the next "new thing" when Discord explodes or whatever. The internet is exhausting.</b></li>
                        <li> Matrix main account: @deer:f0rest.net</li>
                        <li> Matrix previous/backup accounts:
                            <ul>
                                <li>@j0:barr0w.net</li>
                                <li>@j0lol:beeper.com</li>
                                <li>@j0lol:the-apothecary.club</li>
                                <li>@j0lol:matrix.org</li>
                            </ul>
                        </li>
                        <li> XMPP: j0@wetdry.world</li>
                    </ul>
                </details>
        </ul>
        "#))
        },
        "/contact",
    )
}

#[handler]
pub fn projects() -> Markup {
    let projects = [
        [
            "bl0ck",
            "Voxel game engine made with WebGPU.",
            "https://github.com/j0lol/bl0ck",
            "2024–2025",
            "bl0ck.png",
            "https://vps.j0.lol/fyptest/",
            "https://vps.j0.lol/FinalReport%20%28anonymised%29.pdf",
        ],
        [
            "Sokobubble",
            "Block-pushing (via bubbles) game for Global Game Jam 2025",
            "https://github.com/j0lol/ggj25",
            "2025",
            "sokobubble.jpg",
            "",
            "",
        ],
        [
            "mel0n",
            "Sphere-stacker game for the GBA. Rust & agb-rs.",
            "https://github.com/j0lol/mel0n",
            "2024 🚧",
            "mel0n.jpg",
            "",
            "",
        ],
        [
            "Dig",
            "Side-view 2D Platformer with Building. Rust & Macroquad. Click to play! (Keyboard + Mouse)",
            "https://github.com/j0lol/dig-v2",
            "2024 🚧",
            "dig.jpg",
            "https://vps.j0.lol/dig/",
            "",
        ],
        [
            "ZeroBridge",
            "Discord bridge for Minecraft 1.4.7. NilLoader mod.",
            "https://github.com/j0lol/ZeroBridge",
            "2024",
            "zerobridge.jpg",
            "",
            "",
        ],
        [
            "World Conquest",
            "Group project: Risk clone made in Godot. I was project lead.",
            "https://github.com/goblin-code-se/world_conquest",
            "2024",
            "conquest.jpg",
            "https://vps.j0.lol/conquest",
            "",
        ],
        [
            "TIC-80 Tetris",
            "Physics-accurate Tetris clone for the TIC-80 Fantasy console. Click to play! (Keyboard only)",
            "",
            "2023",
            "tetr.jpg",
            "https://vps.j0.lol/tetr/",
            "",
        ],
        [
            "launCCher",
            "ToonTown: Corporate Clash launcher for Linux. Rust & egui.",
            "https://codeberg.org/j0/launccher",
            "2023",
            "launccher.jpg",
            "",
            "",
        ],
        [
            "Modulus",
            "Modular tool mod for Minecraft. Kotlin, Quilt mod.",
            "https://github.com/j0lol/modulus",
            "2022-2024 🚧",
            "modulus.png",
            "",
            "",
        ],
        [
            "Quicksnad",
            "Simple Fabric Minecraft mod that lets you grow cacti & sugarcane quicker. Java.",
            "https://modrinth.com/mod/quicksnad",
            "2022",
            "snad.png",
            "",
            "",
        ],
        [
            "TransVoiceParty",
            "List of trans voice training resources.",
            "https://transvoice.party",
            "2019-now",
            "tvp.jpg",
            "",
            "",
        ],
    ];
    page(
        &html! {
            h1.page-head { "Projects" }
            p { "These are my most notable projects, from latest to oldest." }
            .card-stack {
            @for [name, description, url, year, imslug, play, report] in projects {
                .card {
                    .preview {
                        img.raw src={"/static/card_images/" (imslug)} alt="Screenshot of project";
                    }
                    .blurb {
                        span.title {
                            (name) " "
                            span.year { (year) }
                        }
                        span.description { (description) }
                    }
                    .links {
                        @if !play.is_empty() {
                            a href=(play) { "Play" }
                        }
                        @if !url.is_empty() {
                            a href=(report) { "Source" }
                        }
                        @if !report.is_empty() {
                            a href=(report) { "Report" }
                        }
                    }
                }
            }
            }
        },
        "/projects",
    )
}
