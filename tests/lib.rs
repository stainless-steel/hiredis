extern crate hiredis;

macro_rules! ok(
    ($result:expr) => ($result.unwrap());
);

#[test]
fn workflow() {
    let mut context = ok!(hiredis::connect("127.0.0.1", 4242));
    let _reply = ok!(context.command(&[r#"SET foo "Hello, world!""#]));
}
