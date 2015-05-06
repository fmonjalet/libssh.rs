#![allow(missing_copy_implementations)]
#![allow(non_camel_case_types)]

extern crate libc;
use std::mem;

use native::libssh::*;

extern "C" {
    /*
    ssh_bind ssh_bind_new() [struct ssh_bind_struct *]
    */
    pub fn ssh_bind_new() -> *mut ssh_bind_struct;


    /*
    int ssh_bind_options_set()
        (ssh_bind) sshbind [struct ssh_bind_struct *]
        (enum ssh_bind_options_e) type [enum ssh_bind_options_e]
        (const void *) value
    */
    pub fn ssh_bind_options_set(sshbind: *mut ssh_bind_struct, type_: libc::c_uint, value: *const libc::c_void) -> libc::c_int;


    /*
    int ssh_bind_listen()
        (ssh_bind) ssh_bind_o [struct ssh_bind_struct *]
    */
    pub fn ssh_bind_listen(ssh_bind_o: *mut ssh_bind_struct) -> libc::c_int;


    /*
    int ssh_bind_set_callbacks()
        (ssh_bind) sshbind [struct ssh_bind_struct *]
        (ssh_bind_callbacks) callbacks [struct ssh_bind_callbacks_struct *]
        (void *) userdata
    */
    pub fn ssh_bind_set_callbacks(sshbind: *mut ssh_bind_struct, callbacks: *mut ssh_bind_callbacks_struct, userdata: *mut libc::c_void) -> libc::c_int;


    /*
    void ssh_bind_set_blocking()
        (ssh_bind) ssh_bind_o [struct ssh_bind_struct *]
        (int) blocking
    */
    pub fn ssh_bind_set_blocking(ssh_bind_o: *mut ssh_bind_struct, blocking: libc::c_int);


    /*
    socket_t ssh_bind_get_fd() [int]
        (ssh_bind) ssh_bind_o [struct ssh_bind_struct *]
    */
    pub fn ssh_bind_get_fd(ssh_bind_o: *mut ssh_bind_struct) -> libc::c_int;


    /*
    void ssh_bind_set_fd()
        (ssh_bind) ssh_bind_o [struct ssh_bind_struct *]
        (socket_t) fd [int]
    */
    pub fn ssh_bind_set_fd(ssh_bind_o: *mut ssh_bind_struct, fd: libc::c_int);


    /*
    void ssh_bind_fd_toaccept()
        (ssh_bind) ssh_bind_o [struct ssh_bind_struct *]
    */
    pub fn ssh_bind_fd_toaccept(ssh_bind_o: *mut ssh_bind_struct);


    /*
    int ssh_bind_accept()
        (ssh_bind) ssh_bind_o [struct ssh_bind_struct *]
        (ssh_session) session [struct ssh_session_struct *]
    */
    pub fn ssh_bind_accept(ssh_bind_o: *mut ssh_bind_struct, session: *mut ssh_session_struct) -> libc::c_int;


    /*
    int ssh_bind_accept_fd()
        (ssh_bind) ssh_bind_o [struct ssh_bind_struct *]
        (ssh_session) session [struct ssh_session_struct *]
        (socket_t) fd [int]
    */
    pub fn ssh_bind_accept_fd(ssh_bind_o: *mut ssh_bind_struct, session: *mut ssh_session_struct, fd: libc::c_int) -> libc::c_int;


    /*
    ssh_gssapi_creds ssh_gssapi_get_creds() [void *]
        (ssh_session) session [struct ssh_session_struct *]
    */
    pub fn ssh_gssapi_get_creds(session: *mut ssh_session_struct) -> *mut libc::c_void;


    /*
    int ssh_handle_key_exchange()
        (ssh_session) session [struct ssh_session_struct *]
    */
    pub fn ssh_handle_key_exchange(session: *mut ssh_session_struct) -> libc::c_int;


    /*
    void ssh_bind_free()
        (ssh_bind) ssh_bind_o [struct ssh_bind_struct *]
    */
    pub fn ssh_bind_free(ssh_bind_o: *mut ssh_bind_struct);


    /*
    void ssh_set_auth_methods()
        (ssh_session) session [struct ssh_session_struct *]
        (int) auth_methods
    */
    pub fn ssh_set_auth_methods(session: *mut ssh_session_struct, auth_methods: libc::c_int);


    /*
    int ssh_message_reply_default()
        (ssh_message) msg [struct ssh_message_struct *]
    */
    pub fn ssh_message_reply_default(msg: *mut ssh_message_struct) -> libc::c_int;


    /*
    const char * ssh_message_auth_user()
        (ssh_message) msg [struct ssh_message_struct *]
    */
    pub fn ssh_message_auth_user(msg: *mut ssh_message_struct) -> *const libc::c_char;


    /*
    const char * ssh_message_auth_password()
        (ssh_message) msg [struct ssh_message_struct *]
    */
    pub fn ssh_message_auth_password(msg: *mut ssh_message_struct) -> *const libc::c_char;


    /*
    ssh_key ssh_message_auth_pubkey() [struct ssh_key_struct *]
        (ssh_message) msg [struct ssh_message_struct *]
    */
    pub fn ssh_message_auth_pubkey(msg: *mut ssh_message_struct) -> *mut ssh_key_struct;


    /*
    int ssh_message_auth_kbdint_is_response()
        (ssh_message) msg [struct ssh_message_struct *]
    */
    pub fn ssh_message_auth_kbdint_is_response(msg: *mut ssh_message_struct) -> libc::c_int;


    /*
    enum ssh_publickey_state_e ssh_message_auth_publickey_state() [enum ssh_publickey_state_e]
        (ssh_message) msg [struct ssh_message_struct *]
    */
    pub fn ssh_message_auth_publickey_state(msg: *mut ssh_message_struct) -> libc::c_uint;


    /*
    int ssh_message_auth_reply_success()
        (ssh_message) msg [struct ssh_message_struct *]
        (int) partial
    */
    pub fn ssh_message_auth_reply_success(msg: *mut ssh_message_struct, partial: libc::c_int) -> libc::c_int;


    /*
    int ssh_message_auth_reply_pk_ok()
        (ssh_message) msg [struct ssh_message_struct *]
        (ssh_string) algo [struct ssh_string_struct *]
        (ssh_string) pubkey [struct ssh_string_struct *]
    */
    pub fn ssh_message_auth_reply_pk_ok(msg: *mut ssh_message_struct, algo: *mut ssh_string_struct, pubkey: *mut ssh_string_struct) -> libc::c_int;


    /*
    int ssh_message_auth_reply_pk_ok_simple()
        (ssh_message) msg [struct ssh_message_struct *]
    */
    pub fn ssh_message_auth_reply_pk_ok_simple(msg: *mut ssh_message_struct) -> libc::c_int;


    /*
    int ssh_message_auth_set_methods()
        (ssh_message) msg [struct ssh_message_struct *]
        (int) methods
    */
    pub fn ssh_message_auth_set_methods(msg: *mut ssh_message_struct, methods: libc::c_int) -> libc::c_int;


    /*
    int ssh_message_auth_interactive_request()
        (ssh_message) msg [struct ssh_message_struct *]
        (const char *) name
        (const char *) instruction
        (unsigned int) num_prompts
        (const char **) prompts
        (char *) echo
    */
    pub fn ssh_message_auth_interactive_request(msg: *mut ssh_message_struct, name: *const libc::c_char, instruction: *const libc::c_char, num_prompts: libc::c_uint, prompts: *mut *const libc::c_char, echo: *mut libc::c_char) -> libc::c_int;


    /*
    int ssh_message_service_reply_success()
        (ssh_message) msg [struct ssh_message_struct *]
    */
    pub fn ssh_message_service_reply_success(msg: *mut ssh_message_struct) -> libc::c_int;


    /*
    const char * ssh_message_service_service()
        (ssh_message) msg [struct ssh_message_struct *]
    */
    pub fn ssh_message_service_service(msg: *mut ssh_message_struct) -> *const libc::c_char;


    /*
    int ssh_message_global_request_reply_success()
        (ssh_message) msg [struct ssh_message_struct *]
        (uint16_t) bound_port [unsigned short]
    */
    pub fn ssh_message_global_request_reply_success(msg: *mut ssh_message_struct, bound_port: libc::c_ushort) -> libc::c_int;


    /*
    void ssh_set_message_callback()
        (ssh_session) session [struct ssh_session_struct *]
        (int (*)(ssh_session, ssh_message, void *)) ssh_bind_message_callback [int (*)(struct ssh_session_struct *, struct ssh_message_struct *, void *)]
        (void *) data
    */
    pub fn ssh_set_message_callback(session: *mut ssh_session_struct, ssh_bind_message_callback: Option<extern fn(*mut ssh_session_struct, *mut ssh_message_struct, *mut libc::c_void) -> libc::c_int>, data: *mut libc::c_void);


    /*
    int ssh_execute_message_callbacks()
        (ssh_session) session [struct ssh_session_struct *]
    */
    pub fn ssh_execute_message_callbacks(session: *mut ssh_session_struct) -> libc::c_int;


    /*
    const char * ssh_message_channel_request_open_originator()
        (ssh_message) msg [struct ssh_message_struct *]
    */
    pub fn ssh_message_channel_request_open_originator(msg: *mut ssh_message_struct) -> *const libc::c_char;


    /*
    int ssh_message_channel_request_open_originator_port()
        (ssh_message) msg [struct ssh_message_struct *]
    */
    pub fn ssh_message_channel_request_open_originator_port(msg: *mut ssh_message_struct) -> libc::c_int;


    /*
    const char * ssh_message_channel_request_open_destination()
        (ssh_message) msg [struct ssh_message_struct *]
    */
    pub fn ssh_message_channel_request_open_destination(msg: *mut ssh_message_struct) -> *const libc::c_char;


    /*
    int ssh_message_channel_request_open_destination_port()
        (ssh_message) msg [struct ssh_message_struct *]
    */
    pub fn ssh_message_channel_request_open_destination_port(msg: *mut ssh_message_struct) -> libc::c_int;


    /*
    ssh_channel ssh_message_channel_request_channel() [struct ssh_channel_struct *]
        (ssh_message) msg [struct ssh_message_struct *]
    */
    pub fn ssh_message_channel_request_channel(msg: *mut ssh_message_struct) -> *mut ssh_channel_struct;


    /*
    const char * ssh_message_channel_request_pty_term()
        (ssh_message) msg [struct ssh_message_struct *]
    */
    pub fn ssh_message_channel_request_pty_term(msg: *mut ssh_message_struct) -> *const libc::c_char;


    /*
    int ssh_message_channel_request_pty_width()
        (ssh_message) msg [struct ssh_message_struct *]
    */
    pub fn ssh_message_channel_request_pty_width(msg: *mut ssh_message_struct) -> libc::c_int;


    /*
    int ssh_message_channel_request_pty_height()
        (ssh_message) msg [struct ssh_message_struct *]
    */
    pub fn ssh_message_channel_request_pty_height(msg: *mut ssh_message_struct) -> libc::c_int;


    /*
    int ssh_message_channel_request_pty_pxwidth()
        (ssh_message) msg [struct ssh_message_struct *]
    */
    pub fn ssh_message_channel_request_pty_pxwidth(msg: *mut ssh_message_struct) -> libc::c_int;


    /*
    int ssh_message_channel_request_pty_pxheight()
        (ssh_message) msg [struct ssh_message_struct *]
    */
    pub fn ssh_message_channel_request_pty_pxheight(msg: *mut ssh_message_struct) -> libc::c_int;


    /*
    const char * ssh_message_channel_request_env_name()
        (ssh_message) msg [struct ssh_message_struct *]
    */
    pub fn ssh_message_channel_request_env_name(msg: *mut ssh_message_struct) -> *const libc::c_char;


    /*
    const char * ssh_message_channel_request_env_value()
        (ssh_message) msg [struct ssh_message_struct *]
    */
    pub fn ssh_message_channel_request_env_value(msg: *mut ssh_message_struct) -> *const libc::c_char;


    /*
    const char * ssh_message_channel_request_command()
        (ssh_message) msg [struct ssh_message_struct *]
    */
    pub fn ssh_message_channel_request_command(msg: *mut ssh_message_struct) -> *const libc::c_char;


    /*
    const char * ssh_message_channel_request_subsystem()
        (ssh_message) msg [struct ssh_message_struct *]
    */
    pub fn ssh_message_channel_request_subsystem(msg: *mut ssh_message_struct) -> *const libc::c_char;


    /*
    int ssh_message_channel_request_x11_single_connection()
        (ssh_message) msg [struct ssh_message_struct *]
    */
    pub fn ssh_message_channel_request_x11_single_connection(msg: *mut ssh_message_struct) -> libc::c_int;


    /*
    const char * ssh_message_channel_request_x11_auth_protocol()
        (ssh_message) msg [struct ssh_message_struct *]
    */
    pub fn ssh_message_channel_request_x11_auth_protocol(msg: *mut ssh_message_struct) -> *const libc::c_char;


    /*
    const char * ssh_message_channel_request_x11_auth_cookie()
        (ssh_message) msg [struct ssh_message_struct *]
    */
    pub fn ssh_message_channel_request_x11_auth_cookie(msg: *mut ssh_message_struct) -> *const libc::c_char;


    /*
    int ssh_message_channel_request_x11_screen_number()
        (ssh_message) msg [struct ssh_message_struct *]
    */
    pub fn ssh_message_channel_request_x11_screen_number(msg: *mut ssh_message_struct) -> libc::c_int;


    /*
    const char * ssh_message_global_request_address()
        (ssh_message) msg [struct ssh_message_struct *]
    */
    pub fn ssh_message_global_request_address(msg: *mut ssh_message_struct) -> *const libc::c_char;


    /*
    int ssh_message_global_request_port()
        (ssh_message) msg [struct ssh_message_struct *]
    */
    pub fn ssh_message_global_request_port(msg: *mut ssh_message_struct) -> libc::c_int;


    /*
    int ssh_channel_open_reverse_forward()
        (ssh_channel) channel [struct ssh_channel_struct *]
        (const char *) remotehost
        (int) remoteport
        (const char *) sourcehost
        (int) localport
    */
    pub fn ssh_channel_open_reverse_forward(channel: *mut ssh_channel_struct, remotehost: *const libc::c_char, remoteport: libc::c_int, sourcehost: *const libc::c_char, localport: libc::c_int) -> libc::c_int;


    /*
    int ssh_channel_request_send_exit_status()
        (ssh_channel) channel [struct ssh_channel_struct *]
        (int) exit_status
    */
    pub fn ssh_channel_request_send_exit_status(channel: *mut ssh_channel_struct, exit_status: libc::c_int) -> libc::c_int;


    /*
    int ssh_channel_request_send_exit_signal()
        (ssh_channel) channel [struct ssh_channel_struct *]
        (const char *) signum
        (int) core
        (const char *) errmsg
        (const char *) lang
    */
    pub fn ssh_channel_request_send_exit_signal(channel: *mut ssh_channel_struct, signum: *const libc::c_char, core: libc::c_int, errmsg: *const libc::c_char, lang: *const libc::c_char) -> libc::c_int;


    /*
    int ssh_channel_write_stderr()
        (ssh_channel) channel [struct ssh_channel_struct *]
        (const void *) data
        (uint32_t) len [unsigned int]
    */
    pub fn ssh_channel_write_stderr(channel: *mut ssh_channel_struct, data: *const libc::c_void, len: libc::c_uint) -> libc::c_int;


    /*
    int ssh_send_keepalive()
        (ssh_session) session [struct ssh_session_struct *]
    */
    pub fn ssh_send_keepalive(session: *mut ssh_session_struct) -> libc::c_int;


    /*
    int ssh_accept()
        (ssh_session) session [struct ssh_session_struct *]
    */
    pub fn ssh_accept(session: *mut ssh_session_struct) -> libc::c_int;


    /*
    int channel_write_stderr()
        (ssh_channel) channel [struct ssh_channel_struct *]
        (const void *) data
        (uint32_t) len [unsigned int]
    */
    pub fn channel_write_stderr(channel: *mut ssh_channel_struct, data: *const libc::c_void, len: libc::c_uint) -> libc::c_int;
}

