#![allow(unused_imports)]

extern crate libc;

use libssh::*;
use libssh_server;
use ssh_key::SSHKey;
use ssh_message::SSHMessage;

use std::mem;
use std::ptr;
use std::str::from_utf8;
use self::libc::types::common::c95::c_void;
use self::libc::types::os::arch::c95::c_uint;
use std::ffi::{CString, CStr};

pub struct SSHSession {
    _session: *mut ssh_session_struct
}

impl SSHSession {
    pub fn new(host: Option<&str>) -> Result<SSHSession, &'static str> {
        let ptr = unsafe { ssh_new() };
        assert!(!ptr.is_null());

        let session = SSHSession {_session: ptr};
        if host.is_some() {
            try!(session.set_host(host.unwrap()))
        }

        Ok(session)
    }

    pub fn set_host(&self, host: &str) -> Result<(), &'static str> {
        assert!(!self._session.is_null());

        let opt = ssh_options_e::SSH_OPTIONS_HOST as u32;
        let host_cstr = CString::new(host).unwrap();

        check_ssh_ok!(ssh_options_set(self._session, opt,
                            host_cstr.as_ptr() as *const c_void),
                      self._session)
    }

    pub fn connect<F>(&self, verify_public_key: F)
            -> Result<(), &'static str>
            where F: Fn(&SSHKey) -> bool
    {
        assert!(!self._session.is_null());

        try!(check_ssh_ok!(ssh_connect(self._session), self._session));

        let remote_public_key = try!(
            SSHKey::from_session(self).map_err(|err| err)
        );
        if !verify_public_key(&remote_public_key) {
            self.disconnect();
            return Err("authentication failed");
        }
        else {
            Ok(())
        }
    }

    pub fn disconnect(&self) {
        assert!(!self._session.is_null());

        unsafe {
            ssh_disconnect(self._session);
        }
    }

    pub fn auth_by_public_key(&self, username: Option<&str>, pubkey: &SSHKey)
        -> Result<(),ssh_auth_e>
    {
        /*
            SSH_AUTH_ERROR: A serious error happened.
            SSH_AUTH_DENIED: The server doesn't accept that public key as an authentication token. Try another key or another method.
            SSH_AUTH_PARTIAL: You've been partially authenticated, you still have to use another method.
            SSH_AUTH_SUCCESS: The public key is accepted, you want now to use ssh_userauth_pubkey(). SSH_AUTH_AGAIN: In nonblocking mode, you've got to call this again later.
        */
        assert!(!self._session.is_null());

        let key = pubkey.raw();
        let func = |usr| unsafe {
            ssh_userauth_try_publickey(self._session, usr, key)
        };

        let ires = match username {
            Option::Some(usrn_str)  =>
                    func(CString::new(usrn_str).unwrap().as_ptr()),
            Option::None            => func(ptr::null())
        };

        let res = ssh_auth_e::from_i32(ires);
        match res {
            ssh_auth_e::SSH_AUTH_SUCCESS => Ok(()),
            ssh_auth_e::SSH_AUTH_PARTIAL |
            ssh_auth_e::SSH_AUTH_DENIED |
            ssh_auth_e::SSH_AUTH_AGAIN |
            ssh_auth_e::SSH_AUTH_ERROR => Err(res),
            x => {panic!("{:?}", x);}
        }
    }

    pub fn raw(&self) -> *mut ssh_session_struct {
        assert!(!self._session.is_null());
        self._session
    }

    pub fn set_port(&self, port: &str) -> Result<(),&'static str> {
        assert!(!self._session.is_null());

        let opt = ssh_options_e::SSH_OPTIONS_PORT_STR as u32;
        let p = CString::new(port).unwrap();
        check_ssh_ok!(
            ssh_options_set(self._session, opt, p.as_ptr() as *const c_void),
            self._session
        )
    }

    pub fn auth_with_public_key<'a, F>(&self, verify_public_key: F)
            -> Result<(),&'a str>
            where F: Fn(&SSHKey) -> bool
    {
        const MAX_ATTEMPTS: c_uint = 5;

        for _  in 0..MAX_ATTEMPTS {
            let msg = try!(SSHMessage::from_session(self));

            let type_ = msg.get_type();
            let subtype = msg.get_subtype();

            match (type_, subtype) {
                (libssh_server::ssh_requests_e::SSH_REQUEST_AUTH,
                        libssh_server::SSH_AUTH_METHOD_PUBLICKEY) =>
                {
                    let remote_public_key = try!(SSHKey::from_message(&msg));
                    
                    if verify_public_key(&remote_public_key) {
                        return Ok(());
                    }
                },

                _ => {
                    try!(msg.reply_default())
                }
            }
        }
        Err("authentication with public key failed")
    }

    pub fn handle_key_exchange(&self) -> Result<(),&'static str> {
        assert!(!self._session.is_null());

        let session: *mut libssh_server::ssh_session_struct = unsafe {
            mem::transmute(self._session)
        };
        check_ssh_ok!(
            libssh_server::ssh_handle_key_exchange(session),
            self._session
        )
    }

    pub fn set_log_level(&self, level: i32) -> Result<(),&'static str> {
        assert!(!self._session.is_null());
        // FIXME: Should not be here?
        check_ssh_ok!(ssh_set_log_level(level), self._session)
    }
}

impl Drop for SSHSession {
    fn drop(&mut self) {
        unsafe {
            ssh_disconnect(self._session);
            ssh_free(self._session);
        }
    }
}
