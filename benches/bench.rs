#![feature(test)]
#![cfg(any(
    feature = "v1",
    feature = "v2",
    feature = "v3",
    feature = "v5",
))]

extern crate test;
use test::Bencher;

use mac_address::MacAddress;

use unik::{self, rfc4122::v2::Domain, *};

#[bench]
fn new_uuid_v1(b: &mut Bencher) {
    b.iter(|| UUID::v1(TimeStamp(1234_5678), MacAddress::new([u8::MAX; 6])).generate());
}

#[bench]
fn new_uuid_v2(b: &mut Bencher) {
    b.iter(|| UUID::v2(Domain::PERSON, Some(1234)).generate());
}

#[bench]
fn new_uuid_v3(b: &mut Bencher) {
    b.iter(|| UUID::v3("bench", UUID::NAMESPACE_DNS).generate());
}

#[bench]
fn new_uuid_v4(b: &mut Bencher) {
    b.iter(|| UUID::v4().generate());
}

#[bench]
fn new_uuid_v5(b: &mut Bencher) {
    b.iter(|| UUID::v5("bench", UUID::NAMESPACE_X500).generate());
}
