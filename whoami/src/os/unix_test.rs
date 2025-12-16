//! Mock FFI infrastructure for testing Unix platform implementations
//! 
//! This module provides mock implementations of C functions with error injection
//! capabilities to test error handling paths in a WASM environment.

#![cfg(test)]

use std::{
    cell::RefCell,
    ffi::{c_char, c_int, c_void},
    io::{Error, ErrorKind},
    ptr,
    sync::atomic::{AtomicBool, AtomicI32, AtomicU32, Ordering},
};

// Mock state for error injection
thread_local! {
    static MOCK_ERRNO: RefCell<Option<i32>> = const { RefCell::new(None) };
    static GETPWUID_R_FAIL: AtomicBool = const { AtomicBool::new(false) };
    static GETPWUID_R_NULL: AtomicBool = const { AtomicBool::new(false) };
    static GETEUID_VALUE: AtomicU32 = const { AtomicU32::new(1000) };
    static GETHOSTNAME_FAIL: AtomicBool = const { AtomicBool::new(false) };
    static UNAME_FAIL: AtomicBool = const { AtomicBool::new(false) };
}

/// Set errno value for next syscall
pub fn set_mock_errno(errno: i32) {
    MOCK_ERRNO.with(|e| *e.borrow_mut() = Some(errno));
}

/// Clear mock errno
pub fn clear_mock_errno() {
    MOCK_ERRNO.with(|e| *e.borrow_mut() = None);
}

/// Get current mock errno and clear it
fn get_and_clear_errno() -> Option<i32> {
    MOCK_ERRNO.with(|e| e.borrow_mut().take())
}

/// Set mock to make getpwuid_r fail with errno
pub fn set_getpwuid_r_fail(fail: bool) {
    GETPWUID_R_FAIL.with(|f| f.store(fail, Ordering::SeqCst));
}

/// Set mock to make getpwuid_r return null pointer
pub fn set_getpwuid_r_null(null: bool) {
    GETPWUID_R_NULL.with(|n| n.store(null, Ordering::SeqCst));
}

/// Set mock euid value
pub fn set_mock_euid(uid: u32) {
    GETEUID_VALUE.with(|u| u.store(uid, Ordering::SeqCst));
}

/// Set mock to make gethostname fail
pub fn set_gethostname_fail(fail: bool) {
    GETHOSTNAME_FAIL.with(|f| f.store(fail, Ordering::SeqCst));
}

/// Set mock to make uname fail
pub fn set_uname_fail(fail: bool) {
    UNAME_FAIL.with(|f| f.store(fail, Ordering::SeqCst));
}

/// Reset all mock states to default
pub fn reset_mocks() {
    clear_mock_errno();
    set_getpwuid_r_fail(false);
    set_getpwuid_r_null(false);
    set_mock_euid(1000);
    set_gethostname_fail(false);
    set_uname_fail(false);
}

// Mock PassWd structure (matching the real one from unix.rs)
#[repr(C)]
#[cfg(target_os = "linux")]
pub struct MockPassWd {
    pw_name: *const c_void,
    pw_passwd: *const c_void,
    pw_uid: u32,
    pw_gid: u32,
    pw_gecos: *const c_void,
    pw_dir: *const c_void,
    pw_shell: *const c_void,
}

// Mock implementations of FFI functions

/// Mock getpwuid_r for Linux/macOS/BSD
#[no_mangle]
#[cfg(any(
    target_os = "linux",
    target_os = "macos",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd",
    target_os = "hurd",
))]
pub extern "C" fn mock_getpwuid_r(
    _uid: u32,
    pwd: *mut c_void,
    buf: *mut c_void,
    _buflen: usize,
    result: *mut *mut c_void,
) -> i32 {
    if GETPWUID_R_FAIL.with(|f| f.load(Ordering::SeqCst)) {
        if let Some(errno) = get_and_clear_errno() {
            return errno;
        }
        return libc::ENOENT; // Default error: No such user
    }

    if GETPWUID_R_NULL.with(|n| n.load(Ordering::SeqCst)) {
        unsafe { *result = ptr::null_mut() };
        return 0;
    }

    // Fill in mock data
    unsafe {
        let pwd_ptr = pwd as *mut MockPassWd;
        let buf_ptr = buf as *mut u8;
        
        // Write mock username to buffer
        let username = b"testuser\0";
        ptr::copy_nonoverlapping(username.as_ptr(), buf_ptr, username.len());
        (*pwd_ptr).pw_name = buf_ptr as *const c_void;
        
        // Write mock real name to buffer (offset by username length)
        let realname = b"Test User\0";
        let realname_offset = username.len();
        ptr::copy_nonoverlapping(
            realname.as_ptr(),
            buf_ptr.add(realname_offset),
            realname.len(),
        );
        (*pwd_ptr).pw_gecos = buf_ptr.add(realname_offset) as *const c_void;
        
        (*pwd_ptr).pw_uid = 1000;
        (*pwd_ptr).pw_gid = 1000;
        
        *result = pwd as *mut c_void;
    }
    
    0 // Success
}

