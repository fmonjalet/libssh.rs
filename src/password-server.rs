#[macro_use] extern crate log;
extern crate env_logger;

extern crate libssh;

use libssh::constants::{SSHRequest,SSHAuthMethod};
use libssh::native::libssh::{SSH_LOG_NOLOG};
use libssh::ssh_bind::SSHBind;
use libssh::ssh_session::{SSHSession,SSHMessage};

const HOST: &'static str = "127.0.0.1";
const SSH_LOG_LEVEL: i32 = SSH_LOG_NOLOG;

fn unhandled_preauth_req(msg: &SSHMessage) -> bool {
    let msg_type = msg.get_type();
    let msg_subtype = msg.get_subtype();
    println!("Message before auth: {:?}, {}", msg_type, msg_subtype);

    msg_type != SSHRequest::Auth
        || msg_subtype != SSHAuthMethod::Password as i32
}

fn ans_pass_auth(msg: &SSHMessage) {
    msg.auth_set_methods(&[SSHAuthMethod::Password]).ok();
    msg.reply_default().ok();
}

fn handle_session(session: &SSHSession) {
    loop {
        match session.get_message() {
            Ok(msg) => {
                if unhandled_preauth_req(&msg) {
                    ans_pass_auth(&msg);
                } else {
                    let user = msg.get_auth_user().unwrap();
                    let pass = msg.get_auth_password().unwrap();
                    println!("Authenticated user: {}", user);
                    println!("Password:           {}", pass);
                    ans_pass_auth(&msg);
                }
            },
            Err(err_msg) => {
                println!("End of session: {}", err_msg);
                break;
            }
        }
    }
    session.disconnect();
}

fn server() {
    let bind = SSHBind::new("./keys/id_rsa",
                            Some(HOST),
                            Some("2222")).unwrap();
    bind.set_log_level(SSH_LOG_LEVEL).unwrap();

    bind.listen().unwrap();
    println!("server: listening");

    loop {
        match bind.get_session() {
            Ok(session) => handle_session(&session),
            Err(err) => println!("Error while opening session {}", err)
        }
    }

}

fn main() {
    env_logger::init().unwrap();

    libssh::with_ssh(|| {
        server();
    })
}
