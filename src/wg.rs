use std::io::Cursor;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::time::Duration;
use std::time::SystemTime;

use byteorder::NativeEndian;
use byteorder::ReadBytesExt;
use enum_primitive::FromPrimitive;

use crate::Family;
use crate::NlSocket;
use crate::wg_constants::*;
use crate::generic::__subtypes::__subtypes::__subtypes::PeerAttr;
use crate::generic::__subtypes::__subtypes::Peer as WgPeer;

pub enum IfaceBy {
    Index(u32),
    Name(String),
}

impl NlSocket {
    pub async fn get_family(&mut self, name: &'static str) -> std::io::Result<Option<Family>> {
        let ctx = &crate::generic::Context {
            wireguard_family: 0xffff,
            wireguard_version: 0xff,
        };

        let name = std::ffi::CString::new(name).unwrap().as_bytes_with_nul().to_vec();
        let mut req = crate::generic::NetlinkGeneric {
            header: crate::generic::__subtypes::Nlmsghdr {
                len: 36,
                typ: libc::GENL_ID_CTRL as _,
                flags: (libc::NLM_F_REQUEST | libc::NLM_F_ACK) as _,
                pid: 0,
                seq: 0,
            },
            generic: Some( crate::generic::__subtypes::Genl {
                cmd: libc::CTRL_CMD_GETFAMILY as _,
                version: 1,
                padding: 0,
            }),
            success: None,
            error: None,
            wg_attributes: None,
            attributes: Some(vec![
                crate::generic::__subtypes::Attr {
                    len: 14,
                    typ: libc::CTRL_ATTR_FAMILY_NAME as _,
                    value: name,
                    padding: vec![]
                }
            ]),
        };

        let resp = await!(self.request_generic(&mut req, ctx))?.pop().unwrap();

        let mut id = None;
        let mut version = None;

        let attrs = resp.attributes.unwrap();
        attrs.into_iter().for_each(|a| {
            if a.typ == libc::CTRL_ATTR_FAMILY_ID as _ {
                let mut c = Cursor::new(a.value);
                let v = c.read_u16::<NativeEndian>().unwrap();
                id = Some(v);
            } else if a.typ == libc::CTRL_ATTR_VERSION as _ {
                let mut c = Cursor::new(a.value);
                let v = c.read_u16::<NativeEndian>().unwrap();
                version = Some(v);
            };
        });

        return match (id, version) {
            (Some(id), Some(version)) => Ok(Some(Family { id,
                version: version as _
            })),
            _ => Ok(None),
        };
    }

