use std::sync::LazyLock;

use regex::Regex;

static ID_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[A-Za-z]{2}\d{6}$").unwrap());

pub fn is_valid_id(id: &str) -> bool {
    ID_REGEX.is_match(id)
}
