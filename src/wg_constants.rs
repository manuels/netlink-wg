pub const WG_KEY_LEN: usize = 32;

enum_from_primitive! {
    #[derive(Debug, PartialEq)]
    pub enum WgCmd {
        GetDevice,
        SetDevice,
    }
}

enum_from_primitive! {
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum WgDeviceAttribute {
        Unspec,
        IfIndex,
        IfName,
        PrivateKey,
        PublicKey,
        Flags,
        ListenPort,
        FwMark,
        Peers,
    }
}

enum_from_primitive! {
    #[derive(Debug, PartialEq)]
    pub enum WgDeviceFlag {
    	ReplacePeers = 1 << 0
    }
}

enum_from_primitive! {
    #[derive(Debug, PartialEq)]
    pub enum WgPeerFlag {
        RemoveMe = 1 << 0,
        ReplaceAllowedIps = 1 << 1
    }
}

enum_from_primitive! {
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum WgPeerAttribute {
        Unspec,
        PublicKey,
        PresharedKey,
        Flags,
        Endpoint,
        PersistentKeepaliveInterval,
        LastHandshakeTime,
        RxBytes,
        TxBytes,
        AllowedIps,
        ProtocolVersion,
    }
}

enum_from_primitive! {
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum WgAllowedIpAttribute {
        Unspec,
        Family,
        IpAddr,
        CidrMask,
    }
}
