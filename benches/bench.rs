#![feature(test)]
#![cfg(any(
    feature = "v1",
    feature = "v2",
    feature = "v3",
    feature = "v4",
    feature = "v5",
))]

extern crate test;
use test::Bencher;

use mac_address::MacAddress;

use unik::{self, versions::v2::Domain, *};

#[bench]
fn new_uuid_v1(b: &mut Bencher) {
    b.iter(|| UUID::v1(Timestamp(1234_5678), MacAddress::new([u8::MAX; 6])));
}

#[bench]
fn new_uuid_v2(b: &mut Bencher) {
    b.iter(|| {
        UUID::v2(
            unik::Timestamp::from_utc(),
            MacAddress::new([u8::MAX; 6]),
            Domain::PERSON,
        )
    });
}

#[bench]
fn new_uuid_v3(b: &mut Bencher) {
    b.iter(|| UUID::v3("test", UUID::DNS).generate());
}

#[bench]
fn new_uuid_v4(b: &mut Bencher) {
    b.iter(|| UUID::v4().generate());
}

#[bench]
fn new_uuid_v5(b: &mut Bencher) {
    b.iter(|| UUID::v5("test", UUID::X500).generate());
}
