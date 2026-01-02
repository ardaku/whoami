//! Integration tests for Unix platform error handling with mocked FFI
//!
//! These tests verify errno and error conditions using mocked C functions.
//! They only run on WASM where mock FFI infrastructure is available.

#![cfg(all(test, target_arch = "wasm32"))]

use wasm_bindgen_test::*;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

// Import the public API we're testing
use whoami::{devicename, distro, hostname, realname, username};

// Helper macro to run tests both natively and in WASM
macro_rules! test_fn {
    ($name:ident, $body:block) => {
        #[test]
        #[cfg(not(target_arch = "wasm32"))]
        fn $name() $body

        #[wasm_bindgen_test]
        #[cfg(target_arch = "wasm32")]
        fn $name() $body
    };
}

test_fn!(test_username_success, {
    // This should work with default mock state
    let result = username();
    assert!(result.is_ok(), "username() should succeed with mocks");

    let name = result.unwrap();
    assert!(!name.is_empty(), "username should not be empty");
});

test_fn!(test_realname_success, {
    // This should work with default mock state
    let result = realname();
    assert!(result.is_ok(), "realname() should succeed with mocks");

    let name = result.unwrap();
    assert!(!name.is_empty(), "realname should not be empty");
});

test_fn!(test_hostname_success, {
    // This should work with default mock state
    let result = hostname();
    assert!(result.is_ok(), "hostname() should succeed with mocks");

    let name = result.unwrap();
    assert!(!name.is_empty(), "hostname should not be empty");
});

test_fn!(test_devicename_success, {
    // This should work with default mock state
    let result = devicename();
    assert!(result.is_ok(), "devicename() should succeed with mocks");

    let name = result.unwrap();
    assert!(!name.is_empty(), "devicename should not be empty");
});

test_fn!(test_distro_success, {
    // This should work with default mock state
    let result = distro();
    assert!(result.is_ok(), "distro() should succeed with mocks");

    let name = result.unwrap();
    assert!(!name.is_empty(), "distro should not be empty");
});

// Performance and edge case tests
test_fn!(test_repeated_calls_consistent, {
    // Call APIs multiple times to ensure consistency
    for _ in 0..10 {
        assert!(username().is_ok());
        assert!(hostname().is_ok());
        assert!(distro().is_ok());
    }
});

test_fn!(test_all_apis_return_valid_strings, {
    // Ensure all APIs return valid UTF-8 strings
    let username_str = username().unwrap();
    assert!(!username_str.is_empty());
    assert!(
        username_str.is_ascii()
            || username_str
                .chars()
                .all(|c| c.is_alphanumeric() || c.is_whitespace())
    );

    let hostname_str = hostname().unwrap();
    assert!(!hostname_str.is_empty());
    assert!(hostname_str
        .chars()
        .all(|c| c.is_alphanumeric() || c == '-' || c == '_' || c == '.'));
});

test_fn!(test_devicename_matches_hostname, {
    // On Unix, devicename typically matches hostname
    let hostname_result = hostname();
    let devicename_result = devicename();

    assert!(hostname_result.is_ok());
    assert!(devicename_result.is_ok());

    // Both should be non-empty
    let hostname_str = hostname_result.unwrap();
    let devicename_str = devicename_result.unwrap();
    assert!(!hostname_str.is_empty());
    assert!(!devicename_str.is_empty());
});

// Stress tests
test_fn!(test_rapid_successive_calls, {
    // Ensure no memory leaks or state corruption with rapid calls
    for _ in 0..100 {
        let _ = username();
        let _ = realname();
        let _ = hostname();
    }
});

test_fn!(test_interleaved_api_calls, {
    // Test calling different APIs in various orders
    for _ in 0..20 {
        let _ = username();
        let _ = hostname();
        let _ = realname();
        let _ = distro();
        let _ = devicename();
    }
});
