# Hiredis [![Version][version-img]][version-url] [![Status][status-img]][status-url]

The package provides an interface to [Hiredis][1].

## [Documentation][doc]

## Example

Assuming that Redis is installed and is listening to port 6379, the example
given below can be ran using the following command:

```
cargo run --example workflow
```

```rust
extern crate hiredis;

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
```

## Contributing

1. Fork the project.
2. Implement your idea.
3. Open a pull request.

[1]: https://github.com/redis/hiredis

[version-img]: https://img.shields.io/crates/v/hiredis.svg
[version-url]: https://crates.io/crates/hiredis
[status-img]: https://travis-ci.org/stainless-steel/hiredis.svg?branch=master
[status-url]: https://travis-ci.org/stainless-steel/hiredis
[doc]: https://stainless-steel.github.io/hiredis
