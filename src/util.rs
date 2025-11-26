use std::sync::LazyLock;

use chrono::{Local, NaiveDateTime};
use regex::Regex;

static ID_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[A-Za-z]{2}\d{6}$").unwrap());

pub fn is_valid_id(id: &str) -> bool {
    ID_REGEX.is_match(id)
}

pub fn format_datetime(dt: NaiveDateTime) -> String {
    let now = Local::now().naive_local();
    let delta = now - dt;

    let secs = delta.num_seconds().abs();

    let mins = (secs as f64 / 60.0).ceil() as i64;
    if mins < 60 {
        return format!("{} phút trước", mins.max(1));
    }

    let days = (secs as f64 / 86400.0).ceil() as i64;
    if days < 30 {
        return format!("{} ngày trước", days);
    }

    let months = (days as f64 / 30.0).round() as i64;
    if months < 12 {
        return format!("{} tháng trước", months.max(1));
    }

    let years = (days as f64 / 365.0).round() as i64;
    format!("{} năm trước", years.max(1))
}
