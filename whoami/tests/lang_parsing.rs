use std::str::FromStr;

use whoami::Language;

#[test]
fn test_parsing() {
    assert!(Language::from_str("EN/US").is_err());
    assert!(Language::from_str("en/us").is_err());
    assert!(Language::from_str("eng/US").is_err());
    assert!(Language::from_str("en/USA").is_err());
    assert!(Language::from_str("en/").is_err());
    assert!(Language::from_str("/").is_err());
    assert!(Language::from_str("/US").is_err());

    Language::from_str("en/US").unwrap();
    Language::from_str("en").unwrap();
}
