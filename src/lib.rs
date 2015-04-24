#[macro_use] extern crate log;

use std::sync::{Once, ONCE_INIT};

pub mod libssh_server;
pub mod libssh;
pub mod ssh_key;
pub mod ssh_session;
pub mod ssh_bind;
pub mod ssh_message;

static SSH_INIT: Once = ONCE_INIT;

pub fn ssh_init() {
	SSH_INIT.call_once(|| {
		unsafe { libssh::ssh_init() };
	})
}

pub fn ssh_finalize() {
	debug!("calling ssh_finalize().");
	unsafe { libssh::ssh_finalize() };
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
