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
    if s.ends_with(' ') {
        return false;
    }
    let mut b = s.bytes();
    match b.next() {
        // Must start with a letter.
        Some(c) => {
            if !c.is_ascii_alphabetic() {
                return false;
            }
        }
        // Must not be empty.
        None => return false,
    }
    for c in b {
        if c.is_ascii_alphanumeric() {
            continue;
        }
        if !c.is_ascii_punctuation() && c != b' ' {
            return false;
        }
    }
    true
}

/// Validate a path component (file or directory name).
pub fn valid_path_part(s: &str) -> bool {
    if s.starts_with('.') {
        return false;
    }
    if s.is_empty() {
        return false;
    }
    for c in s.bytes() {
        if c.is_ascii_alphanumeric() {
            continue;
        }
        if c != b'.' && c != b'_' && c != b'-' {
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
        assert!(!valid_id("-"));
        assert!(!valid_id("--"));
        assert!(!valid_id("?hello"));
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
        assert!(valid_name("Hello"));
        assert!(valid_name("Hello?"));
        assert!(valid_name("Yes? Yes!"));
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
        assert!(!valid_name("?hello"));
    }

    #[test]
    fn test_valid_path_part() {
        assert!(valid_path_part("app"));
        assert!(valid_path_part("a"));
        assert!(valid_path_part("some-app"));
        assert!(valid_path_part("App"));
        assert!(valid_path_part("file.wasm"));
        assert!(valid_path_part("file_name.wasm"));
        assert!(valid_path_part("FileName.wasm"));
    }

    #[test]
    fn test_invalid_path_part() {
        assert!(!valid_path_part(".gitignore"));
        assert!(!valid_path_part(".."));
        assert!(!valid_path_part("/"));
        assert!(!valid_path_part("./"));
        assert!(!valid_path_part("???"));
        assert!(!valid_path_part("file/../root"));
        assert!(!valid_path_part("file name"));
        assert!(!valid_path_part(" file"));
        assert!(!valid_path_part("file "));
        assert!(!valid_path_part(""));
        assert!(!valid_path_part(" "));
    }
}
