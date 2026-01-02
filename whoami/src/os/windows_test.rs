//! Mock FFI infrastructure for testing Windows platform implementations
//! 
//! This module provides mock implementations of Windows API functions with error
//! injection capabilities to test error handling paths in a WASM environment.

#![cfg(test)]

use std::{
    cell::RefCell,
    ffi::{c_char, c_int, c_uchar, c_ulong, c_ushort},
    ptr,
    sync::atomic::{AtomicBool, AtomicI32, Ordering},
};

// Windows error codes
pub const ERR_INSUFFICIENT_BUFFER: i32 = 0x7A;
pub const ERR_NONE_MAPPED: i32 = 0x534;
pub const ERROR_SUCCESS: i32 = 0;

// Mock state for error injection
thread_local! {
    static GET_USERNAME_FAIL: AtomicBool = const { AtomicBool::new(false) };
    static GET_USERNAME_EX_FAIL: AtomicBool = const { AtomicBool::new(false) };
    static GET_COMPUTER_NAME_FAIL: AtomicBool = const { AtomicBool::new(false) };
    static MOCK_LAST_ERROR: AtomicI32 = const { AtomicI32::new(ERROR_SUCCESS) };
    static USERNAME_SIZE: RefCell<usize> = const { RefCell::new(32) };
}

/// Set mock to make GetUserNameW fail
pub fn set_get_username_fail(fail: bool) {
    GET_USERNAME_FAIL.with(|f| f.store(fail, Ordering::SeqCst));
}

/// Set mock to make GetUserNameExW fail
pub fn set_get_username_ex_fail(fail: bool) {
    GET_USERNAME_EX_FAIL.with(|f| f.store(fail, Ordering::SeqCst));
}

/// Set mock to make GetComputerNameExW fail
pub fn set_get_computer_name_fail(fail: bool) {
    GET_COMPUTER_NAME_FAIL.with(|f| f.store(fail, Ordering::SeqCst));
}

/// Set mock last error value
pub fn set_mock_last_error(error: i32) {
    MOCK_LAST_ERROR.with(|e| e.store(error, Ordering::SeqCst));
}

/// Get mock last error value
pub fn get_mock_last_error() -> i32 {
    MOCK_LAST_ERROR.with(|e| e.load(Ordering::SeqCst))
}

/// Set mock username buffer size
pub fn set_username_size(size: usize) {
    USERNAME_SIZE.with(|s| *s.borrow_mut() = size);
}

/// Reset all mock states to default
pub fn reset_mocks() {
    set_get_username_fail(false);
    set_get_username_ex_fail(false);
    set_get_computer_name_fail(false);
    set_mock_last_error(ERROR_SUCCESS);
    set_username_size(32);
}

// Extended name format enum (matching Windows API)
#[repr(i32)]
#[derive(Debug, Clone, Copy)]
pub enum ExtendedNameFormat {
    NameDisplay = 3,
}

// Computer name format enum
#[repr(i32)]
#[derive(Debug, Clone, Copy)]
pub enum ComputerNameFormat {
    NetBIOS = 0,
    DnsHostname = 1,
    DnsDomain = 2,
    DnsFullyQualified = 3,
}

/// Mock GetUserNameW
#[no_mangle]
pub extern "system" fn mock_GetUserNameW(
    lpbuffer: *mut c_char,
    pcbbuffer: *mut c_ulong,
) -> c_int {
    unsafe {
        if lpbuffer.is_null() || pcbbuffer.is_null() {
            set_mock_last_error(0x57); // ERROR_INVALID_PARAMETER
            return 0;
        }

        let size = USERNAME_SIZE.with(|s| *s.borrow());
        
        // First call: return required buffer size
        if lpbuffer as usize == 0 || (*pcbbuffer as usize) < size {
            *pcbbuffer = size as c_ulong;
            set_mock_last_error(ERR_INSUFFICIENT_BUFFER);
            return 0;
        }

        if GET_USERNAME_FAIL.with(|f| f.load(Ordering::SeqCst)) {
            set_mock_last_error(0x5); // ERROR_ACCESS_DENIED
            return 0;
        }

        // Write mock username as UTF-16
        let username = "testuser\0";
        let utf16: Vec<u16> = username.encode_utf16().collect();
        
        let dest = lpbuffer as *mut u16;
        for (i, &ch) in utf16.iter().enumerate() {
            *dest.add(i) = ch;
        }
        
        *pcbbuffer = utf16.len() as c_ulong;
        set_mock_last_error(ERROR_SUCCESS);
        1 // Success
    }
}

