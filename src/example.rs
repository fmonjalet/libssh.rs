extern crate libssh;

use std::sync::{Arc, Barrier};
use std::thread::spawn;

use libssh::ssh_key::SSHKey;
use libssh::ssh_session::SSHSession;
use libssh::ssh_bind::SSHBind;
use libssh::native::libssh::{SSH_LOG_NOLOG};

const PRIVATE_KEY1: &'static str = "-----BEGIN RSA PRIVATE KEY-----
MIIEpgIBAAKCAQEAtVRiaUPBXiqVNw4By07q+nqDAfIKzuo2Nrdm2TbNMaZzcbY4
dYOwr4fj4FcwRx3PkDDTZsDw83GRUqoyk/wbz+TsgRe40S2AlhtJvH+77vcAIoSb
UMCjdBcQxdu+b6nZKJ9kMh+nus6y9/syx3prci6PPcRhyYRjV8UfjgXVYGyXMWgs
rGs5y/Y228ugiwEfgUF+DInmzlnVDbDqzx+nEY09aVPcnckTH9eBddwNN7YCDcY8
/Ag8+ASbuNo7O7zr2i5/n811UTio6ED2GiP0rjgNuW7bYOgiGsHouWoFDVpSn94s
pxA3t9wO+6WCF2px/6AfxM2SNJwy1lewcqBV4QIDAQABAoIBAQCA2TKIzC2WZTnc
giaCOlS8odt/wWcuurzFSrNZfBh4xGdaEPqzfl1JjY0+d5YFoshAFIHTjRxqUHPM
QsZn44g7xNbNsHaSpPuvLjrKKBX56yf8XzAiRJChSFaR0eDTZeS6efBvsZC1LHV9
wtDFcFbzLuR4Jpi54knZL2iJudlyhuSn8avvvNrqAvbxXmBlnAXACKLWLr7lT0VC
kfQHcYxMwKUTJzq2G2RAXH2Vdr6HiaZgvlZfq2CkQaFIqya1LOaG2LocDW2MhHSe
Ojy5L/zlKkRk923/vNxha9zOyr+MOHkDuY767RzQhmVBVtfZzEgiM0PQSdjVShWX
0CHr9hgBAoGBAOIKaLupK04KHQ4zxTVIDjaeW2M83///hiJoXOH8MGGJ3h9skbgJ
ughWmjBohpLJFvIKNs14cwjPi8tDtfkK3QTL/6ACjl8QuN2YlCzY1avltQIPPiou
W53HRrNwhZysdLgH06o/CX6OvW0NJnqTt58q4uXvkyfDEybjLSo7I7khAoGBAM1c
7HoJcNITL5B5mWvh/fqolQyHyMo/4yiLqRsr52rgYGGkiY4lJXPLmtBJFNS7vw/0
7wrnYt6XI/TMTZtIVVovX7jWi+bZh9/VhwxKljFnGav6msAONooscE5PWpiYQ8fs
QsOZx/UcG0kz1QQMNGpWVBM019sA+YgLbq4l5UTBAoGBAINsOeiiOyNsjegsAYUx
F9J5z/iq9DILhxmKRDbAQgDz/8mVfkPao+clMxDiNRwy/rxLZAGi/n8o7MaJ38uk
nUykr0OBOPXc6x8sDzrj95eyPsOrySENQwdBTcIWshidzF8TbeWWMRb8Nvaopq6u
JBzO+o1l9dEwgnohq6jaKbMBAoGBAJrgFpOepRA5Wei6XAMph1JPa0Ds8nfdIKKG
WT1dqgRHPUjGPtsNlqYyignE48nf4aLWFKUDhePa1koa/fg63+vIyIbsfsvViAw9
y8BwS77sQ0cZEzX+QhGInBXi8K8ePhf7TQqY4l0vGkDlryODVNBRVMy7UIMgxA9e
l9UMTVDBAoGBAK7W+4IKkC2tMVEUNoZV6JlSp+WQROKWvyvtv01MiDtJYcfNMFL8
ikwcwZIsiVeoAm6m5J1wKxAdpkz/JDR+x20SJrnFeITAMGaUsqf6JP4SqyazD+0C
7Spmt4KQ/ybYFHnyVelZMs/QiU5eNZGXVzY3RWze7pyZDg1RVYeztOKf
-----END RSA PRIVATE KEY-----
";
const HOST: &'static str = "127.0.0.1";
const SSH_LOG_LEVEL: i32 = SSH_LOG_NOLOG;

fn client(barrier: &Arc<Barrier>) {
    let priv_key = SSHKey::private_key_from_base64(PRIVATE_KEY1).unwrap();
    assert!(priv_key.is_public());

    let session = SSHSession::new(Some(HOST)).unwrap();
    session.set_log_level(SSH_LOG_LEVEL).unwrap();
    session.set_port("2222").unwrap();

    println!("client: waiting for server...");
    barrier.wait();
    
    println!("client: connecting...");
    session.connect(|_remote_public_key| {
        return true;
    }).unwrap();
    println!("client: connected to {}", HOST);

    let _res = session.auth_by_public_key(None, &priv_key);
    println!("client: authenticated");
    session.disconnect();
}

fn server(barrier: &Arc<Barrier>) {
    let bind = SSHBind::new("./keys/id_rsa", Some(HOST), Some("2222")).unwrap();
    bind.set_log_level(SSH_LOG_LEVEL).unwrap();

    bind.listen().unwrap();
    println!("server: listening");
    barrier.wait();

    let session = bind.accept().unwrap();
    println!("server: accepted");

    session.handle_key_exchange().unwrap();
    println!("server: handle_key_exchange() done");

    let _res = session.auth_with_public_key(|_remote_public_key| {
        println!("server: client pubkey");
        true
    }).unwrap();
    println!("yay!");

    session.disconnect();
}

fn main() {
    let barrier = Arc::new(Barrier::new(2));

    libssh::with_ssh(|| {
        let b = barrier.clone();
        spawn(move || { client(&b) });
        server(&barrier);
    })
}
