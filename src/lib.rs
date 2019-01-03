#![allow(dead_code)]
#![feature(stmt_expr_attributes)]
#![feature(try_blocks)]
#![feature(await_macro, async_await, futures_api)]

#[macro_use] extern crate log;
#[macro_use] extern crate taikai;
#[macro_use] extern crate tokio;
#[macro_use] extern crate futures;
#[macro_use] extern crate enum_primitive;

#[cfg(test)]
#[macro_use] extern crate pretty_assertions;

use bytes::BytesMut;
use tokio::prelude::*;

#[cfg(test)]
mod tests;

mod io;
pub mod wg;
mod codec;
pub mod routes;
mod wg_constants;

pub use crate::wg::IfaceBy;
pub use crate::wg_constants::WG_KEY_LEN;

pub mod nlmsghdr {
    taikai_from_file!(crate::nlmsghdr, "./netlink.tsy");
}

pub mod route {
    taikai_from_file!(crate::route, "./netlink_route.tsy");

    impl NetlinkRoute {
        pub fn pad(&mut self) {
            let mut len = self.header.pad()
                + self.route.as_mut().map(|g| g.pad()).unwrap_or(0);
            
            if let Some(attrs) = &mut self.attributes {
                for attr in attrs {
                    len += attr.pad();
                }
            }

            self.header.len = len as _;
        }
    }

    impl __subtypes::Nlmsghdr {
        pub fn pad(&mut self) -> usize {
            16
        }
    }

    impl __subtypes::Ifinfomsg {
        pub fn pad(&mut self) -> usize {
            16
        }
    }

    impl __subtypes::Attr {
        pub fn pad(&mut self) -> usize {
            let len = 4 + self.value.len();
            let p = if len % 4 == 0 { 0 } else { 4 - len % 4 };
            self.padding.resize(p, 0);
            self.len = len as _;
            len + p
        }
    }
}

pub mod generic {
    use crate::wg_constants::*;
    taikai_from_file!(crate::generic, "./netlink_generic.tsy");

    impl NetlinkGeneric {
        pub fn pad(&mut self) {
            let mut len = self.header.pad()
                + self.generic.as_mut().map(|g| g.pad()).unwrap_or(0);
            
            if let Some(attrs) = &mut self.attributes {
                for attr in attrs {
                    len += attr.pad();
                }
            }

            if let Some(attrs) = &mut self.wg_attributes {
                for attr in attrs {
                    len += attr.pad();
                }
            }

            self.header.len = len as _;
        }
    }

    impl __subtypes::Nlmsghdr {
        pub fn pad(&mut self) -> usize {
            16
        }
    }

    impl __subtypes::Genl {
        pub fn pad(&mut self) -> usize {
            4
        }
    }

    impl __subtypes::Attr {
        pub fn pad(&mut self) -> usize {
            let len = 4 + self.value.len();
            let p = if len % 4 == 0 { 0 } else { 4 - len % 4 };
            self.padding.resize(p, 0);
            self.len = len as _;
            len + p
        }
    }

    impl __subtypes::__subtypes::__subtypes::PeerAttr {
        pub fn pad(&mut self) -> usize {
            let mut len = 4;

            len += self.persistent_keepalive_interval.as_ref().map(std::mem::size_of_val).unwrap_or(0);
            len += self.public_key.as_ref().map(|_| WG_KEY_LEN).unwrap_or(0);
            len += self.preshared_key.as_ref().map(|_| WG_KEY_LEN).unwrap_or(0);
            len += self.allowed_ips.as_ref().map(|_| unimplemented!()).unwrap_or(0);
            len += self.last_handshake_time.as_ref().map(Vec::len).unwrap_or(0);
            len += self.rx_bytes.as_ref().map(std::mem::size_of_val).unwrap_or(0);
            len += self.tx_bytes.as_ref().map(std::mem::size_of_val).unwrap_or(0);
            len += self.flags.as_ref().map(std::mem::size_of_val).unwrap_or(0);
            len += self.protocol_version.as_ref().map(std::mem::size_of_val).unwrap_or(0);
            len += self.endpoint.as_ref().map(Vec::len).unwrap_or(0);

            let p = if len % 4 == 0 { 0 } else { 4 - len % 4 };
            self.padding.resize(p, 0);
            self.len = len as _;
            len + p
        }
    }

    impl Default for __subtypes::__subtypes::__subtypes::PeerAttr {
        fn default() -> Self {
            Self {
                len: 0,
                typ: 0xffff,
                persistent_keepalive_interval: None,
                public_key: None,
                preshared_key: None,
                allowed_ips: None,
                last_handshake_time: None,
                rx_bytes: None,
                tx_bytes: None,
                flags: None,
                protocol_version: None,
                endpoint: None,
                padding: vec![],
            }
        }
    }


    impl __subtypes::__subtypes::Peer {
        pub fn pad(&mut self) -> usize {
            let mut len = 4;

            for p in self.peer.iter_mut() {
                len += p.pad();
            }
            
            let p = if len % 4 == 0 { 0 } else { 4 - len % 4 };
            self.padding.resize(p, 0);
            self.len = len as _;
            len + p
        }
    }

    impl Default for __subtypes::WgAttr {
        fn default() -> Self {
            Self {
                len: 0,
                typ: 0xffff,
                ifname: None,
                ifindex: None,
                private_key: None,
                public_key: None,
                flags: None,
                fwmark: None,
                peers: None,
                listen_port: None,
                padding: vec![],
            }
        }
    }