/// Mock GetUserNameExW
#[no_mangle]
pub extern "system" fn mock_GetUserNameExW(
    name_format: ExtendedNameFormat,
    lpbuffer: *mut c_char,
    nsize: *mut c_ulong,
) -> c_uchar {
    unsafe {
        if lpbuffer.is_null() && !nsize.is_null() {
            // First call: return required buffer size
            *nsize = 64;
            set_mock_last_error(ERR_INSUFFICIENT_BUFFER);
            return 0;
        }

        if GET_USERNAME_EX_FAIL.with(|f| f.load(Ordering::SeqCst)) {
            set_mock_last_error(ERR_NONE_MAPPED);
            return 0;
        }

        // Write mock display name as UTF-16
        let display_name = match name_format {
            ExtendedNameFormat::NameDisplay => "Test User\0",
        };
        
        let utf16: Vec<u16> = display_name.encode_utf16().collect();
        
        if !lpbuffer.is_null() && !nsize.is_null() {
            let dest = lpbuffer as *mut u16;
            for (i, &ch) in utf16.iter().enumerate() {
                *dest.add(i) = ch;
            }
            *nsize = utf16.len() as c_ulong;
        }
        
        set_mock_last_error(ERROR_SUCCESS);
        1 // Success
    }
}

/// Mock GetComputerNameExW
#[no_mangle]
pub extern "system" fn mock_GetComputerNameExW(
    name_type: ComputerNameFormat,
    lpbuffer: *mut c_char,
    nsize: *mut c_ulong,
) -> c_int {
    unsafe {
        if lpbuffer.is_null() && !nsize.is_null() {
            // First call: return required buffer size
            *nsize = 64;
            set_mock_last_error(ERR_INSUFFICIENT_BUFFER);
            return 0;
        }

        if GET_COMPUTER_NAME_FAIL.with(|f| f.load(Ordering::SeqCst)) {
            set_mock_last_error(0x5); // ERROR_ACCESS_DENIED
            return 0;
        }

        // Write mock computer name as UTF-16
        let computer_name = match name_type {
            ComputerNameFormat::NetBIOS => "TESTPC\0",
            ComputerNameFormat::DnsHostname => "testpc\0",
            ComputerNameFormat::DnsDomain => "example.com\0",
            ComputerNameFormat::DnsFullyQualified => "testpc.example.com\0",
        };
        
        let utf16: Vec<u16> = computer_name.encode_utf16().collect();
        
        if !lpbuffer.is_null() && !nsize.is_null() {
            let dest = lpbuffer as *mut u16;
            for (i, &ch) in utf16.iter().enumerate() {
                *dest.add(i) = ch;
            }
            *nsize = utf16.len() as c_ulong;
        }
        
        set_mock_last_error(ERROR_SUCCESS);
        1 // Success
    }
}

/// Mock GetUserPreferredUILanguages
#[no_mangle]
pub extern "system" fn mock_GetUserPreferredUILanguages(
    _dw_flags: c_ulong,
    pul_num_languages: *mut c_ulong,
    pwsz_languages_buffer: *mut u16,
    pcch_languages_buffer: *mut c_ulong,
) -> c_int {
    unsafe {
        if !pul_num_languages.is_null() {
            *pul_num_languages = 1;
        }

        // Mock language: "en-US\0\0"
        let languages = "en-US\0\0";
        let utf16: Vec<u16> = languages.encode_utf16().collect();

        if pwsz_languages_buffer.is_null() && !pcch_languages_buffer.is_null() {
            // Return required buffer size
            *pcch_languages_buffer = utf16.len() as c_ulong;
            set_mock_last_error(ERR_INSUFFICIENT_BUFFER);
            return 0;
        }

        if !pwsz_languages_buffer.is_null() && !pcch_languages_buffer.is_null() {
            for (i, &ch) in utf16.iter().enumerate() {
                *pwsz_languages_buffer.add(i) = ch;
            }
            *pcch_languages_buffer = utf16.len() as c_ulong;
        }

        set_mock_last_error(ERROR_SUCCESS);
        1 // Success
    }
}

