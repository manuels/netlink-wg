use std::collections::HashMap;

use crate::NlSocket;
use crate::route::*;

enum_from_primitive! {
    #[derive(Debug, PartialEq)]
    pub enum RouteMsg {
    	RTM_NEWLINK = 16,
    	RTM_DELLINK = 17,
    	RTM_GETLINK = 18,
    	RTM_SETLINK = 19,
    }
}

#[derive(Debug)]
pub struct Interface {
    pub ifname: String,
    pub ifindex: u32,
    pub link_info: Option<String>,
}

impl NlSocket {
    pub async fn get_devices(&mut self) -> std::io::Result<Vec<Interface>> {
        let mut req = NetlinkRoute {
            header: __subtypes::Nlmsghdr {
                len: 32,
                typ: RouteMsg::RTM_GETLINK as _,
                flags: (libc::NLM_F_REQUEST | libc::NLM_F_ACK | libc::NLM_F_ROOT | libc::NLM_F_MATCH) as _,
                seq: 0,
                pid: 0
            },
            error: None,
            success: None,
            route: Some(__subtypes::Ifinfomsg {
                family: 0,
                reserved: 0,
                typ: 0,
                index: 0,
                flags: 0,
                change: 0
            }),
            attributes: Some(vec![]),
        };
        req.pad();

        let resp = await!(self.request_route(&mut req))?;
        
        let ifaces = resp.into_iter().map(|msg| {
            let route = msg.route.unwrap();
            let mut map: HashMap<u16, _> = msg.attributes.unwrap().into_iter()
                .map(|a| (a.typ, a)).collect();

            let ifname = &map[&0x0003].value;
            let ifname = std::ffi::CStr::from_bytes_with_nul(&ifname[..]).unwrap();
            let ifname = ifname.to_str().unwrap().to_string();

            let link_info = map.remove(&0x0012).map(|a| {
                let subattrs = &a.link_info.unwrap();
                let map: HashMap<u16, _> = subattrs.into_iter()
                    .map(|a| (a.typ, a)).collect();

                let s = std::ffi::CStr::from_bytes_with_nul(&map[&0x0001].value).unwrap();
                s.to_str().unwrap().to_string()
            });

            Interface {
                ifname,
                link_info: link_info,
                ifindex: route.index,
            }
        }).collect();

        Ok(ifaces)
    }
}

#[test]
fn test_get_devices() {
    use crate::Protocol;

    let mut sock = NlSocket::new(Protocol::Route).unwrap();
    tokio::run_async(async move {
        let _devs = await!(sock.get_devices()).unwrap();
    });
}
