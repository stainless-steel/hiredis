extern crate hiredis;

macro_rules! ok(
    ($result:expr) => ($result.unwrap());
);

#[test]
fn workflow() {
    let _redis = ok!(hiredis::connect("127.0.0.1", 4242));
}
