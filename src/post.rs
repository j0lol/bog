pub mod admin;
pub mod fetch;
pub mod list;
pub mod render;
pub mod view;

use std::fs::read_to_string;

use chrono::{DateTime, Datelike as _, FixedOffset, Local, NaiveDateTime, Utc};
use maud::{Markup, html};
use serde::Deserialize;
const ISO8601_DATE: &str = "%Y-%m-%dT%H:%M";

#[derive(Deserialize, Debug)]
pub struct Post {
    pub bsky_uri: Option<String>,
    pub category: Option<String>,
    pub contents: String,
    pub creation_datetime: DateTime<Local>,
    pub slug: String,
    pub subtitle: Option<String>,
    pub title: String,
}

#[must_use]
pub fn clock_icon() -> Markup {
    html! {
        span.emoji-icon aria-label="Published on" {( "🕒" )}
    }
}

#[must_use]
pub fn clean_empty_string(s: Option<String>) -> Option<String> {
    match s {
        Some(string) if string.trim().is_empty() => None,
        other => other,
    }
}

#[must_use]
pub fn format_date(date: chrono::DateTime<Local>) -> String {
    let suffix = eng_ordinal_suffix(date.day() as usize);
    date.format(&format!("%B %d{suffix}, %Y")).to_string()
}

#[must_use]
pub fn parse_date(datestring: &str) -> chrono::DateTime<FixedOffset> {
    match chrono::DateTime::parse_from_rfc3339(datestring) {
        Ok(dt) => dt,
        Err(_) => {
            match NaiveDateTime::parse_from_str(datestring, "%Y-%m-%dT%H:%M")
                .map(|ndt| DateTime::<Utc>::from_naive_utc_and_offset(ndt, Utc))
            {
                Ok(dt) => dt.into(),
                Err(_) => chrono::DateTime::UNIX_EPOCH.into(),
            }
        }
    }
}

#[must_use]
#[expect(
    clippy::expect_used,
    reason = "We need the secret for progam execution"
)]
pub fn read_secret() -> String {
    read_to_string("./.secret")
        .expect("Failed to read secret file")
        .trim()
        .to_owned()
}

#[must_use]
pub fn eng_ordinal_suffix(n: usize) -> &'static str {
    let tens = n % 100;
    if (11..=13).contains(&tens) {
        return "th";
    }

    match n % 10 {
        1 => "st",
        2 => "nd",
        3 => "rd",
        _ => "th",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::AppError;

    #[test]
    fn test_eng_ordinal_suffix_basic() {
        assert_eq!(eng_ordinal_suffix(1), "st");
        assert_eq!(eng_ordinal_suffix(2), "nd");
        assert_eq!(eng_ordinal_suffix(3), "rd");
        assert_eq!(eng_ordinal_suffix(4), "th");
        assert_eq!(eng_ordinal_suffix(11), "th");
        assert_eq!(eng_ordinal_suffix(12), "th");
        assert_eq!(eng_ordinal_suffix(13), "th");
        assert_eq!(eng_ordinal_suffix(21), "st");
        assert_eq!(eng_ordinal_suffix(22), "nd");
        assert_eq!(eng_ordinal_suffix(23), "rd");
    }

    #[test]
    fn test_app_error_conversion() {
        let db_error = rusqlite::Error::QueryReturnedNoRows;
        let app_error: AppError = db_error.into();
        match app_error {
            AppError::NotFound => {}
            _ => panic!("Should convert to NotFound"),
        }
    }
}
