#![feature(test)]
#![cfg(any(feature = "v2", feature = "v3", feature = "v5",))]

extern crate test;
use test::Bencher;

use unik::{self, rfc4122::v2::Domain, *};

#[bench]
fn new_uuid_v1(b: &mut Bencher) {
    b.iter(|| UUID::v1().generate());
}

#[bench]
fn new_uuid_v2(b: &mut Bencher) {
    b.iter(|| UUID::v2(Domain::PERSON).generate());
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
