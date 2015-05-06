extern crate libc;

use std::ffi::CStr;
use std::str::from_utf8;

use native::libssh;


pub fn from_native_str<'a>(native_str: *const libc::c_char)
            -> Result<&'a str, &'static str> {
    if native_str.is_null() {
        return Err("Could not convert NULL native string");
    }
    unsafe {
        match from_utf8(CStr::from_ptr(native_str).to_bytes()) {
            Ok(s) => Ok(s),
            Err(_) => Err("Could not convert native string to String")
        }
    }
}

pub fn ssh_get_error(ptr: *mut libc::c_void)
            -> Result<&'static str, &'static str> {
    let err = unsafe {
        libssh::ssh_get_error(ptr)
    };
    if err.is_null() {
        Err("Unknown SSH error")
    } else {
        from_native_str(err)
    }
}

/*
trait SSHErrorMessage {
    fn get_error(&self) -> &'static str;
}
*/

#[macro_export]
macro_rules! ssh_err_msg {
    ($ptr: expr, $cause: expr) => {
        match $crate::util::ssh_get_error($ptr as *mut libc::c_void) {
            Ok(err_str) => Err(err_str),
            Err(_) => Err(concat!(stringify!($cause),
                              " failed (could not get libssh error)"))
        }
    }

}


macro_rules! _check_ssh_ok {
    ($e: expr, $error: expr) => {
        match unsafe { $e } {
            libssh::SSH_OK => Ok(()),
            _      => {$error}
        }
    }
}

/// Checks that the return value of a given function is SSH_OK.
/// unsafe expr(, *mut ptr) -> Result<(), &str>
#[macro_export]
macro_rules! check_ssh_ok {
    ($e: expr) => {
        _check_ssh_ok!($e, concat!(stringify!($e), " failed"))
    };
    ($e: expr, $ptr: expr) => {
        _check_ssh_ok!($e, ssh_err_msg!($ptr, $e))
    }
}

