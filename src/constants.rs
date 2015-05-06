
enum_from_primitive! {
    #[derive(Copy, Clone, PartialEq, Debug)]
    #[repr(i32)]
    pub enum SSHAuthMethod {
        Unknown     = 0,
        None        = 1,
        Password    = 2,
        PublicKey   = 4,
        HostBased   = 8,
        Interactive = 16,
        GssApiMic   = 32,
    }
}


enum_from_primitive! {
    #[derive(Copy, Clone, PartialEq, Debug)]
    #[repr(u32)]
    pub enum SSHOption {
        Host                        = 0,
        Port                        = 1,
        PortStr                     = 2,
        Fd                          = 3,
        User                        = 4,
        SshDir                      = 5,
        Identity                    = 6,
        AddIdentity                 = 7,
        Knownhosts                  = 8,
        Timeout                     = 9,
        TimeoutUsec                 = 10,
        Ssh1                        = 11,
        Ssh2                        = 12,
        LogVerbosity                = 13,
        LogVerbosityStr             = 14,
        CiphersCS                   = 15,
        CiphersSC                   = 16,
        CompressionCS               = 17,
        CompressionSC               = 18,
        Proxycommand                = 19,
        Bindaddr                    = 20,
        Stricthostkeycheck          = 21,
        Compression                 = 22,
        CompressionLevel            = 23,
        KeyExchange                 = 24,
        Hostkeys                    = 25,
        GssapiServerIdentity        = 26,
        GssapiClientIdentity        = 27,
        GssapiDelegateCredentials   = 28,
    }
}

enum_from_primitive! {
    #[derive(Copy, Clone, PartialEq, Debug)]
    #[repr(i32)]
    pub enum SSHAuthResult {
        Success = 0,
        Denied  = 1,
        Partial = 2,
        Info    = 3,
        Again   = 4,
        Error   = -1,
    }
}

enum_from_primitive! {
    #[derive(Copy, Clone, PartialEq, Debug)]
    #[repr(u32)]
    pub enum SSHRequest {
        Auth        = 1,
        ChannelOpen = 2,
        Channel     = 3,
        Service     = 4,
        Global      = 5,
    }
}
