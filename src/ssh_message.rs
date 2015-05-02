#![allow(unused_imports)]

extern crate libc;
extern crate num;

use libssh_server::*;
use libssh;
use ssh_key;
use ssh_session::SSHSession;

use std::mem;
use std::ptr;
use std::str::from_utf8;
use self::libc::types::common::c95::c_void;
use std::ffi::CStr;

use self::num::FromPrimitive;

pub struct SSHMessage {
    _msg: *mut ssh_message_struct
}

impl Drop for SSHMessage {
    fn drop(&mut self) {
        /*
         * not necessary: issues "double free()" panic
        unsafe {
            ssh_message_free(self._msg)
        }*/
    }
}
impl SSHMessage {
    pub fn from_session(session: &SSHSession) -> Result<SSHMessage, &'static str> {
        let session: *mut ssh_session_struct = unsafe {
            mem::transmute(session.raw())
        };
        assert!(!session.is_null());

        let msg = unsafe { ssh_message_get(session) };
        if msg.is_null() {
            Err("ssh_message_get() returned NULL")
        }
        else {
            Ok(SSHMessage { _msg: msg })
        }
    }

    pub fn raw(self: &Self) -> *mut ssh_message_struct {
        self._msg
    }

    pub fn get_type(self: &Self) -> ssh_requests_e {
        assert!(!self._msg.is_null());

        let ityp = unsafe { ssh_message_type(self._msg) };
        ssh_requests_e::from_u32(ityp as u32)
    }

    pub fn get_subtype(self: &Self) -> i32 {
        assert!(!self._msg.is_null());

        unsafe { ssh_message_subtype(self._msg) }
    }

    pub fn auth_set_methods(&self, method_mask: i32) -> Result<(), &'static str> {
        let ret = unsafe {
            ssh_message_auth_set_methods(self._msg, method_mask as libc::c_int)
        };
        match ret {
            SSH_OK => Ok(()),
            _      => Err("ssh_message_auth_set_methods() failed")
        }
    }

    pub fn reply_default(&self) -> Result<(), &'static str> {
        assert!(!self._msg.is_null());

        let res = unsafe { ssh_message_reply_default(self._msg) };
        match res {
            SSH_OK => Ok(()),
            _      => Err("ssh_message_reply_default() failed"),
        }
    }

    pub fn get_auth_user(&self) -> Result<String, &'static str> {
        let c_user = unsafe { ssh_message_auth_user(self._msg) };
        if c_user.is_null() {
            Err("ssh_message_auth_user() failed")
        } else {
            let user = unsafe {
                from_utf8(CStr::from_ptr(c_user).to_bytes()).ok().unwrap()
                                                                 .to_string()
            };
            Ok(user)
        }
    }

    pub fn get_auth_password(&self) -> Result<String, &'static str> {
        let c_pass = unsafe { ssh_message_auth_password(self._msg) };
        if c_pass.is_null() {
            Err("ssh_message_auth_password() failed")
        } else {
            let pass = unsafe {
                from_utf8(CStr::from_ptr(c_pass).to_bytes()).ok().unwrap()
                                                                 .to_string()
            };
            Ok(pass)
        }
    }
}
