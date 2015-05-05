extern crate libc;

use std::ffi::CStr;
use std::str::from_utf8;

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

macro_rules! _check_ssh_ok {
    ($e: expr, $error: block) => {
        match unsafe { $e } {
            SSH_OK => Ok(()),
            _      => {$error}
        }
    }
}

/* unsafe expr -> Result<(), &str> */
#[macro_export]
macro_rules! check_ssh_ok {
    ($e: expr) => {
        _check_ssh_ok!($e, concat!(stringify!($e), " failed"))
    };
    ($e: expr, $ptr: expr) => {
        _check_ssh_ok!($e,
           {
                let err = unsafe {
                    $crate::libssh::ssh_get_error($ptr as *mut c_void)
                };
                let tomatch = if err.is_null() {
                    Err("")
                } else {
                    $crate::util::from_native_str(err)
                };
                match tomatch {
                    Ok(err_str) => Err(err_str),
                    Err(_) => Err(concat!(stringify!($e),
                                      " failed (could not get libssh error)"))
                }
            }
        )
    }
}
