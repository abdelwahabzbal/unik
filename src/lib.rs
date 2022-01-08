//! This crate defines a powerful uniform resource name namespace for UUIDs
//! (Universally Unique Identifier), which are suited for modern use.
//!
//! This lib can be used to create unique and reasonably short
//! values without requiring extra knowledge.
//!
//! A UUID is 128 bits long, and can guarantee
//! uniqueness across space and time.
#![doc(html_root_url = "https://docs.rs/unik")]
#![feature(doc_cfg)]

use core::fmt;
use core::sync::atomic;

use chrono::Utc;
// use mac_address::MacAddress;

//
#[derive(Debug, Default)]
pub struct Node(pub [u8; 8]); // NOTICE: We have not `u48` so use `u64` instead

/// Timestamp used as a `u64`. For this reason, dates prior to gregorian
/// calendar are not supported.
pub struct Timestamp(u64);

impl Timestamp {
    pub fn from_hidjri() {
        todo!()
    }

    pub fn from_utc() -> Self {
        Timestamp(Utc::now().timestamp_nanos() as u64)
    }

    pub fn from_unix() -> Self {
        Timestamp(
            Utc::now()
                .checked_sub_signed(chrono::Duration::nanoseconds(0x01B2_1DD2_1381_4000))
                .unwrap()
                .timestamp_nanos() as u64,
        )
    }
}

/// Used to avoid duplicates that could arise when the clock is set backwards in time.
pub struct ClockSeq(u16);

impl ClockSeq {
    pub fn new(rand: u16) -> u16 {
        atomic::AtomicU16::new(rand).fetch_add(1, atomic::Ordering::SeqCst)
    }
}

#[derive(Debug)]
pub struct Layout {
    /// The low field of the Timestamp.
    pub field_low: u32,
    /// The mid field of the Timestamp.
    pub field_mid: u16,
    /// The high field of the Timestamp multiplexed with the version number.
    pub field_high_and_version: u16,
    /// The high field of the ClockSeq multiplexed with the variant.
    pub clock_seq_high_and_reserved: u8,
    /// The low field of the ClockSeq.
    pub clock_seq_low: u8,
    /// IEEE-802 network address.
    pub node: Node,
}

impl Layout {
    pub fn as_fields(&self) -> (u32, u16, u16, u16, u64) {
        (
            u32::from_ne_bytes(self.field_low.to_ne_bytes()),
            u16::from_ne_bytes(self.field_mid.to_ne_bytes()),
            u16::from_ne_bytes(self.field_high_and_version.to_ne_bytes()),
            u16::from_ne_bytes(
                ((self.clock_seq_high_and_reserved as u16) << 8 | self.clock_seq_low as u16)
                    .to_ne_bytes(),
            ),
            u64::from_ne_bytes(self.node.0),
        )
    }

    /// Returns the five field values of the `UUID` in little-endian order.
    pub fn as_fields_le(&self) -> (u32, u16, u16, u16, u64) {
        (
            self.field_low.to_le(),
            self.field_mid.to_le(),
            self.field_high_and_version.to_le(),
            ((self.clock_seq_high_and_reserved as u16) << 8 | self.clock_seq_low as u16).to_le(),
            u64::from_ne_bytes(self.node.0),
        )
    }

    /// Returns the five field values of the `UUID` in big-endian order.
    pub fn as_fields_be(&self) -> (u32, u16, u16, u16, u64) {
        (
            self.field_low.to_be(),
            self.field_mid.to_be(),
            self.field_high_and_version.to_be(),
            ((self.clock_seq_high_and_reserved as u16) << 8 | self.clock_seq_low as u16).to_be(),
            u64::from_ne_bytes(self.node.0),
        )
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct UUID([u8; 16]);

impl UUID {
    /// UUID namespace for domain name system (DNS).
    pub const DNS: UUID = UUID([
        0x6b, 0xa7, 0xb8, 0x10, 0x9d, 0xad, 0x11, 0xd1, 0x80, 0xb4, 0x00, 0xc0, 0x4f, 0xd4, 0x30,
        0xc8,
    ]);

    /// UUID namespace for ISO object identifiers (OIDs).
    pub const OID: UUID = UUID([
        0x6b, 0xa7, 0xb8, 0x12, 0x9d, 0xad, 0x11, 0xd1, 0x80, 0xb4, 0x00, 0xc0, 0x4f, 0xd4, 0x30,
        0xc8,
    ]);

    /// UUID namespace for uniform resource locators (URLs).
    pub const URL: UUID = UUID([
        0x6b, 0xa7, 0xb8, 0x11, 0x9d, 0xad, 0x11, 0xd1, 0x80, 0xb4, 0x00, 0xc0, 0x4f, 0xd4, 0x30,
        0xc8,
    ]);

    /// UUID namespace for X.500 distinguished names (DNs).
    pub const X500: UUID = UUID([
        0x6b, 0xa7, 0xb8, 0x14, 0x9d, 0xad, 0x11, 0xd1, 0x80, 0xb4, 0x00, 0xc0, 0x4f, 0xd4, 0x30,
        0xc8,
    ]);
}

impl fmt::Display for UUID {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            fmt,
            "{:02}{:02}{:02}{:02}{:02}{:02}{:02}{:02}{:02}{:02}{:02}{:02}{:02}{:02}{:02}{:02}",
            self.0[0],
            self.0[1],
            self.0[2],
            self.0[3],
            self.0[4],
            self.0[5],
            self.0[6],
            self.0[7],
            self.0[8],
            self.0[9],
            self.0[10],
            self.0[11],
            self.0[12],
            self.0[13],
            self.0[14],
            self.0[15],
        )
    }
}

impl fmt::LowerHex for UUID {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            fmt,
            "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            self.0[0],
            self.0[1],
            self.0[2],
            self.0[3],
            self.0[4],
            self.0[5],
            self.0[6],
            self.0[7],
            self.0[8],
            self.0[9],
            self.0[10],
            self.0[11],
            self.0[12],
            self.0[13],
            self.0[14],
            self.0[15],
        )
    }
}

impl fmt::UpperHex for UUID {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            fmt,
            "{:02X}{:02X}{:02X}{:02X}-{:02X}{:02X}-{:02X}{:02X}-{:02X}{:02X}-{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}",
            self.0[0],
            self.0[1],
            self.0[2],
            self.0[3],
            self.0[4],
            self.0[5],
            self.0[6],
            self.0[7],
            self.0[8],
            self.0[9],
            self.0[10],
            self.0[11],
            self.0[12],
            self.0[13],
            self.0[14],
            self.0[15],
        )
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Version {
    /// The time-based version specified in `rfc4122` document.
    TIME = 1,
    /// DCE-security version, with embedded POSIX UIDs.
    DCE,
    /// The name-based version specified in `rfc4122` document that uses MD5 hashing.
    MD5,
    /// The randomly or pseudo-randomly generated version specified in `rfc4122` document.
    RAND,
    /// The name-based version specified in `rfc4122`document that uses SHA-1 hashing.
    SHA1,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Variant {
    /// Reserved, NCS backward compatibility.
    NCS = 0,
    /// The variant specified in `rfc4122` document.
    RFC,
    /// Reserved, Microsoft Corporation backward compatibility.
    MS,
    /// Reserved for future definition.
    FUT,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn uuid_default() {
        let uuid = UUID::default();
        assert_eq!(uuid, UUID([0; 16]));
    }
}
