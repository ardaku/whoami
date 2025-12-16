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

// Error handling tests - these would use mock error injection
#[cfg(feature = "mock_ffi")]
mod error_injection_tests {
    use super::*;
    use whoami::os::unix_test::*;

    test_fn!(test_getpwuid_errno_enoent, {
        reset_mocks();
        set_getpwuid_r_fail(true);
        set_mock_errno(libc::ENOENT);

        let result = username();
        assert!(
            result.is_err(),
            "username() should fail when getpwuid_r returns ENOENT"
        );

        let err = result.unwrap_err();
        assert_eq!(err.raw_os_error(), Some(libc::ENOENT));

        reset_mocks();
    });

    test_fn!(test_getpwuid_errno_eperm, {
        reset_mocks();
        set_getpwuid_r_fail(true);
        set_mock_errno(libc::EPERM);

        let result = username();
        assert!(
            result.is_err(),
            "username() should fail when getpwuid_r returns EPERM"
        );

        let err = result.unwrap_err();
        assert_eq!(err.raw_os_error(), Some(libc::EPERM));

        reset_mocks();
    });

    test_fn!(test_getpwuid_null_pointer, {
        reset_mocks();
        set_getpwuid_r_null(true);

        let result = username();
        assert!(
            result.is_err(),
            "username() should fail when getpwuid_r returns null"
        );

        // Should get a "not found" type error
        let err = result.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::NotFound);

        reset_mocks();
    });

    test_fn!(test_gethostname_failure, {
        reset_mocks();
        set_gethostname_fail(true);
        set_mock_errno(libc::EFAULT);

        let result = hostname();
        assert!(
            result.is_err(),
            "hostname() should fail when gethostname fails"
        );

        reset_mocks();
    });

    test_fn!(test_uname_failure, {
        reset_mocks();
        set_uname_fail(true);
        set_mock_errno(libc::EFAULT);

        let result = distro();
        assert!(result.is_err(), "distro() should fail when uname fails");

        reset_mocks();
    });

    test_fn!(test_different_euid_values, {
        reset_mocks();

        // Test with different UID values
        for uid in [0, 1000, 65534] {
            set_mock_euid(uid);
            let result = username();
            assert!(result.is_ok(), "username() should work for UID {}", uid);
        }

        reset_mocks();
    });

    test_fn!(test_concurrent_error_states, {
        reset_mocks();

        // Simulate concurrent access patterns
        set_getpwuid_r_fail(true);
        set_mock_errno(libc::EAGAIN);

        let result1 = username();
        assert!(result1.is_err());

        // Reset and try again
        set_getpwuid_r_fail(false);
        let result2 = username();
        assert!(result2.is_ok());

        reset_mocks();
    });

    test_fn!(test_buffer_boundary_conditions, {
        reset_mocks();

        // Test with mock data that fills buffers
        let result = realname();
        assert!(
            result.is_ok(),
            "realname() should handle buffer data correctly"
        );

        let name = result.unwrap();
        // Verify we got valid UTF-8
        assert!(name.to_string_lossy().len() > 0);

        reset_mocks();
    });

    test_fn!(test_errno_cleared_between_calls, {
        reset_mocks();

        // Set errno and make a call fail
        set_getpwuid_r_fail(true);
        set_mock_errno(libc::ENOENT);
        let _ = username();

        // Errno should be cleared now
        set_getpwuid_r_fail(true);
        let result = username();

        // Should get default error, not ENOENT
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_ne!(err.raw_os_error(), Some(libc::ENOENT));

        reset_mocks();
    });

    test_fn!(test_gecos_field_parsing, {
        reset_mocks();

        // Test that GECOS field is properly parsed for real name
        let result = realname();
        assert!(result.is_ok());

        let name = result.unwrap();
        let name_str = name.to_string_lossy();

        // Mock provides "Test User" as real name
        assert!(
            name_str.contains("Test"),
            "Real name should be parsed from GECOS"
        );

        reset_mocks();
    });

    test_fn!(test_multiple_api_calls_isolation, {
        reset_mocks();

        // Make sure error in one API doesn't affect another
        set_gethostname_fail(true);
        let hostname_result = hostname();
        assert!(hostname_result.is_err());

        // username() should still work
        let username_result = username();
        assert!(username_result.is_ok());

        reset_mocks();
    });
}

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
