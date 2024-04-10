use core::fmt::Display;

pub enum ValidationError {
    TrailingMinus,
    DoubleMinus,
    Empty,
    InvalidChar(u8),
    InvalidFirstChar(u8),
    TooLong,
    TrailingSpace,
    TrailingDot,
    Reserved,
}

impl Display for ValidationError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ValidationError::TrailingMinus => write!(f, "must not start or end with minus"),
            ValidationError::DoubleMinus => write!(f, "must not contain '--'"),
            ValidationError::Empty => write!(f, "must not be empty"),
            ValidationError::InvalidChar(c) => match char::from_u32(*c as u32) {
                Some(c) => write!(f, "must not contain {c}"),
                None => write!(f, "must contain only valid ASCII characters"),
            },
            ValidationError::InvalidFirstChar(c) => match char::from_u32(*c as u32) {
                Some(c) => write!(f, "must not start with {c}"),
                None => write!(f, "must start with an ASCII character"),
            },
            ValidationError::TooLong => write!(f, "too long"),
            ValidationError::TrailingSpace => write!(f, "must not start or end with space"),
            ValidationError::TrailingDot => write!(f, "must not start or end with dot"),
            ValidationError::Reserved => write!(f, "the name is reserved"),
        }
    }
}

/// Validate the author or the app ID.
///
/// The ID should have at least one character and may contain only
/// ASCII lowercase letters, ASCII digits, and hyphen.
///
/// Without ID validation, an app may use a malformed ID (like "../../../")
/// to gain access to arbitrary files of other apps, including secrets.
pub fn validate_id(s: &str) -> Result<(), ValidationError> {
    if s.len() > 16 {
        return Err(ValidationError::TooLong);
    }
    if s.starts_with('-') || s.ends_with('-') {
        return Err(ValidationError::TrailingMinus);
    }
    if s.contains("--") {
        return Err(ValidationError::DoubleMinus);
    }
    if s.is_empty() {
        return Err(ValidationError::Empty);
    }
    for c in s.bytes() {
        if !c.is_ascii_lowercase() && !c.is_ascii_digit() && c != b'-' {
            return Err(ValidationError::InvalidChar(c));
        }
    }
    Ok(())
}

/// Validate a name (app name, author name).
///
/// We currently force all text to be printable ASCII.
/// Keep in mind that the validation DOES NOT ensure
/// that the text is a safe file name.
/// You need to sanitize it first to use in a file path.
pub fn validate_name(s: &str) -> Result<(), ValidationError> {
    if s.len() > 40 {
        return Err(ValidationError::TooLong);
    }
    if s.ends_with(' ') {
        return Err(ValidationError::TrailingSpace);
    }
    let mut b = s.bytes();
    match b.next() {
        // Must start with a letter.
        Some(c) => {
            if !c.is_ascii_alphabetic() {
                return Err(ValidationError::InvalidFirstChar(c));
            }
        }
        // Must not be empty.
        None => return Err(ValidationError::Empty),
    }
    for c in b {
        if c.is_ascii_alphanumeric() {
            continue;
        }
        if !c.is_ascii_punctuation() && c != b' ' {
            return Err(ValidationError::InvalidChar(c));
        }
    }
    Ok(())
}

/// Validate a path component (file or directory name).
pub fn validate_path_part(s: &str) -> Result<(), ValidationError> {
    if s.starts_with('.') {
        return Err(ValidationError::TrailingDot);
    }
    if s.is_empty() {
        return Err(ValidationError::Empty);
    }
    if s == "meta" || s == "bin" {
        return Err(ValidationError::Reserved);
    }
    for c in s.bytes() {
        if c.is_ascii_alphanumeric() {
            continue;
        }
        if c != b'.' && c != b'_' && c != b'-' {
            return Err(ValidationError::InvalidChar(c));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_id() {
        assert!(validate_id("app").is_ok());
        assert!(validate_id("some-app").is_ok());
        assert!(validate_id("some-app-13").is_ok());
        assert!(validate_id("13app").is_ok());
        assert!(validate_id("a").is_ok());
        assert!(validate_id("a-bit-long-name").is_ok());
    }

    #[test]
    fn test_invalid_id() {
        assert!(validate_id("app.name").is_err());
        assert!(validate_id("app--name").is_err());
        assert!(validate_id("-appname").is_err());
        assert!(validate_id("-app-name").is_err());
        assert!(validate_id("-appname").is_err());
        assert!(validate_id("app-name-").is_err());
        assert!(validate_id("appname-").is_err());
        assert!(validate_id("-appname-").is_err());
        assert!(validate_id("app name").is_err());
        assert!(validate_id("appname ").is_err());
        assert!(validate_id(" appname").is_err());
        assert!(validate_id("App").is_err());
        assert!(validate_id("AppName").is_err());
        assert!(validate_id("APPNAME").is_err());
        assert!(validate_id("").is_err());
        assert!(validate_id(" ").is_err());
        assert!(validate_id("-").is_err());
        assert!(validate_id("--").is_err());
        assert!(validate_id("?hello").is_err());
        assert!(validate_id("a-very-long-app-name").is_err());
    }

    #[test]
    fn test_valid_name() {
        assert!(validate_name("app").is_ok());
        assert!(validate_name("a").is_ok());
        assert!(validate_name("some-app").is_ok());
        assert!(validate_name("App").is_ok());
        assert!(validate_name("Some app").is_ok());
        assert!(validate_name("Some App").is_ok());
        assert!(validate_name("SOME APP").is_ok());
        assert!(validate_name("Hello").is_ok());
        assert!(validate_name("Hello?").is_ok());
        assert!(validate_name("Yes? Yes!").is_ok());
    }

    #[test]
    fn test_invalid_name() {
        assert!(validate_name(" ").is_err());
        assert!(validate_name("  ").is_err());
        assert!(validate_name("").is_err());
        assert!(validate_name(" abc").is_err());
        assert!(validate_name("abc ").is_err());
        assert!(validate_name("ab\tcd").is_err());
        assert!(validate_name("тест").is_err());
        assert!(validate_name("?hello").is_err());
    }

    #[test]
    fn test_valid_path_part() {
        assert!(validate_path_part("app").is_ok());
        assert!(validate_path_part("a").is_ok());
        assert!(validate_path_part("some-app").is_ok());
        assert!(validate_path_part("App").is_ok());
        assert!(validate_path_part("file.wasm").is_ok());
        assert!(validate_path_part("file_name.wasm").is_ok());
        assert!(validate_path_part("FileName.wasm").is_ok());
    }

    #[test]
    fn test_invalid_path_part() {
        assert!(validate_path_part(".gitignore").is_err());
        assert!(validate_path_part("..").is_err());
        assert!(validate_path_part("/").is_err());
        assert!(validate_path_part("./").is_err());
        assert!(validate_path_part("???").is_err());
        assert!(validate_path_part("file/../root").is_err());
        assert!(validate_path_part("file name").is_err());
        assert!(validate_path_part(" file").is_err());
        assert!(validate_path_part("file ").is_err());
        assert!(validate_path_part("").is_err());
        assert!(validate_path_part(" ").is_err());
        assert!(validate_path_part("bin").is_err());
        assert!(validate_path_part("meta").is_err());
    }
}
