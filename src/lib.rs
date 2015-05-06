#[macro_use] extern crate log;
#[macro_use] extern crate enum_primitive;
extern crate num;

use std::sync::{Once, ONCE_INIT};

#[macro_use] mod util;
pub mod constants;
pub mod native;
pub mod ssh_key;
pub mod ssh_session;
pub mod ssh_bind;

static SSH_INIT: Once = ONCE_INIT;

pub fn ssh_init() {
    //check_ssh_ok!(1);
    SSH_INIT.call_once(|| {
        unsafe { native::libssh::ssh_init() };
    })
}

pub fn ssh_finalize() {
    debug!("calling ssh_finalize().");
    unsafe { native::libssh::ssh_finalize() };
}

pub struct SSHFinalizer;
impl Drop for SSHFinalizer {
    fn drop(&mut self) {
        ssh_finalize();
    }
}

pub fn with_ssh<F: Fn()>(func: F) {
    ssh_init();

    let finalizer = SSHFinalizer;
    func();
    drop(finalizer);
}
