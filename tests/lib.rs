extern crate hiredis;

use hiredis::Reply;

macro_rules! ok(
    ($result:expr) => ($result.unwrap());
);

#[test]
fn workflow() {
    let mut context = ok!(hiredis::connect("127.0.0.1", 4242));
    match ok!(context.command(&["SET", "foo", "Hi, there!"])) {
        Reply::Status(ref string) => assert_eq!(&string[..], "OK"),
        _ => assert!(false),
    }
    match ok!(context.command(&["GET", "foo"])) {
        Reply::String(ref string) => assert_eq!(&string[..], "Hi, there!"),
        _ => assert!(false),
    }
}