/*
struct ssh_bind_struct
*/
#[repr(C)]
pub struct ssh_bind_struct;

/*
struct ssh_bind_callbacks_struct
        (size_t) size [unsigned long]
        (ssh_bind_incoming_connection_callback) incoming_connection [void (*)(struct ssh_bind_struct *, void *)]
*/
#[repr(C)]
pub struct ssh_bind_callbacks_struct {
    size: libc::c_ulong,
    incoming_connection: Option<extern fn(*mut ssh_bind_struct, *mut libc::c_void)>,
}

/*
enum ssh_bind_options_e {
    SSH_BIND_OPTIONS_BINDADDR =    0x00000000 (0)
    SSH_BIND_OPTIONS_BINDPORT =    0x00000001 (1)
    SSH_BIND_OPTIONS_BINDPORT_STR =    0x00000002 (2)
    SSH_BIND_OPTIONS_HOSTKEY =    0x00000003 (3)
    SSH_BIND_OPTIONS_DSAKEY =    0x00000004 (4)
    SSH_BIND_OPTIONS_RSAKEY =    0x00000005 (5)
    SSH_BIND_OPTIONS_BANNER =    0x00000006 (6)
    SSH_BIND_OPTIONS_LOG_VERBOSITY =    0x00000007 (7)
    SSH_BIND_OPTIONS_LOG_VERBOSITY_STR =    0x00000008 (8)
}
*/
#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(u32)]
pub enum ssh_bind_options_e {
    SSH_BIND_OPTIONS_BINDADDR =    0,
    SSH_BIND_OPTIONS_BINDPORT =    1,
    SSH_BIND_OPTIONS_BINDPORT_STR =    2,
    SSH_BIND_OPTIONS_HOSTKEY =    3,
    SSH_BIND_OPTIONS_DSAKEY =    4,
    SSH_BIND_OPTIONS_RSAKEY =    5,
    SSH_BIND_OPTIONS_BANNER =    6,
    SSH_BIND_OPTIONS_LOG_VERBOSITY =    7,
    SSH_BIND_OPTIONS_LOG_VERBOSITY_STR =    8,
}

impl ssh_bind_options_e {
    pub fn to_u32(&self) -> libc::c_uint {
        *self as libc::c_uint
    }

    pub fn from_u32(v: libc::c_uint) -> ssh_bind_options_e {
        unsafe { mem::transmute(v) }
    }
}

