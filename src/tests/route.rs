use crate::route::*;
use crate::route::__subtypes::*;

test_compare!(test_get_interface, "./data/route-get-interface.bin", NetlinkRoute, 0,
    NetlinkRoute {
        header: Nlmsghdr {
            len: 32,
            typ: 18,
            flags: 773,
            seq: 1545653536,
            pid: 0
        },
        route: Some(Ifinfomsg {
            family: 0,
            reserved: 0,
            typ: 0,
            index: 0,
            flags: 0,
            change: 0
        }),
        error: None,
        success: None,
        attributes: Some(vec![]),
    }
);

test_compare!(test_create_interface, "./data/route-create-interface.bin", NetlinkRoute, 0,
    NetlinkRoute {
        header: Nlmsghdr {
            len: 1288,
            typ: 16,
            flags: 2,
            seq: 1545653536,
            pid: 5756
        },
        route: Some(Ifinfomsg {
            family: 0,
            reserved: 0,
            typ: 772,
            index: 1,
            flags: 8,
            change: 0
        }),
        error: None,
        success: None,
        attributes: Some(vec![
            Attr { len: 7, typ: 3, value: vec![108, 111, 0], padding: vec![0], link_info: None },
            Attr { len: 8, typ: 13, value: vec![232, 3, 0, 0], padding: vec![], link_info: None },
            Attr { len: 5, typ: 16, value: vec![2], padding: vec![0, 0, 0], link_info: None },
            Attr { len: 5, typ: 17, value: vec![0], padding: vec![0, 0, 0], link_info: None },
            Attr { len: 8, typ: 4, value: vec![0, 0, 1, 0], padding: vec![], link_info: None },
            Attr { len: 8, typ: 27, value: vec![0, 0, 0, 0], padding: vec![], link_info: None },
            Attr { len: 8, typ: 30, value: vec![0, 0, 0, 0], padding: vec![], link_info: None },
            Attr { len: 8, typ: 31, value: vec![1, 0, 0, 0], padding: vec![], link_info: None },
            Attr { len: 8, typ: 40, value: vec![255, 255, 0, 0], padding: vec![], link_info: None },
            Attr { len: 8, typ: 41, value: vec![0, 0, 1, 0], padding: vec![], link_info: None },
            Attr { len: 8, typ: 32, value: vec![1, 0, 0, 0], padding: vec![], link_info: None },
            Attr { len: 5, typ: 33, value: vec![1], padding: vec![0, 0, 0], link_info: None },
            Attr { len: 9, typ: 6, value: vec![110, 111, 111, 112, 0], padding: vec![0, 0, 0], link_info: None },
            Attr { len: 8, typ: 35, value: vec![0, 0, 0, 0], padding: vec![], link_info: None },
            Attr { len: 5, typ: 39, value: vec![0], padding: vec![0, 0, 0], link_info: None },
            Attr { len: 8, typ: 47, value: vec![0, 0, 0, 0], padding: vec![], link_info: None },
            Attr { len: 8, typ: 48, value: vec![0, 0, 0, 0], padding: vec![], link_info: None },
            Attr { len: 36, typ: 14, value: vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], padding: vec![], link_info: None },
            Attr { len: 10, typ: 1, value: vec![0, 0, 0, 0, 0, 0], padding: vec![0, 0], link_info: None},
            Attr { len: 10, typ: 2, value: vec![0, 0, 0, 0, 0, 0], padding: vec![0, 0], link_info: None },
            Attr { len: 196, typ: 23, value: vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], padding: vec![], link_info: None },
            Attr { len: 100, typ: 7, value: vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], padding: vec![], link_info: None },
            Attr { len: 12, typ: 43, value: vec![5, 0, 2, 0, 0, 0, 0, 0], padding: vec![], link_info: None },
            Attr { len: 748, typ: 26, value: vec![132, 0, 2, 0, 128, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 39, 0, 0, 232, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 100, 2, 10, 0, 8, 0, 1, 0, 0, 0, 0, 0, 20, 0, 5, 0, 255, 255, 0, 0, 235, 146, 7, 0, 120, 158, 0, 0, 232, 3, 0, 0, 208, 0, 2, 0, 0, 0, 0, 0, 64, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 255, 255, 255, 255, 160, 15, 0, 0, 232, 3, 0, 0, 255, 255, 255, 255, 128, 58, 9, 0, 128, 81, 1, 0, 3, 0, 0, 0, 88, 2, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 96, 234, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 16, 39, 0, 0, 232, 3, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 238, 54, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 1, 3, 0, 36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 52, 0, 6, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 20, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 8, 0, 0, 0, 0, 0], padding: vec![], link_info: None }
        ]),
    }
);

test_compare!(test_eod, "./data/route-eod.bin", NetlinkRoute, 0,
    NetlinkRoute {
        header: Nlmsghdr {
            len: 20,
            typ: 3,
            flags: 2,
            seq: 1545653536,
            pid: 5756
        },
        error: None,
        success: Some(vec![0; 4]),
        route: None,
        attributes: None,
    }
);
