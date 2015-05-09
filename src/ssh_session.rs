extern crate libc;
extern crate num;

use std::ffi::CString;
use num::FromPrimitive;
use std::mem;
use std::ptr;

use constants::{SSHAuthResult,SSHAuthMethod,SSHOption,SSHRequest};
use native::libssh;
use native::server;
use ssh_key::SSHKey;
use util;


pub struct SSHSession {
    _session: *mut libssh::ssh_session_struct
}

impl SSHSession {
    pub fn new(host: Option<&str>) -> Result<SSHSession, &'static str> {
        let ptr = unsafe { libssh::ssh_new() };
        assert!(!ptr.is_null());

        let session = SSHSession {_session: ptr};
        if host.is_some() {
            try!(session.set_host(host.unwrap()))
        }

        Ok(session)
    }

    pub fn set_host(&self, host: &str) -> Result<(), &'static str> {
        assert!(!self._session.is_null());

        let opt = SSHOption::Host as u32;
        let host_cstr = CString::new(host).unwrap();

        check_ssh_ok!(libssh::ssh_options_set(self._session, opt,
                                  host_cstr.as_ptr() as *const libc::c_void),
                      self._session)
    }

    pub fn connect<F>(&self, verify_public_key: F)
            -> Result<(), &'static str>
            where F: Fn(&SSHKey) -> bool
    {
        assert!(!self._session.is_null());

        try!(check_ssh_ok!(libssh::ssh_connect(self._session), self._session));

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
            libssh::ssh_disconnect(self._session);
        }
    }

    pub fn auth_by_public_key(&self, username: Option<&str>, pubkey: &SSHKey)
        -> Result<(),SSHAuthResult>
    {
        /*
            SSH_AUTH_ERROR: A serious error happened.
            SSH_AUTH_DENIED: The libssh doesn't accept that public key as an authentication token. Try another key or another method.
            SSH_AUTH_PARTIAL: You've been partially authenticated, you still have to use another method.
            SSH_AUTH_SUCCESS: The public key is accepted, you want now to use ssh_userauth_pubkey(). SSH_AUTH_AGAIN: In nonblocking mode, you've got to call this again later.
        */
        assert!(!self._session.is_null());

        let key = pubkey.raw();
        let func = |usr| unsafe {
            libssh::ssh_userauth_try_publickey(self._session, usr, key)
        };

        let ires = match username {
            Some(usrn_str)  => func(CString::new(usrn_str).unwrap().as_ptr()),
            None            => func(ptr::null())
        };

        let res = SSHAuthResult::from_i32(ires);
        match res {
            Some(SSHAuthResult::Success) => Ok(()),
            Some(err) => Err(err),
            None => {panic!("Unrecoverable result in auth_by_public_key");}
        }
    }

    pub fn raw(&self) -> *mut libssh::ssh_session_struct {
        assert!(!self._session.is_null());
        self._session
    }

    pub fn set_port(&self, port: &str) -> Result<(),&'static str> {
        assert!(!self._session.is_null());

        let opt = SSHOption::PortStr as u32;
        let p = CString::new(port).unwrap();
        check_ssh_ok!(
            libssh::ssh_options_set(self._session, opt,
                                    p.as_ptr() as *const libc::c_void),
            self._session
        )
    }

    pub fn auth_with_public_key<'a, F>(&self, verify_public_key: F)
            -> Result<(),&'a str>
            where F: Fn(&SSHKey) -> bool
    {
        const MAX_ATTEMPTS: libc::c_uint = 5;

        for _  in 0..MAX_ATTEMPTS {
            let msg = try!(SSHMessage::from_session(self));

            let type_ = msg.get_type();
            let subtype = SSHAuthMethod::from_i32(msg.get_subtype());

            match (type_, subtype) {
                (SSHRequest::Auth, Some(SSHAuthMethod::PublicKey)) =>
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

        let session: *mut libssh::ssh_session_struct = unsafe {
            mem::transmute(self._session)
        };
        check_ssh_ok!(server::ssh_handle_key_exchange(session))
    }

    pub fn get_message(&self) -> Result<SSHMessage, &'static str> {
        assert!(!self._session.is_null());

        let msg = try!(check_ssh_ptr!(libssh::ssh_message_get(self._session),
                                      self._session));
        Ok(SSHMessage { _msg: msg })
    }

    pub fn set_log_level(&self, level: i32) -> Result<(),&'static str> {
        assert!(!self._session.is_null());
        // FIXME: Should not be here?
        check_ssh_ok!(libssh::ssh_set_log_level(level), self._session)
    }
}

impl Drop for SSHSession {
    fn drop(&mut self) {
        unsafe {
            libssh::ssh_disconnect(self._session);
            libssh::ssh_free(self._session);
        }
    }
}


pub struct SSHMessage {
    _msg: *mut libssh::ssh_message_struct
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
        let session: *mut libssh::ssh_session_struct = unsafe {
            mem::transmute(session.raw())
        };
        assert!(!session.is_null());

        let msg = try!(check_ssh_ptr!(libssh::ssh_message_get(session)));
        Ok(SSHMessage { _msg: msg })
    }

    pub fn raw(self: &Self) -> *mut libssh::ssh_message_struct {
        self._msg
    }

    pub fn get_type(self: &Self) -> SSHRequest {
        assert!(!self._msg.is_null());

        let ityp = unsafe { libssh::ssh_message_type(self._msg) };
        SSHRequest::from_u32(ityp as u32).unwrap()
    }

    pub fn get_subtype(self: &Self) -> i32 {
        assert!(!self._msg.is_null());

        unsafe { libssh::ssh_message_subtype(self._msg) }
    }

    pub fn auth_set_methods(&self, methods: &[SSHAuthMethod])
                -> Result<(), &'static str> {
        // FIXME: dirty
        let method_mask = methods.iter()
                                 .fold(0i32,
                                       |mask, meth|
                                       mask | (meth.clone() as i32));
        let ret = unsafe {
            server::ssh_message_auth_set_methods(self._msg,
                                                 method_mask as libc::c_int)
        };
        match ret {
            libssh::SSH_OK => Ok(()),
            _      => Err("ssh_message_auth_set_methods() failed")
        }
    }

    pub fn reply_default(&self) -> Result<(), &'static str> {
        assert!(!self._msg.is_null());

        let res = unsafe { server::ssh_message_reply_default(self._msg) };
        match res {
            libssh::SSH_OK => Ok(()),
            _      => Err("ssh_message_reply_default() failed"),
        }
    }

    pub fn get_auth_user(&self) -> Result<String, &'static str> {
        let c_user = try!(check_ssh_ptr!(
                server::ssh_message_auth_user(self._msg)
        ));
        // FIXME: free?
        Ok(try!(util::from_native_str(c_user)).to_string())
    }

    pub fn get_auth_password(&self) -> Result<String, &'static str> {
        let c_pass = try!(check_ssh_ptr!(
                server::ssh_message_auth_password(self._msg)
        ));
        // FIXME: free?
        Ok(try!(util::from_native_str(c_pass)).to_string())
    }
}
