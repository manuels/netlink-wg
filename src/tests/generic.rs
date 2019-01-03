use crate::generic::*;
use crate::generic::__subtypes::*;

test_compare!(test_get_family, "./data/generic-get-family.bin", NetlinkGeneric, 0,
    Context {
        wireguard_family: 0xffff,
        wireguard_version: 0xff,
    },
    NetlinkGeneric {
        header: Nlmsghdr {
            len: 36,
            typ: 16,
            flags: 5,
            seq: 1545653536,
            pid: 0
        },
        error: None,
        success: None,
        generic: Some( Genl {
            cmd: 3,
            version: 1,
            padding: 0
        }),
        wg_attributes: None,
        attributes: Some(vec![
            Attr {
                len: 14,
                typ: 2,
                value: b"wireguard\0".to_vec(),
                padding: vec![
                    0,
                    0
                ]
            }
        ]),
    }
);

test_compare!(test_new_family, "./data/generic-new-family.bin", NetlinkGeneric, 0,
    Context {
        wireguard_family: 0xffff,
        wireguard_version: 0xff,
    },
    NetlinkGeneric {
        header: Nlmsghdr {
            len: 112,
            typ: 16,
            flags: 0,
            seq: 1545653536,
            pid: 5756
        },
        generic: Some(
            Genl {
                cmd: 1,
                version: 2,
                padding: 0
            }
        ),
        error: None,
        success: None,
        wg_attributes: None,
        attributes: Some(
            vec![
                Attr {
                    len: 14,
                    typ: 2,
                    value: b"wireguard\0".to_vec(),
                    padding: vec![
                        0,
                        0
                    ]
               },
                Attr {
                    len: 6,
                    typ: 1,
                    value: vec![
                        25,
                        0
                    ],
                    padding: vec![
                        0,
                        0
                    ]
                },
                Attr {
                    len: 8,
                    typ: 3,
                    value: vec![
                        1,
                        0,
                        0,
                        0
                    ],
                    padding: vec![]
                },
                Attr {
                    len: 8,
                    typ: 4,
                    value: vec![
                        0,
                        0,
                        0,
                        0
                    ],
                    padding: vec![]
                },
                Attr {
                    len: 8,
                    typ: 5,
                    value: vec![
                        8,
                        0,
                        0,
                        0
                    ],
                    padding: vec![]
                },
                Attr {
                    len: 44,
                    typ: 6,
                    value: vec![
                        20,
                        0,
                        1,
                        0,
                        8,
                        0,
                        1,
                        0,
                        0,
                        0,
                        0,
                        0,
                        8,
                        0,
                        2,
                        0,
                        28,
                        0,
                        0,
                        0,
                        20,
                        0,
                        2,
                        0,
                        8,
                        0,
                        1,
                        0,
                        1,
                        0,
                        0,
                        0,
                        8,
                        0,
                        2,
                        0,
                        26,
                        0,
                        0,
                        0
                    ],
                    padding: vec![]
                }
            ]
        )
    }
);
