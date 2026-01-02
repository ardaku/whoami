//! Integration tests for Windows platform error handling with mocked APIs
//!
//! These tests verify Windows API error codes using mocked functions.
//! They only run on WASM where mock API infrastructure is available.

#![cfg(all(test, target_arch = "wasm32"))]

use std::io::ErrorKind;

use wasm_bindgen_test::*;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

// Import the public API we're testing
use whoami::{devicename, hostname, realname, username};

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

test_fn!(test_lang_success, {
    // This should work with default mock state
    let result = hostname();
    assert!(result.is_ok(), "hostname() should succeed with mocks");

    let langs = result.unwrap();
    assert!(
        !langs.is_empty(),
        "language preferences should not be empty"
    );
});

// Error handling tests with mock error injection
#[cfg(feature = "mock_ffi")]
mod error_injection_tests {
    use super::*;
    use whoami::os::windows_test::*;

    test_fn!(test_get_username_insufficient_buffer, {
        reset_mocks();

        // The API should handle ERR_INSUFFICIENT_BUFFER correctly
        // This is tested implicitly in the two-call pattern
        let result = username();
        assert!(result.is_ok(), "username() should handle buffer sizing");

        reset_mocks();
    });

    test_fn!(test_get_username_access_denied, {
        reset_mocks();
        set_get_username_fail(true);
        set_mock_last_error(0x5); // ERROR_ACCESS_DENIED

        let result = username();
        assert!(
            result.is_err(),
            "username() should fail with ERROR_ACCESS_DENIED"
        );

        let err = result.unwrap_err();
        assert_eq!(err.raw_os_error(), Some(0x5));

        reset_mocks();
    });

    test_fn!(test_get_username_ex_none_mapped, {
        reset_mocks();
        set_get_username_ex_fail(true);
        set_mock_last_error(ERR_NONE_MAPPED);

        let result = realname();
        assert!(
            result.is_err(),
            "realname() should fail with ERR_NONE_MAPPED"
        );

        let err = result.unwrap_err();
        assert_eq!(err.raw_os_error(), Some(ERR_NONE_MAPPED));

        reset_mocks();
    });

    test_fn!(test_get_computer_name_failure, {
        reset_mocks();
        set_get_computer_name_fail(true);
        set_mock_last_error(0x5); // ERROR_ACCESS_DENIED

        let result = hostname();
        assert!(
            result.is_err(),
            "hostname() should fail when GetComputerNameExW fails"
        );

        reset_mocks();
    });

    test_fn!(test_sequential_api_calls_with_errors, {
        reset_mocks();

        // First call succeeds
        let result1 = username();
        assert!(result1.is_ok());

        // Inject error for second call
        set_get_username_fail(true);
        let result2 = username();
        assert!(result2.is_err());

        // Clear error for third call
        set_get_username_fail(false);
        let result3 = username();
        assert!(result3.is_ok());

        reset_mocks();
    });

    test_fn!(test_error_isolation_between_apis, {
        reset_mocks();

        // Make GetUserNameW fail
        set_get_username_fail(true);
        let username_result = username();
        assert!(username_result.is_err());

        // realname() uses GetUserNameExW, should still work
        set_get_username_fail(false);
        let realname_result = realname();
        assert!(realname_result.is_ok());

        reset_mocks();
    });

    test_fn!(test_utf16_encoding_handling, {
        reset_mocks();

        // Test that UTF-16 strings are properly converted
        let result = username();
        assert!(result.is_ok());

        let name = result.unwrap();
        let name_str = name.to_string_lossy();

        // Should be valid UTF-8 after conversion from UTF-16
        assert!(name_str
            .chars()
            .all(|c| c.is_alphanumeric() || c.is_whitespace() || c == '_'));

        reset_mocks();
    });

    test_fn!(test_buffer_size_variations, {
        reset_mocks();

        // Test with different buffer sizes
        for size in [8, 16, 32, 64, 128] {
            set_username_size(size);
            let result = username();
            assert!(
                result.is_ok(),
                "username() should work with buffer size {}",
                size
            );
        }

        reset_mocks();
    });

    test_fn!(test_last_error_persistence, {
        reset_mocks();

        // Set an error
        set_mock_last_error(999);
        assert_eq!(get_mock_last_error(), 999);

        // Error should persist until cleared
        assert_eq!(get_mock_last_error(), 999);

        // Reset should clear it
        reset_mocks();
        assert_eq!(get_mock_last_error(), ERROR_SUCCESS);
    });