    pub async fn set_wg_endpoint<'a>(&'a mut self,
            family: &'a Family,
            iface: IfaceBy,
            peer: &'a [u8],
            endpoint: SocketAddr)
        -> std::io::Result<()>
    {
        use crate::wg_constants::*;

        let ctx = &crate::generic::Context {
            wireguard_family: family.id,
            wireguard_version: family.version,
        };

        assert_eq!(peer.len(), WG_KEY_LEN);

        let by_iface = match iface {
            IfaceBy::Name(ifname) => {
                let ifname = std::ffi::CString::new(ifname).unwrap().as_bytes_with_nul().to_vec();

                crate::generic::__subtypes::WgAttr {
                    typ: WgDeviceAttribute::IfName as _,
                    ifname: Some(ifname),
                    ..Default::default()
                }
            },
            IfaceBy::Index(ifindex) => {
                crate::generic::__subtypes::WgAttr {
                    typ: WgDeviceAttribute::IfIndex as _,
                    ifindex: Some(ifindex),
                    ..Default::default()
                }
            },
        };
        
        let endpoint = nix::sys::socket::InetAddr::from_std(&endpoint);
        let endpoint = match endpoint {
            nix::sys::socket::InetAddr::V4(s) => {
                unsafe {
                    std::mem::transmute_copy::<_, [u8; 16]>(&s)
                }.to_vec()
            },
            nix::sys::socket::InetAddr::V6(s) => {
                unsafe {
                    std::mem::transmute_copy::<_, [u8; 28]>(&s)
                }.to_vec()
            },
        };
  
        let mut req = crate::generic::NetlinkGeneric {
            header: crate::generic::__subtypes::Nlmsghdr {
                len: 32,
                typ: family.id as _,
                flags: (libc::NLM_F_REQUEST | libc::NLM_F_ACK) as _,
                pid: 0,
                seq: 0,
            },
            generic: Some( crate::generic::__subtypes::Genl {
                cmd: WgCmd::SetDevice as _,
                version: family.version,
                padding: 0,
            }),
            success: None,
            error: None,
            attributes: None,
            wg_attributes: Some(vec![
                by_iface,
                crate::generic::__subtypes::WgAttr {
                    typ: WgDeviceAttribute::Flags as _,
                    flags: Some(0),
                    ..Default::default()
                },
                crate::generic::__subtypes::WgAttr {
                    typ: WgDeviceAttribute::Peers as _,
                    peers: Some(vec![
                        WgPeer {
                            len: 0,
                            typ: 0,
                            peer: vec![
                                PeerAttr {
                                    typ: WgPeerAttribute::PublicKey as _,
                                    public_key: Some(peer.to_vec()),
                                    ..Default::default()
                                },
                                PeerAttr {
                                    typ: WgPeerAttribute::Flags as _,
                                    flags: Some(0),
                                    ..Default::default()
                                },
                                PeerAttr {
                                    typ: WgPeerAttribute::Endpoint as _,
                                    endpoint: Some(endpoint),
                                    ..Default::default()
                                },
                            ],
                            padding: vec![],
                        }
                    ]),
                    ..Default::default()
                }
            ]),
        };
        req.pad();

        let _ = await!(self.request_generic(&mut req, ctx))?;
        Ok(())
    }

    pub async fn get_wg_device<'a>(&'a mut self, family: &'a Family, iface: IfaceBy) -> std::io::Result<IfConfig> {
        use crate::wg_constants::*;

        let ctx = &crate::generic::Context {
            wireguard_family: family.id,
            wireguard_version: family.version,
        };

        let by_iface = match iface {
            IfaceBy::Name(ifname) => {
                let ifname = std::ffi::CString::new(ifname).unwrap().as_bytes_with_nul().to_vec();

                crate::generic::__subtypes::WgAttr {
                    typ: WgDeviceAttribute::IfName as _,
                    ifname: Some(ifname),
                    ..Default::default()
                }
            },
            IfaceBy::Index(ifindex) => {
                crate::generic::__subtypes::WgAttr {
                    typ: WgDeviceAttribute::IfIndex as _,
                    ifindex: Some(ifindex),
                    ..Default::default()
                }
            },
        };
        
        let mut req = crate::generic::NetlinkGeneric {
            header: crate::generic::__subtypes::Nlmsghdr {
                len: 32,
                typ: family.id as _,
                flags: (libc::NLM_F_REQUEST | libc::NLM_F_DUMP | libc::NLM_F_ACK) as _,
                pid: 0,
                seq: 0,
            },
            generic: Some( crate::generic::__subtypes::Genl {
                cmd: WgCmd::GetDevice as _,
                version: family.version,
                padding: 0,
            }),
            success: None,
            error: None,
            attributes: None,
            wg_attributes: Some(vec![by_iface]),
        };

        let resp = await!(self.request_generic(&mut req, ctx))?;

        let mut cfg = None;
        for msg in resp {
            if let crate::generic::NetlinkGeneric { wg_attributes: Some(attrs), .. } = msg {
                cfg = if let Some(_cfg) = cfg {
                    unimplemented!("multiple messages not supported yet");
                } else {
                    Some(IfConfig::parse(attrs))
                };
            } else {
                unreachable!()
            }
        }

        Ok(cfg.unwrap())
    }
}

#[derive(Debug)]
pub struct IfConfig {
    ifname: String,
    ifindex: u32,
    private_key: [u8; WG_KEY_LEN],
    public_key: [u8; WG_KEY_LEN],
    fwmark: u32,
    listen_port: u16,
    peers: Vec<Peer>,
}

#[derive(Debug)]
pub struct Peer {
    public_key: [u8; WG_KEY_LEN],
    preshared_key: Option<[u8; WG_KEY_LEN]>,
    endpoint: Option<SocketAddr>,
    persistent_keepalive_interval: Option<Duration>,
    last_handshake_time: Option<SystemTime>,
    rx_bytes: usize,
    tx_bytes: usize,
    allowed_ips: Option<()>,
    protocol_version: u32
}

impl IfConfig {
    pub fn listen_port(&self) -> u16 {
        self.listen_port
    }

    pub fn private_key<'a>(&'a self) -> &'a [u8; WG_KEY_LEN] {
        &self.private_key
    }

    pub fn public_key<'a>(&'a self) -> &'a [u8; WG_KEY_LEN] {
        &self.public_key
    }

    pub fn peers(&self) -> impl Iterator<Item=&Peer> {
        self.peers.iter()
    }

    fn parse(attrs: Vec<crate::generic::__subtypes::WgAttr>) -> IfConfig {
        let mut map: HashMap<_, _> = attrs.into_iter()
            .map(|a| (WgDeviceAttribute::from_u16(a.typ), a)).collect();
        
        let mut private_key = [0; WG_KEY_LEN];
        let mut public_key = [0; WG_KEY_LEN];
        private_key.clone_from_slice(&map.remove(&Some(WgDeviceAttribute::PrivateKey)).unwrap().private_key.unwrap());
        public_key.clone_from_slice(&map.remove(&Some(WgDeviceAttribute::PublicKey)).unwrap().public_key.unwrap());

        let ifname = map.remove(&Some(WgDeviceAttribute::IfName)).unwrap().ifname.unwrap();
        let ifname = std::ffi::CStr::from_bytes_with_nul(&ifname[..]).unwrap();
        let ifname = ifname.to_str().unwrap();

        let peers = map.remove(&Some(WgDeviceAttribute::Peers)).unwrap().peers.unwrap();
        let peers = peers.into_iter().map(|p| Peer::parse(p.peer)).collect();

        IfConfig {
            ifname: ifname.to_string(),
            ifindex: map.remove(&Some(WgDeviceAttribute::IfIndex)).unwrap().ifindex.unwrap(),
            private_key,
            public_key,
            fwmark: map.remove(&Some(WgDeviceAttribute::FwMark)).unwrap().fwmark.unwrap(),
            listen_port: map.remove(&Some(WgDeviceAttribute::ListenPort)).unwrap().listen_port.unwrap(),
            peers,
        }
    }
}

