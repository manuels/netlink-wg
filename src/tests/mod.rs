extern crate libc;

macro_rules! test_compare {
    ($fn_name: ident, $path: expr, $typ: ident, $rest: expr, $expected: expr) => (
        test_compare!($fn_name, $path, $typ, $rest, Context {}, $expected);
    );
    ($fn_name: ident, $path: expr, $typ: ident, $rest: expr, $ctx: expr, $expected: expr) => (

        #[test]
        fn $fn_name() {
            let ctx = &$ctx;
            let buf = include_bytes!($path);
            let (rest, actual) = $typ::read(buf, ctx).unwrap();

            println!("{:?}", actual);

            assert_eq!($expected, actual);
            assert_eq!($rest, rest.len());

            let mut buf2 = vec![];
            $expected.write(&mut buf2, ctx).unwrap();

            assert_eq!(&buf[..buf.len() - $rest], &buf2[..]);
        }
    )
}

mod route;
mod generic;
