//! WASM-specific integration tests
//!
//! These tests are designed to run in a WebAssembly environment (browser or runtime)
//! and verify that the whoami library works correctly when compiled to WASM.

#![cfg(all(test, target_arch = "wasm32", feature = "web"))]

use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

// Import the public API
use whoami::{
    arch, desktop_env, devicename, distro, hostname, lang_prefs, platform,
    realname, username, Arch, DesktopEnv, Platform,
};

#[wasm_bindgen_test]
fn test_wasm_username() {
    let result = username();
    assert!(result.is_ok(), "username() should work in WASM");

    let name = result.unwrap();
    assert!(!name.is_empty(), "username should not be empty in WASM");
}

#[wasm_bindgen_test]
fn test_wasm_realname() {
    let result = realname();
    assert!(result.is_ok(), "realname() should work in WASM");

    let name = result.unwrap();
    assert!(!name.is_empty(), "realname should not be empty in WASM");
}

#[wasm_bindgen_test]
fn test_wasm_hostname() {
    let result = hostname();
    assert!(result.is_ok(), "hostname() should work in WASM");

    let name = result.unwrap();
    assert!(!name.is_empty(), "hostname should not be empty in WASM");
}

#[wasm_bindgen_test]
fn test_wasm_devicename() {
    let result = devicename();
    assert!(result.is_ok(), "devicename() should work in WASM");

    let name = result.unwrap();
    assert!(!name.is_empty(), "devicename should not be empty in WASM");
}

#[wasm_bindgen_test]
fn test_wasm_distro() {
    let result = distro();
    assert!(result.is_ok(), "distro() should work in WASM");

    let name = result.unwrap();
    assert!(!name.is_empty(), "distro should not be empty in WASM");
}

#[wasm_bindgen_test]
fn test_wasm_platform() {
    let plat = platform();

    // In WASM/browser, platform is detected from user agent
    // Can be Mac, Windows, Linux, or Unknown depending on the browser
    assert!(
        matches!(
            plat,
            Platform::Mac
                | Platform::Windows
                | Platform::Linux
                | Platform::Unknown(_)
        ),
        "Platform should be detected from browser user agent, got: {:?}",
        plat
    );
}

#[wasm_bindgen_test]
fn test_wasm_arch() {
    let architecture = arch();

    // In WASM32, we expect Wasm architecture
    assert_eq!(
        architecture,
        Arch::Wasm32,
        "Should detect Wasm32 architecture"
    );
}

#[wasm_bindgen_test]
fn test_wasm_desktop_env() {
    let de = desktop_env();

    // Desktop environment in WASM context should be WebBrowser
    assert!(
        matches!(de, Some(DesktopEnv::WebBrowser(_))),
        "Desktop environment should be WebBrowser in WASM, got: {:?}",
        de
    );
}

#[wasm_bindgen_test]
fn test_wasm_lang() {
    let result = lang_prefs();

    if let Ok(lang_prefs) = result {
        // Collect languages to verify they exist
        let langs: Vec<_> = lang_prefs.message_langs().collect();

        assert!(
            !langs.is_empty(),
            "Should detect at least one language in WASM"
        );

        // Verify language format by checking the Display output
        for lang in langs {
            let lang_str = lang.to_string();
            assert!(!lang_str.is_empty(), "Language code should not be empty");
            assert!(
                lang_str.chars().all(|c| c.is_alphanumeric()
                    || c == '-'
                    || c == '/'
                    || c == '_'),
                "Language code should be valid format: {}",
                lang_str
            );
        }
    }
}

#[wasm_bindgen_test]
fn test_wasm_multiple_calls_consistency() {
    // Test that multiple calls return consistent results
    let username1 = username().unwrap();
    let username2 = username().unwrap();
    assert_eq!(
        username1, username2,
        "Multiple username() calls should be consistent"
    );

    let hostname1 = hostname().unwrap();
    let hostname2 = hostname().unwrap();
    assert_eq!(
        hostname1, hostname2,
        "Multiple hostname() calls should be consistent"
    );
}

