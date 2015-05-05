#![allow(unused_imports)]
#![allow(missing_copy_implementations)]

extern crate libc;

use libssh_server::*;
use ssh_key;
use ssh_session::SSHSession;
use ssh_message;

use std::mem;
use std::ptr;
use self::libc::types::common::c95::c_void;
use std::ffi::CString;

pub struct SSHBind {
    _bind: *mut ssh_bind_struct
}

impl SSHBind {
    pub fn new(private_key_file: &str, host: Option<&str>, port: Option<&str>)
        -> Result<SSHBind, &'static str>
    {
        let ptr = unsafe { ssh_bind_new() };
        assert!(!ptr.is_null());

        let bind = SSHBind { _bind: ptr };
        
        if host.is_some() {
            try!(bind.set_host(host.unwrap()));
        }
        try!(bind.set_port(port.unwrap_or("22")));

        try!(bind.set_private_key_file(private_key_file));

        Ok(bind)
    }

    pub fn set_host(&self, host: &str) -> Result<(),&'static str> {
        assert!(!self._bind.is_null());

        let opt = ssh_bind_options_e::SSH_BIND_OPTIONS_BINDADDR as u32;
        let h = CString::new(host).unwrap();
        check_ssh_ok!(
            ssh_bind_options_set(self._bind, opt, h.as_ptr() as *const c_void),
            self._bind
        )
    }

    pub fn set_port(&self, port: &str) -> Result<(),&'static str> {
        assert!(!self._bind.is_null());

        let opt = ssh_bind_options_e::SSH_BIND_OPTIONS_BINDPORT_STR as u32;
        let p = CString::new(port).unwrap();
        check_ssh_ok!(
            ssh_bind_options_set(self._bind, opt, p.as_ptr() as *const c_void),
            self._bind
        )
    }

    pub fn set_private_key_file(&self, key_file: &str) -> Result<(),&'static str> {
        assert!(!self._bind.is_null());

        let opt_type = ssh_bind_options_e::SSH_BIND_OPTIONS_HOSTKEY as u32;
        let typ = CString::new("ssh-rsa").unwrap();
        try!(check_ssh_ok!(
            ssh_bind_options_set(self._bind, opt_type,
                                 typ.as_ptr() as *const c_void),
            self._bind
        ));

        let opt_key = ssh_bind_options_e::SSH_BIND_OPTIONS_RSAKEY as u32;
        let pkey_file = CString::new(key_file).unwrap();
        check_ssh_ok!(
            ssh_bind_options_set(self._bind, opt_key,
                                 pkey_file.as_ptr() as *const c_void),
            self._bind
        )
    }

    /*
    pub fn wait_for_session(&self) -> Result<SSHSession, String> {
    }
    */

    pub fn listen(&self) -> Result<(),&'static str> {
        assert!(!self._bind.is_null());

        check_ssh_ok!(ssh_bind_listen(self._bind), self._bind)
    }

    pub fn accept(&self, session: &SSHSession) -> Result<(),&'static str> {
        assert!(!self._bind.is_null());
        check_ssh_ok!(
            ssh_bind_accept(self._bind, mem::transmute(session.raw())),
            self._bind
        )
    }

    pub fn set_log_level(&self, level: i32) -> Result<(),&'static str> {
        assert!(!self._bind.is_null());
        // FIXME: Should not be here?
        check_ssh_ok!(ssh_set_log_level(level), self._bind)
    }
}
