#[macro_use] extern crate log;
extern crate env_logger;

extern crate libssh;

use libssh::{ssh_session,ssh_bind,ssh_message,constants};
use libssh::libssh_server::ssh_requests_e;

const HOST: &'static str = "127.0.0.1";
const SSH_LOG_LEVEL: i32 = libssh::libssh::SSH_LOG_NOLOG;

fn unhandled_preauth_req(msg: &ssh_message::SSHMessage) -> bool {
    let msg_type = msg.get_type();
    let msg_subtype = msg.get_subtype();
    println!("Message before auth: {:?}, {}", msg_type, msg_subtype);
    msg_type != ssh_requests_e::SSH_REQUEST_AUTH
        || msg_subtype != constants::SSH_AUTH_METHOD_PASSWORD
}

fn ans_pass_auth(msg: &ssh_message::SSHMessage) {
    msg.auth_set_methods(
        constants::SSH_AUTH_METHOD_PASSWORD).ok();
    msg.reply_default().ok();
}

fn server() {
    let session = ssh_session::SSHSession::new(None).unwrap();
    let bind = ssh_bind::SSHBind::new("./keys/id_rsa",
                                      Some(HOST),
                                      Some("2222")).unwrap();
    bind.set_log_level(SSH_LOG_LEVEL).unwrap();

    bind.listen().unwrap();
    println!("server: listening");

    bind.accept(&session).unwrap();
    println!("server: accepted");

    session.handle_key_exchange().unwrap();
    println!("server: handle_key_exchange() done");

    loop {
        match ssh_message::SSHMessage::from_session(&session) {
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
                println!("Exiting because of error: {}", err_msg);
                break;
            }
        }
    }

    session.disconnect();
}

fn main() {
    env_logger::init().unwrap();

    libssh::with_ssh(|| {
        server();
    })
}