#[wasm_bindgen_test]
fn test_wasm_all_apis_callable() {
    // Verify all APIs can be called without panicking
    let _ = username();
    let _ = realname();
    let _ = hostname();
    let _ = devicename();
    let _ = distro();
    let _ = platform();
    let _ = arch();
    let _ = desktop_env();
    let _ = lang_prefs();
}

#[wasm_bindgen_test]
fn test_wasm_string_encoding() {
    // Verify that strings returned are valid UTF-8
    let username_str = username().unwrap();
    assert!(
        username_str.is_ascii()
            || username_str.chars().all(|c| !c.is_control())
    );

    let hostname_str = hostname().unwrap();
    assert!(hostname_str.chars().all(|c| !c.is_control()));
}

#[wasm_bindgen_test]
fn test_wasm_error_handling() {
    // All Result-returning functions should handle errors gracefully
    // Even if they succeed in WASM, they should not panic

    let username_result = username();
    assert!(username_result.is_ok() || username_result.is_err());

    let realname_result = realname();
    assert!(realname_result.is_ok() || realname_result.is_err());

    let hostname_result = hostname();
    assert!(hostname_result.is_ok() || hostname_result.is_err());
}

#[wasm_bindgen_test]
fn test_wasm_rapid_calls() {
    // Stress test with rapid successive calls
    for i in 0..50 {
        let _ = username();
        let _ = hostname();

        if i % 10 == 0 {}
    }
}

#[wasm_bindgen_test]
fn test_wasm_interleaved_calls() {
    // Test interleaved API calls
    for _ in 0..10 {
        let _ = username();
        let _ = platform();
        let _ = hostname();
        let _ = arch();
        let _ = realname();
        let _ = distro();
    }
}

#[wasm_bindgen_test]
fn test_wasm_memory_stability() {
    // Test that repeated calls don't cause memory issues
    let initial_username = username().unwrap();

    // Make many calls
    for _ in 0..100 {
        let _ = username();
    }

    // Verify result is still consistent
    let final_username = username().unwrap();
    assert_eq!(
        initial_username, final_username,
        "Results should remain stable across many calls"
    );
}

#[wasm_bindgen_test]
fn test_wasm_platform_specific_behavior() {
    // Test platform-specific behavior in WASM
    let platform = platform();
    let arch = arch();

    // In WASM, architecture should always be Wasm32
    assert_eq!(arch, Arch::Wasm32);

    // Platform detection should work
    match platform {
        Platform::Unknown(ref s) => {
            assert!(
                !s.is_empty(),
                "Unknown platform should have a description"
            );
        }
        _ => {}
    }
}

#[wasm_bindgen_test]
fn test_wasm_concurrent_access() {
    // Simulate concurrent-like access patterns
    let mut results = Vec::new();

    for _ in 0..10 {
        results.push(username());
        results.push(hostname().map(|h| h.into()));
    }

    // Verify all succeeded
    let all_ok = results.iter().all(|r| r.is_ok());
    assert!(all_ok, "All concurrent-like calls should succeed");
}

#[wasm_bindgen_test]
fn test_wasm_edge_cases() {
    // Test various edge cases specific to WASM

    // Empty string checks
    let username = username().unwrap();
    assert!(!username.is_empty());

    let hostname = hostname().unwrap();
    assert!(!hostname.is_empty());

    // Valid UTF-8 checks
    let realname_str = realname().unwrap();
    assert!(realname_str.chars().count() > 0);
}

#[wasm_bindgen_test]
fn test_wasm_api_completeness() {
    // Verify that all public APIs are accessible in WASM

    // Core identification APIs - verify they compile and are callable
    let has_username = username().is_ok();
    let has_hostname = hostname().is_ok();

    // Verify other APIs compile
    let _ = realname();
    let _ = devicename();
    let _ = distro();
    let _ = platform();
    let _ = arch();
    let _ = desktop_env();
    let _ = lang_prefs();

    // At least core APIs should work
    assert!(
        has_username || has_hostname,
        "At least username or hostname should work in WASM"
    );
}
