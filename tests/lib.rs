extern crate hiredis;

use hiredis::Reply;

macro_rules! ok(
    ($result:expr) => ($result.unwrap());
);

#[test]
fn set_get_strings() {
    let mut context = ok!(hiredis::connect("127.0.0.1", 4242));
    match ok!(context.command(&["SET", "foo", "Hi, there!"])) {
        Reply::Status(ref string) => assert_eq!(&string[..], "OK"),
        _ => assert!(false),
    }
    match ok!(context.command(&["GET", "foo"])) {
        Reply::Bulk(bytes) => assert_eq!(&ok!(String::from_utf8(bytes))[..], "Hi, there!"),
        _ => assert!(false),
    }
}

#[test]
fn set_get_bytes() {
    let mut context = ok!(hiredis::connect("127.0.0.1", 4242));
    match ok!(context.command(&[&b"SET"[..], &b"bar"[..], &[42u8]])) {
        Reply::Status(ref string) => assert_eq!(&string[..], "OK"),
        _ => assert!(false),
    }
    match ok!(context.command(&["GET", "bar"])) {
        Reply::Bulk(ref bytes) => assert_eq!(&bytes[..], &[42u8]),
        _ => assert!(false),
    }
}