/// Mock getpwuid_r for illumos
#[no_mangle]
#[cfg(target_os = "illumos")]
pub extern "C" fn mock_getpwuid_r_illumos(
    _uid: u32,
    pwd: *mut c_void,
    buf: *mut c_void,
    _buflen: c_int,
) -> *mut c_void {
    if GETPWUID_R_FAIL.with(|f| f.load(Ordering::SeqCst)) {
        return ptr::null_mut();
    }

    // Similar mock data filling as above
    unsafe {
        let pwd_ptr = pwd as *mut MockPassWd;
        let buf_ptr = buf as *mut u8;
        
        let username = b"testuser\0";
        ptr::copy_nonoverlapping(username.as_ptr(), buf_ptr, username.len());
        (*pwd_ptr).pw_name = buf_ptr as *const c_void;
        
        let realname = b"Test User\0";
        let realname_offset = username.len();
        ptr::copy_nonoverlapping(
            realname.as_ptr(),
            buf_ptr.add(realname_offset),
            realname.len(),
        );
        (*pwd_ptr).pw_gecos = buf_ptr.add(realname_offset) as *const c_void;
        
        (*pwd_ptr).pw_uid = 1000;
        (*pwd_ptr).pw_gid = 1000;
    }
    
    pwd
}

/// Mock geteuid
#[no_mangle]
pub extern "C" fn mock_geteuid() -> u32 {
    GETEUID_VALUE.with(|u| u.load(Ordering::SeqCst))
}

/// Mock gethostname
#[no_mangle]
pub extern "C" fn mock_gethostname(name: *mut c_void, len: usize) -> i32 {
    if GETHOSTNAME_FAIL.with(|f| f.load(Ordering::SeqCst)) {
        if let Some(errno) = get_and_clear_errno() {
            return errno;
        }
        return -1;
    }

    let hostname = b"test-hostname\0";
    if len < hostname.len() {
        return -1; // ENAMETOOLONG
    }

    unsafe {
        ptr::copy_nonoverlapping(hostname.as_ptr(), name as *mut u8, hostname.len());
    }
    
    0 // Success
}

/// Mock UtsName structure
#[repr(C)]
pub struct MockUtsName {
    pub sysname: [c_char; 65],
    pub nodename: [c_char; 65],
    pub release: [c_char; 65],
    pub version: [c_char; 65],
    pub machine: [c_char; 65],
}

/// Mock uname
#[no_mangle]
pub extern "C" fn mock_uname(buf: *mut MockUtsName) -> c_int {
    if UNAME_FAIL.with(|f| f.load(Ordering::SeqCst)) {
        if let Some(errno) = get_and_clear_errno() {
            return errno;
        }
        return -1;
    }

    unsafe {
        // Fill with mock system information
        let sysname = b"Linux\0";
        let nodename = b"test-node\0";
        let release = b"5.10.0\0";
        let version = b"#1 SMP\0";
        let machine = b"x86_64\0";

        (*buf).sysname[..sysname.len()].copy_from_slice(
            std::slice::from_raw_parts(sysname.as_ptr() as *const c_char, sysname.len())
        );
        (*buf).nodename[..nodename.len()].copy_from_slice(
            std::slice::from_raw_parts(nodename.as_ptr() as *const c_char, nodename.len())
        );
        (*buf).release[..release.len()].copy_from_slice(
            std::slice::from_raw_parts(release.as_ptr() as *const c_char, release.len())
        );
        (*buf).version[..version.len()].copy_from_slice(
            std::slice::from_raw_parts(version.as_ptr() as *const c_char, version.len())
        );
        (*buf).machine[..machine.len()].copy_from_slice(
            std::slice::from_raw_parts(machine.as_ptr() as *const c_char, machine.len())
        );
    }

    0 // Success
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_errno() {
        reset_mocks();
        assert!(get_and_clear_errno().is_none());
        
        set_mock_errno(42);
        assert_eq!(get_and_clear_errno(), Some(42));
        assert!(get_and_clear_errno().is_none());
    }

    #[test]
    fn test_mock_geteuid() {
        reset_mocks();
        assert_eq!(mock_geteuid(), 1000);
        
        set_mock_euid(2000);
        assert_eq!(mock_geteuid(), 2000);
    }

    #[test]
    fn test_mock_gethostname_success() {
        reset_mocks();
        let mut buf = [0u8; 256];
        let result = mock_gethostname(buf.as_mut_ptr() as *mut c_void, buf.len());
        assert_eq!(result, 0);
        assert_eq!(&buf[..13], b"test-hostname");
    }

    #[test]
    fn test_mock_gethostname_fail() {
        reset_mocks();
        set_gethostname_fail(true);
        
        let mut buf = [0u8; 256];
        let result = mock_gethostname(buf.as_mut_ptr() as *mut c_void, buf.len());
        assert_eq!(result, -1);
    }

    #[test]
    fn test_mock_uname_success() {
        reset_mocks();
        let mut buf = MockUtsName {
            sysname: [0; 65],
            nodename: [0; 65],
            release: [0; 65],
            version: [0; 65],
            machine: [0; 65],
        };
        
        let result = mock_uname(&mut buf);
        assert_eq!(result, 0);
        
        // Verify sysname
        let sysname = unsafe {
            std::ffi::CStr::from_ptr(buf.sysname.as_ptr())
        };
        assert_eq!(sysname.to_bytes(), b"Linux");
    }

    #[test]
    fn test_mock_uname_fail() {
        reset_mocks();
        set_uname_fail(true);
        
        let mut buf = MockUtsName {
            sysname: [0; 65],
            nodename: [0; 65],
            release: [0; 65],
            version: [0; 65],
            machine: [0; 65],
        };
        
        let result = mock_uname(&mut buf);
        assert_eq!(result, -1);
    }
}