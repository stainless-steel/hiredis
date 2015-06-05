extern crate hiredis;

use hiredis::Reply;

fn main() {
    let mut context = hiredis::connect("127.0.0.1", 6379).unwrap();

    match context.command(&["SET", "greeting", "Hi, there!"]).unwrap() {
        Reply::Status(_) => {},
        _ => assert!(false),
    }

    match context.command(&["GET", "greeting"]).unwrap() {
        Reply::Bulk(bytes) => println!("{}", String::from_utf8(bytes).unwrap()),
        _ => assert!(false),
    };
}
