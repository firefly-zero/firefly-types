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

/// Validate a name (app name, author name).
///
/// We currently force all text to be printable ASCII.
/// Keep in mind that the validation DOES NOT ensure
/// that the text is a safe file name.
/// You need to sanitize it first to use in a file path.
pub fn valid_name(s: &str) -> bool {
    if s.len() > 40 {
        return false;
    }
    if s.starts_with(' ') || s.ends_with(' ') {
        return false;
    }
    let mut alnum_found = false;
    for c in s.bytes() {
        if c.is_ascii_alphanumeric() {
            alnum_found = true;
            continue;
        }
        if !c.is_ascii_punctuation() && c != b' ' {
            return false;
        }
    }
    alnum_found
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
        assert!(!valid_id("-"));
        assert!(!valid_id("--"));
    }

    #[test]
    fn test_valid_name() {
        assert!(valid_name("app"));
        assert!(valid_name("a"));
        assert!(valid_name("some-app"));
        assert!(valid_name("App"));
        assert!(valid_name("Some app"));
        assert!(valid_name("Some App"));
        assert!(valid_name("SOME APP"));
    }

    #[test]
    fn test_invalid_name() {
        assert!(!valid_name(" "));
        assert!(!valid_name("  "));
        assert!(!valid_name(""));
        assert!(!valid_name(" abc"));
        assert!(!valid_name("abc "));
        assert!(!valid_name("ab\tcd"));
        assert!(!valid_name("тест"));
    }
}
