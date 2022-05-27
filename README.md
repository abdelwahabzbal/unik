## UUID ![](https://github.com/awh6al/unik/workflows/unik/badge.svg)
A universally unique identifier (UUID) is a 128-bit number used to identify
information in computer systems. The term globally unique identifier (GUID)
is also used.

This project is in the development stage. All API's might change without warning and no 
guarantees are given about stability. `DO NOT USE IN PRODUCTION.`

This crate generates and inspects UUIDs based on
 * [RFC 4122](http://tools.ietf.org/html/rfc4122)
 * [DCE 1.1](https://pubs.opengroup.org/onlinepubs/9696989899/chap5.htm#tagcjh_08_02_01_01)

## Install
```TOML
[dependencies]

unik = { version = "*" }
```

## Usage
```Rust
fn main() {
    println!("{:x}", unik::UUID::v4().generate());
}
```

## Security

Do not assume that UUIDs are hard to guess; they should not be used as security capabilities.
