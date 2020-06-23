#[macro_use]
extern crate lazy_static;

use chrono::NaiveDate;
use regex::Regex;

lazy_static! {
    static ref EMAIL_REGEX: Regex = Regex::new(
        r#"^[a-zA-Z0-9.!#$%&’*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,253}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,253}[a-zA-Z0-9])?)*$"#
    ).unwrap();

    static ref DATE_FORMATS: Vec<&'static str> = vec!["%F", "%x", "%m/%d/%Y"];
}

#[no_mangle]
pub extern "C" fn validate_email(email: &str) -> bool {
    EMAIL_REGEX.is_match(email)
}

#[no_mangle]
pub extern "C" fn validate_date(date: &str) -> bool {
    DATE_FORMATS
        .iter()
        .any(|format| NaiveDate::parse_from_str(date, format).is_ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_email_valid() {
        assert!(validate_email("ethan@kent.com"));
    }

    #[test]
    fn test_validate_email_invalid() {
        assert!(!validate_email("ethan@kent@com"));
    }

    #[test]
    fn test_validate_date_valid() {
        assert!(validate_date("1981-08-26"));
    }

    #[test]
    fn test_validate_date_invalid() {
        assert!(!validate_date("1981-02-29"));
    }
}
