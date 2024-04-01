/// Validate the author or the app ID.
///
/// The ID should have at least one character and may contain only
/// ASCII lowercase letters, ASCII digits, and hyphen.
///
/// Without ID validation, an app may use a malformed ID (like "../../../")
/// to gain access to arbitrary files of other apps, including secrets.
pub fn valid_id(s: &str) -> bool {
    if s.starts_with('-') || s.ends_with('-') {
        return false;
    }
    if s.contains("--") {
        return false;
    }
    if s.is_empty() {
        return false;
    }
    for c in s.bytes() {
        if !c.is_ascii_lowercase() && !c.is_ascii_digit() && c != b'-' {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_id() {
        assert!(valid_id("app"));
        assert!(valid_id("some-app"));
        assert!(valid_id("some-app-13"));
        assert!(valid_id("13app"));
        assert!(valid_id("relatively-long-app-name"));
        assert!(valid_id("a"));
    }

    #[test]
    fn test_invalid_id() {
        assert!(!valid_id("app.name"));
        assert!(!valid_id("app--name"));
        assert!(!valid_id("-appname"));
        assert!(!valid_id("-app-name"));
        assert!(!valid_id("-appname"));
        assert!(!valid_id("app-name-"));
        assert!(!valid_id("appname-"));
        assert!(!valid_id("-appname-"));
        assert!(!valid_id("app name"));
        assert!(!valid_id("appname "));
        assert!(!valid_id(" appname"));
        assert!(!valid_id("App"));
        assert!(!valid_id("AppName"));
        assert!(!valid_id("APPNAME"));
        assert!(!valid_id(""));
        assert!(!valid_id(" "));
    }
}
