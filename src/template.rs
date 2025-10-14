use maud::{DOCTYPE, Markup, html};

pub fn header() -> Markup {
    html! {
        (DOCTYPE)
        head {
            link rel="stylesheet" href="/static/style.css";
        }
    }
}
