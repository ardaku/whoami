use std::{mem, str::FromStr};

use whoami::Language;

#[test]
fn parsing() {
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

#[test]
fn lang_size() {
    assert_eq!(4, mem::size_of::<Language>());
}

#[test]
fn lang_equivalence() {
    assert_eq!(Language::from_str("en/US").unwrap(), "en/US");
    assert_eq!("en/US", Language::from_str("en/US").unwrap());

    assert_eq!(Language::from_str("en_US").unwrap(), "en-US");
    assert_eq!("en-US", Language::from_str("en_US").unwrap());

    assert_eq!(Language::from_str("en-US").unwrap(), "en_US");
    assert_eq!("en_US", Language::from_str("en-US").unwrap());

    assert_ne!(Language::from_str("en-GB").unwrap(), "en_US");
    assert_ne!("en_US", Language::from_str("en-GB").unwrap());

    assert_ne!(Language::from_str("en/US").unwrap(), "en|US");
    assert_ne!("en|US", Language::from_str("en/US").unwrap());
}

#[test]
fn lang_to_string() {
    assert_eq!("en/US", Language::from_str("en/US").unwrap().to_string());
}
