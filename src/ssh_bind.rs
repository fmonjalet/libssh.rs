extern crate libc;

use std::mem;
use std::ffi::CString;

use constants;
use native::server;
use native::libssh;
use ssh_session::SSHSession;

pub struct SSHBind {
    _bind: *mut server::ssh_bind_struct,
}

impl SSHBind {
    pub fn new(private_key_file: &str, host: Option<&str>, port: Option<&str>)
        -> Result<SSHBind, &'static str>
    {
        let ptr = unsafe { server::ssh_bind_new() };
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

        let opt = constants::SSHBindOption::BindAddr as u32;
        let h = CString::new(host).unwrap();
        check_ssh_ok!(
            server::ssh_bind_options_set(self._bind, opt,
                                         h.as_ptr() as *const libc::c_void),
            self._bind
        )
    }

    pub fn set_port(&self, port: &str) -> Result<(),&'static str> {
        assert!(!self._bind.is_null());

        let opt = constants::SSHBindOption::BindPortStr as u32;
        let p = CString::new(port).unwrap();
        check_ssh_ok!(
            server::ssh_bind_options_set(self._bind, opt,
                                         p.as_ptr() as *const libc::c_void),
            self._bind
        )
    }

    pub fn set_private_key_file(&self, key_file: &str) -> Result<(),&'static str> {
        assert!(!self._bind.is_null());

        let opt_type = constants::SSHBindOption::HostKey as u32;
        let typ = CString::new("ssh-rsa").unwrap();
        try!(check_ssh_ok!(
            server::ssh_bind_options_set(self._bind, opt_type,
                                         typ.as_ptr() as *const libc::c_void),
            self._bind
        ));

        let opt_key = constants::SSHBindOption::RSAKey as u32;
        let pkey_file = CString::new(key_file).unwrap();
        check_ssh_ok!(
            server::ssh_bind_options_set(self._bind, opt_key,
                                         pkey_file.as_ptr() as *const libc::c_void),
            self._bind
        )
    }

    pub fn listen(&self) -> Result<(),&'static str> {
        assert!(!self._bind.is_null());
        check_ssh_ok!(server::ssh_bind_listen(self._bind), self._bind)
    }

    pub fn accept(&self) -> Result<SSHSession, &'static str> {
        assert!(!self._bind.is_null());
        let session = try!(SSHSession::new(None));
        try!(check_ssh_ok!(
            server::ssh_bind_accept(self._bind, mem::transmute(session.raw())),
            self._bind
        ));
        Ok(session)
    }

    /* Returns a ready session */
    pub fn get_session(&self) -> Result<SSHSession, &'static str> {
        assert!(!self._bind.is_null());
        let session = try!(self.accept());
        try!(check_ssh_ok!(server::ssh_handle_key_exchange(session.raw())));
        Ok(session)
    }

    pub fn set_log_level(&self, level: i32) -> Result<(),&'static str> {
        assert!(!self._bind.is_null());
        // FIXME: Should not be here?
        check_ssh_ok!(libssh::ssh_set_log_level(level), self._bind)
    }
}

impl Iterator for SSHBind {
    type Item = SSHSession;
    fn next(&mut self) -> Option<SSHSession> {
        let session = self.accept().unwrap();
        session.handle_key_exchange().unwrap();
        Some(session)
    }
}