/// Mock SystemInfo structure
#[repr(C)]
pub struct MockSystemInfo {
    pub processor_architecture: c_ushort,
    pub reserved: c_ushort,
    pub page_size: c_ulong,
    // ... other fields omitted for brevity
}

/// Mock GetNativeSystemInfo
#[no_mangle]
pub extern "system" fn mock_GetNativeSystemInfo(system_info: *mut MockSystemInfo) {
    unsafe {
        if !system_info.is_null() {
            (*system_info).processor_architecture = 9; // PROCESSOR_ARCHITECTURE_AMD64
            (*system_info).reserved = 0;
            (*system_info).page_size = 4096;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_get_username_success() {
        reset_mocks();
        
        let mut size = 0;
        // First call to get size
        let result = mock_GetUserNameW(ptr::null_mut(), &mut size);
        assert_eq!(result, 0);
        assert_eq!(get_mock_last_error(), ERR_INSUFFICIENT_BUFFER);
        assert!(size > 0);

        // Second call with buffer
        let mut buffer: Vec<u16> = vec![0; size as usize];
        let mut actual_size = size;
        let result = mock_GetUserNameW(buffer.as_mut_ptr() as *mut c_char, &mut actual_size);
        assert_eq!(result, 1);
        assert_eq!(get_mock_last_error(), ERROR_SUCCESS);
    }

    #[test]
    fn test_mock_get_username_fail() {
        reset_mocks();
        set_get_username_fail(true);
        
        let mut size = 32;
        let mut buffer: Vec<u16> = vec![0; size as usize];
        let result = mock_GetUserNameW(buffer.as_mut_ptr() as *mut c_char, &mut size);
        assert_eq!(result, 0);
        assert_eq!(get_mock_last_error(), 0x5); // ERROR_ACCESS_DENIED
    }

    #[test]
    fn test_mock_get_username_ex_success() {
        reset_mocks();
        
        let mut size = 0;
        // First call to get size
        let result = mock_GetUserNameExW(
            ExtendedNameFormat::NameDisplay,
            ptr::null_mut(),
            &mut size,
        );
        assert_eq!(result, 0);
        assert!(size > 0);

        // Second call with buffer
        let mut buffer: Vec<u16> = vec![0; size as usize];
        let result = mock_GetUserNameExW(
            ExtendedNameFormat::NameDisplay,
            buffer.as_mut_ptr() as *mut c_char,
            &mut size,
        );
        assert_eq!(result, 1);
    }

    #[test]
    fn test_mock_get_username_ex_fail() {
        reset_mocks();
        set_get_username_ex_fail(true);
        
        let mut size = 64;
        let mut buffer: Vec<u16> = vec![0; size as usize];
        let result = mock_GetUserNameExW(
            ExtendedNameFormat::NameDisplay,
            buffer.as_mut_ptr() as *mut c_char,
            &mut size,
        );
        assert_eq!(result, 0);
        assert_eq!(get_mock_last_error(), ERR_NONE_MAPPED);
    }

    #[test]
    fn test_mock_get_computer_name_success() {
        reset_mocks();
        
        let mut size = 0;
        let result = mock_GetComputerNameExW(
            ComputerNameFormat::DnsHostname,
            ptr::null_mut(),
            &mut size,
        );
        assert_eq!(result, 0);
        assert!(size > 0);

        let mut buffer: Vec<u16> = vec![0; size as usize];
        let result = mock_GetComputerNameExW(
            ComputerNameFormat::DnsHostname,
            buffer.as_mut_ptr() as *mut c_char,
            &mut size,
        );
        assert_eq!(result, 1);
    }

    #[test]
    fn test_mock_last_error_tracking() {
        reset_mocks();
        
        assert_eq!(get_mock_last_error(), ERROR_SUCCESS);
        
        set_mock_last_error(ERR_INSUFFICIENT_BUFFER);
        assert_eq!(get_mock_last_error(), ERR_INSUFFICIENT_BUFFER);
        
        set_mock_last_error(ERROR_SUCCESS);
        assert_eq!(get_mock_last_error(), ERROR_SUCCESS);
    }

    #[test]
    fn test_reset_mocks() {
        set_get_username_fail(true);
        set_mock_last_error(999);
        
        reset_mocks();
        
        assert_eq!(GET_USERNAME_FAIL.with(|f| f.load(Ordering::SeqCst)), false);
        assert_eq!(get_mock_last_error(), ERROR_SUCCESS);
    }
}