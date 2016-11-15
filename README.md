# Hiredis [![Version][version-img]][version-url] [![Status][status-img]][status-url]

The package provides an interface to [Hiredis][1].

## [Documentation][documentation]

## Example

```rust
use hiredis::Reply;

let mut context = hiredis::connect("127.0.0.1", 6379).unwrap();

match context.command(&["SET", "greeting", "Hi, there!"]).unwrap() {
    Reply::Status(_) => {},
    _ => assert!(false),
}

match context.command(&["GET", "greeting"]).unwrap() {
    Reply::Bulk(bytes) => println!("{}", String::from_utf8(bytes).unwrap()),
    _ => assert!(false),
};
```

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE.md](LICENSE.md).

[1]: https://github.com/redis/hiredis

[documentation]: https://docs.rs/hiredis
[status-img]: https://travis-ci.org/stainless-steel/hiredis.svg?branch=master
[status-url]: https://travis-ci.org/stainless-steel/hiredis
[version-img]: https://img.shields.io/crates/v/hiredis.svg
[version-url]: https://crates.io/crates/hiredis