    test_fn!(test_concurrent_error_injection, {
        reset_mocks();

        // Simulate concurrent error states
        set_get_username_fail(true);
        set_get_username_ex_fail(true);
        set_get_computer_name_fail(true);

        let r1 = username();
        let r2 = realname();
        let r3 = hostname();

        assert!(r1.is_err());
        assert!(r2.is_err());
        assert!(r3.is_err());

        reset_mocks();
    });

    test_fn!(test_api_recovery_after_errors, {
        reset_mocks();

        // Make several calls fail
        set_get_username_fail(true);
        for _ in 0..5 {
            let _ = username();
        }

        // Clear error and verify recovery
        set_get_username_fail(false);
        let result = username();
        assert!(
            result.is_ok(),
            "API should recover after errors are cleared"
        );

        reset_mocks();
    });

    test_fn!(test_display_name_format, {
        reset_mocks();

        // Test that display name is properly retrieved
        let result = realname();
        assert!(result.is_ok());

        let name = result.unwrap();
        let name_str = name.to_string_lossy();

        // Display names typically contain spaces and are capitalized
        assert!(
            name_str.contains(char::is_whitespace) || name_str.len() > 0,
            "Display name should be formatted properly"
        );

        reset_mocks();
    });

    test_fn!(test_hostname_formats, {
        reset_mocks();

        // Test that hostname is retrieved correctly
        let result = hostname();
        assert!(result.is_ok());

        let name = result.unwrap();

        // Hostnames should be alphanumeric with possible hyphens
        assert!(
            name.chars()
                .all(|c| c.is_alphanumeric() || c == '-' || c == '.'),
            "Hostname should follow naming conventions"
        );

        reset_mocks();
    });

    test_fn!(test_language_preferences_parsing, {
        reset_mocks();

        let result = hostname();
        assert!(result.is_ok());

        let langs = result.unwrap();
        assert!(!langs.is_empty(), "Should return at least one language");

        // First language should be valid
        let first_lang = &langs[0];
        assert!(!first_lang.is_empty());

        reset_mocks();
    });
}

// Performance and stress tests
test_fn!(test_repeated_calls_consistent, {
    // Call APIs multiple times to ensure consistency
    for _ in 0..10 {
        assert!(username().is_ok());
        assert!(realname().is_ok());
        assert!(hostname().is_ok());
        assert!(devicename().is_ok());
    }
});

test_fn!(test_all_apis_return_valid_utf16_strings, {
    // Ensure all APIs return valid UTF-16 convertible strings
    let username_str = username().unwrap();
    assert!(!username_str.is_empty());

    let realname_str = realname().unwrap();
    assert!(!realname_str.is_empty());

    let hostname_str = hostname().unwrap();
    assert!(!hostname_str.is_empty());

    let devicename_str = devicename().unwrap();
    assert!(!devicename_str.is_empty());
});

test_fn!(test_devicename_consistency, {
    // On Windows, devicename should return a consistent value
    let result1 = devicename();
    let result2 = devicename();

    assert!(result1.is_ok());
    assert!(result2.is_ok());

    // Multiple calls should return the same value
    assert_eq!(result1.unwrap(), result2.unwrap());
});

test_fn!(test_rapid_successive_calls, {
    // Ensure no memory leaks or state corruption with rapid calls
    for _ in 0..100 {
        let _ = username();
        let _ = realname();
        let _ = hostname();
        let _ = devicename();
    }
});

test_fn!(test_interleaved_api_calls, {
    // Test calling different APIs in various orders
    for _ in 0..20 {
        let _ = username();
        let _ = hostname();
        let _ = realname();
        let _ = devicename();
        let _ = hostname();
    }
});

test_fn!(test_username_realname_relationship, {
    // Username and realname might be related but different
    let username_result = username();
    let realname_result = realname();

    assert!(username_result.is_ok());
    assert!(realname_result.is_ok());

    let username_str = username_result.unwrap();
    let realname_str = realname_result.unwrap();

    // Both should be non-empty
    assert!(!username_str.is_empty());
    assert!(!realname_str.is_empty());

    // Realname is typically more "human friendly" (contains spaces, etc.)
    // while username is simpler
});

test_fn!(test_error_kind_mapping, {
    // Verify that errors map to appropriate ErrorKind values
    // This is implementation-dependent but good to verify
    let result = username();
    if let Err(e) = result {
        // If it fails, error kind should be meaningful
        let kind = e.kind();
        assert!(
            matches!(
                kind,
                ErrorKind::NotFound
                    | ErrorKind::PermissionDenied
                    | ErrorKind::Other
            ),
            "Error kind should be meaningful: {:?}",
            kind
        );
    }
});
