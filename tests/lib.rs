extern crate hiredis;

use hiredis::Reply;

macro_rules! ok(
    ($result:expr) => ($result.unwrap());
);

#[test]
fn set_get_strings() {
    let mut context = ok!(hiredis::connect("127.0.0.1", 6379));
    match ok!(context.command(&["SET", "hiredis-foo", "Hi, there!"])) {
        Reply::Status(ref string) => assert_eq!(&string[..], "OK"),
        _ => assert!(false),
    }
    match ok!(context.command(&["GET", "hiredis-foo"])) {
        Reply::Bulk(bytes) => assert_eq!(&ok!(String::from_utf8(bytes))[..], "Hi, there!"),
        _ => assert!(false),
    }
}

#[test]
fn set_get_bytes() {
    let mut context = ok!(hiredis::connect("127.0.0.1", 6379));
    match ok!(context.command(&[&b"SET"[..], &b"hiredis-bar"[..], &[42u8]])) {
        Reply::Status(ref string) => assert_eq!(&string[..], "OK"),
        _ => assert!(false),
    }
    match ok!(context.command(&["GET", "hiredis-bar"])) {
        Reply::Bulk(ref bytes) => assert_eq!(&bytes[..], &[42u8]),
        _ => assert!(false),
    }
}

#[test]
fn push_pop_strings() {
    let mut context = ok!(hiredis::connect("127.0.0.1", 6379));
    match ok!(context.command(&["RPUSH", "hiredis-baz", "Good news, everyone!"])) {
        Reply::Integer(integer) => assert!(integer > 0),
        _ => assert!(false),
    }
    match ok!(context.command(&["BLPOP", "hiredis-baz", "10"])) {
        Reply::Array(mut elements) => {
            println!("Array!");
            assert_eq!(elements.len(), 2);
            elements.reverse();
            match elements.pop().unwrap() {
                Reply::Bulk(bytes) => {
                    assert_eq!(&ok!(String::from_utf8(bytes))[..], "hiredis-baz");
                },
                _ => assert!(false),
            }
            match elements.pop().unwrap() {
                Reply::Bulk(bytes) => {
                    assert_eq!(&ok!(String::from_utf8(bytes))[..], "Good news, everyone!");
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}