impl Peer {
    pub fn public_key<'a>(&'a self) -> &'a [u8; WG_KEY_LEN] {
        &self.public_key
    }

    pub fn parse(attrs: Vec<PeerAttr>) -> Peer {
        let mut map: HashMap<Option<WgPeerAttribute>, PeerAttr> = attrs.into_iter()
            .map(|a| (WgPeerAttribute::from_u16(a.typ), a)).collect();
        
        let mut public_key = [0; WG_KEY_LEN];
        let mut preshared_key = [0; WG_KEY_LEN];
        public_key.clone_from_slice(&map.remove(&Some(WgPeerAttribute::PublicKey)).unwrap().public_key.unwrap());
        preshared_key.clone_from_slice(&map.remove(&Some(WgPeerAttribute::PresharedKey)).unwrap().preshared_key.unwrap());

        let preshared_key = if preshared_key == [0; WG_KEY_LEN] {
            None
        } else {
            Some(preshared_key)
        };

        let keepalive = map.remove(&Some(WgPeerAttribute::PersistentKeepaliveInterval)).unwrap().persistent_keepalive_interval.unwrap();
        let keepalive = if keepalive == 0 {None} else {Some(Duration::from_secs(keepalive.into()))};

        let last_handshake = map.remove(&Some(WgPeerAttribute::LastHandshakeTime)).unwrap().last_handshake_time.unwrap();
        let last_handshake: libc::timespec = unsafe { std::mem::transmute(&last_handshake[..]) };
        let last_handshake = Duration::from_secs(last_handshake.tv_sec as _) + Duration::from_nanos(last_handshake.tv_nsec as _);
        let last_handshake_time = if last_handshake == Duration::from_secs(0) {None} else {
            Some(SystemTime::UNIX_EPOCH + last_handshake)
        };

        let endpoint = map.remove(&Some(WgPeerAttribute::Endpoint)).map(|attr| {
            let endpoint = attr.endpoint.unwrap();
            match endpoint.len() {
                l if l == std::mem::size_of::<nix::sys::socket::sockaddr_in>() => {
                    let s: &nix::sys::socket::sockaddr_in = unsafe {
                        std::mem::transmute(&endpoint[0])
                    };
                    nix::sys::socket::InetAddr::V4(s.clone()).to_std()
                },
                l if l == std::mem::size_of::<nix::sys::socket::sockaddr_in6>() => {
                    let s: &nix::sys::socket::sockaddr_in6 = unsafe {
                        std::mem::transmute(&endpoint[0])
                    };
                    nix::sys::socket::InetAddr::V6(s.clone()).to_std()
                },
                _ => unreachable!(),
            }
        });
        
        Peer {
            public_key,
            preshared_key,
            endpoint,
            persistent_keepalive_interval: keepalive,
            allowed_ips: Some(()),
            protocol_version: map.remove(&Some(WgPeerAttribute::ProtocolVersion)).unwrap().protocol_version.unwrap(),
            rx_bytes: map.remove(&Some(WgPeerAttribute::RxBytes)).unwrap().rx_bytes.unwrap() as _,
            tx_bytes: map.remove(&Some(WgPeerAttribute::TxBytes)).unwrap().tx_bytes.unwrap() as _,
            last_handshake_time,
        }
    }
}

#[test]
fn test_get_family() {
    use crate::Protocol;

    let mut sock = NlSocket::new(Protocol::Generic).unwrap();
    tokio::run_async(async move {
        let _wg = await!(sock.get_family("wireguard")).unwrap().unwrap();
    });
}

#[test]
fn test_get_wg_device() {
    use crate::Protocol;

    let mut sock = NlSocket::new(Protocol::Generic).unwrap();
    tokio::run_async(async move {
        let wg = await!(sock.get_family("wireguard")).unwrap().unwrap();
        let _cfg = await!(sock.get_wg_device(&wg, IfaceBy::Name("test1".to_string()))).unwrap();
    });
}

#[test]
fn test_set_wg_endpoint() {
    use crate::Protocol;
    use std::net::ToSocketAddrs;
    let peer = [131, 221, 118, 101, 147, 16, 7, 68, 68, 184, 14, 219, 172, 182, 46, 153, 221, 21, 65, 45, 191, 39, 255, 40, 118, 254, 203, 97, 41, 190, 115, 13];
    let addr = "127.0.0.1:1234".to_socket_addrs().unwrap().next().unwrap();

    let mut sock = NlSocket::new(Protocol::Generic).unwrap();
    tokio::run_async(async move {
        let wg = await!(sock.get_family("wireguard")).unwrap().unwrap();
        await!(sock.set_wg_endpoint(&wg, IfaceBy::Name("test1".to_string()), &peer, addr)).unwrap();
    });
}