    impl __subtypes::WgAttr {
        pub fn pad(&mut self) -> usize {
            let mut len = 4;

            len += self.ifname.as_ref().map(|v| v.len()).unwrap_or(0);
            len += self.ifindex.as_ref().map(std::mem::size_of_val).unwrap_or(0);
            len += self.private_key.as_ref().map(|v| v.len()).unwrap_or(0);
            len += self.public_key.as_ref().map(|v| v.len()).unwrap_or(0);
            len += self.flags.as_ref().map(std::mem::size_of_val).unwrap_or(0);
            len += self.fwmark.as_ref().map(std::mem::size_of_val).unwrap_or(0);
            len += self.peers.as_mut().map(|peers| {
                let mut len = 0;
                for p in peers {
                    len += p.pad();
                }
                len
            }).unwrap_or(0);

            let p = if len % 4 == 0 { 0 } else { 4 - len % 4 };
            self.padding.resize(p, 0);
            self.len = len as _;
            len + p
        }
    }
}

pub enum Protocol {
    Route,
    Generic,
}

impl Into<libc::c_int> for Protocol {
    fn into(self) -> libc::c_int {
        match self {
            Protocol::Route => libc::NETLINK_ROUTE,
            Protocol::Generic => libc::NETLINK_GENERIC,
        }
    }
}

const INITIAL_RD_CAPACITY: usize = 64 * 1024;
const INITIAL_WR_CAPACITY: usize = 8 * 1024;

pub struct NlSocket {
    io: crate::io::Io,
    rd: BytesMut,
    wr: BytesMut,
    flushed: bool,

    seq: u32,
}

#[derive(Clone, Copy, Debug)]
pub struct Family {
    id: u16,
    version: u8,
}

use std::path::Path;
use std::os::unix::io::AsRawFd;
use nix::errno::Errno;
use nix::unistd::getpid;
use nix::unistd::gettid;

impl NlSocket {
    pub fn new(protocol: impl Into<libc::c_int>) -> nix::Result<NlSocket> {
        Ok(NlSocket {
            seq: 0,
            flushed: true,
            rd: BytesMut::with_capacity(INITIAL_RD_CAPACITY),
            wr: BytesMut::with_capacity(INITIAL_WR_CAPACITY),
            io: crate::io::Io::new(protocol.into())?,
        })
    }

    pub fn new_in_netns(netns: String, protocol: impl Into<libc::c_int>) -> nix::Result<NlSocket> {
        let old_ns = std::fs::File::open(format!("/proc/{}/task/{}/ns/net", getpid(), gettid()))
            .map_err(|e| nix::Error::from_errno(Errno::from_i32(e.raw_os_error().unwrap())))?;

        let path = Path::new("/run/netns").join(netns);
        let ns = std::fs::File::open(path)
            .map_err(|e| nix::Error::from_errno(Errno::from_i32(e.raw_os_error().unwrap())))?;

        nix::sched::setns(ns.as_raw_fd(), nix::sched::CloneFlags::CLONE_NEWNET)?;

        let res = Self::new(protocol);

        nix::sched::setns(old_ns.as_raw_fd(), nix::sched::CloneFlags::CLONE_NEWNET).unwrap();

        res
    }
}

impl NlSocket {
    async fn request_generic<'a>(&'a mut self,
            req: &'a mut crate::generic::NetlinkGeneric,
            ctx: &'a crate::generic::Context)
        -> std::io::Result<Vec<crate::generic::NetlinkGeneric>>
    {
        req.pad();
        req.header.seq = self.seq;
        self.seq += 1;

        let mut buf = vec![];
        req.write(&mut buf, ctx).unwrap();
        
        await!(self.send_async(buf.into())).unwrap();

        let mut responses: Vec<_> = vec![];
        while let Some(Ok(item)) = await!(self.next()) {
            let mut buf = &item[..];
            while buf.len() > 0 {
                let resp = crate::generic::NetlinkGeneric::read(buf, ctx);
                if let Ok((rest, resp)) = resp {
                    buf = rest;

                    if resp.header.seq == req.header.seq {
                        if let Some(err) = resp.error {
                            if err.error < 0 {
                                return Err(std::io::Error::from_raw_os_error(-err.error));
                            } else {
                                return Ok(responses);
                            }
                        }
                        if resp.success.is_some() {
                            return Ok(responses);
                        }
                        responses.push(resp);
                    }
                } else {
                    return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "failed to read netlink packet"));
                }
            }
        };

        unreachable!("got no response")
    }

    async fn request_route<'a>(&'a mut self,
            req: &'a mut crate::route::NetlinkRoute)
        -> std::io::Result<Vec<crate::route::NetlinkRoute>>
    {
        req.pad();
        req.header.seq = self.seq;
        self.seq += 1;

        let ctx = &crate::route::Context {};

        let mut buf = vec![];
        req.write(&mut buf, ctx).unwrap();
        
        await!(self.send_async(buf.into())).unwrap();

        let mut responses: Vec<_> = vec![];
        while let Some(Ok(item)) = await!(self.next()) {
            let mut buf = &item[..];
            while buf.len() > 0 {
                let resp = crate::route::NetlinkRoute::read(buf, ctx);
                if let Ok((rest, resp)) = resp {
                    buf = rest;

                    if resp.header.seq == req.header.seq {
                        if let Some(err) = resp.error {
                            if err.error < 0 {
                                return Err(std::io::Error::from_raw_os_error(-err.error));
                            } else {
                                return Ok(responses);
                            }
                        }
                        if resp.success.is_some() {
                            return Ok(responses);
                        }
                        responses.push(resp);
                    }
                } else {
                    return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "failed to read netlink packet"));
                }
            }
        };

        unreachable!("got no response")
    }
}
