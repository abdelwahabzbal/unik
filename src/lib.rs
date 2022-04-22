//! This crate defines a powerful uniform resource name namespace for UUIDs
//! (Universally Unique Identifier), which are suited for modern use.
//!
//! This lib can be used to create unique and reasonably short
//! values without requiring extra knowledge.
//!
//! A UUID is 128 bits long, and can guarantee uniqueness across space and time.
//!
//! ```toml
//! [dependencies]
//! unik = { version = "0.2.4", features = ["v4"] }
//! ```
//!
//! ```rust
//! use unik::rfc::v4;
//! use unik::UUID;
//!
//! fn main() {
//!     println!("{:x}", UUID::v4().generate());
//! }
//! ```
#![doc(html_root_url = "https://docs.rs/unik")]
#![feature(doc_cfg)]

pub mod rfc;

use core::fmt;
use std::sync::atomic::{self, AtomicU16};

use chrono::Utc;
use nanorand::{Rng, WyRand};

pub use mac_address::{get_mac_address, MacAddress};

/// Represent bytes of MAC address.
pub type Node = MacAddress;

/// The simplified version of `UUID` in terms of fields that are integral numbers of octets.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Layout {
    /// The low field of the Timestamp.
    field_low: u32,
    /// The mid field of the Timestamp.
    field_mid: u16,
    /// The high field of the Timestamp multiplexed with the version number.
    field_high_and_version: u16,
    /// The high field of the ClockSeq multiplexed with the variant.
    clock_seq_high_and_reserved: u8,
    /// The low field of the ClockSeq.
    clock_seq_low: u8,
    /// IEEE-802 network address.
    node: Node,
}

impl Layout {
    /// Returns the five fields of `UUID`.
    pub fn as_fields(&self) -> (u32, u16, u16, u16, u64) {
        (
            u32::from_ne_bytes(self.field_low.to_ne_bytes()),
            u16::from_ne_bytes(self.field_mid.to_ne_bytes()),
            u16::from_ne_bytes(self.field_high_and_version.to_ne_bytes()),
            u16::from_ne_bytes(
                ((self.clock_seq_high_and_reserved as u16) << 8 | self.clock_seq_low as u16)
                    .to_ne_bytes(),
            ),
            u64::from_ne_bytes([
                (self.node.bytes()[0]),
                (self.node.bytes()[1]),
                (self.node.bytes()[2]),
                (self.node.bytes()[3]),
                (self.node.bytes()[4]),
                (self.node.bytes()[5]),
                /* Make `from_ne_bytes` method happy
                 */
                0,
                0,
            ]),
        )
    }

    /// Returns the memory representation of `UUID` in native byte order.
    pub fn generate(&self) -> UUID {
        UUID([
            self.field_low.to_ne_bytes()[0],
            self.field_low.to_ne_bytes()[1],
            self.field_low.to_ne_bytes()[2],
            self.field_low.to_ne_bytes()[3],
            self.field_mid.to_ne_bytes()[0],
            self.field_mid.to_ne_bytes()[1],
            self.field_high_and_version.to_ne_bytes()[0],
            self.field_high_and_version.to_ne_bytes()[1],
            self.clock_seq_high_and_reserved,
            self.clock_seq_low,
            self.node.bytes()[0],
            self.node.bytes()[1],
            self.node.bytes()[2],
            self.node.bytes()[3],
            self.node.bytes()[4],
            self.node.bytes()[5],
        ])
    }

    /// Get version of the current generated `UUID`.
    pub const fn get_version(&self) -> Result<Version, &str> {
        match (self.field_high_and_version) >> 0xc {
            0x1 => Ok(Version::TIME),
            0x2 => Ok(Version::DCE),
            0x3 => Ok(Version::MD5),
            0x4 => Ok(Version::RAND),
            0x5 => Ok(Version::SHA1),
            _ => Err("Invalid version"),
        }
    }

    /// Get variant of the current generated `UUID`.
    pub const fn get_variant(&self) -> Result<Variant, &str> {
        match self.clock_seq_high_and_reserved >> 0x4 {
            0x0 => Ok(Variant::NCS),
            0x1 => Ok(Variant::RFC),
            0x2 => Ok(Variant::MS),
            0x3 => Ok(Variant::FUT),
            _ => Err("Invalid variant"),
        }
    }
}

/// The `UUID` format is 16 octets.
pub type Bytes = [u8; 16];

/// Is a 128-bit number used to identify information in computer systems.
#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub struct UUID(Bytes);

impl UUID {
    /// Return the memory representation of `UUID`.
    pub const fn as_bytes(&self) -> [u8; 16] {
        self.0
    }

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

    // Parse `UUID` from a string of hex digits.
    pub fn from_str(us: &str) -> Result<Layout, &str> {
        let mut us = us.to_string();
        let mut bytes = [0; 16];

        if us.len() == 36 || us.len() == 32 {
            if us.contains('-') {
                us.retain(|c| !c.is_ascii_whitespace() && c != '-');
            }

            for i in 0..15 {
                let s = &us[i * 2..i * 2 + 2];
                let byte = u8::from_str_radix(s, 16).map_err(|_| "Invalid UUID string")?;

                bytes[i] = byte;
            }
        } else {
            return Err("Invalid UUID string");
        }

        Ok(layout!(
            bytes[3], bytes[2], bytes[1], bytes[0], bytes[5], bytes[4], bytes[7], bytes[6],
            bytes[9], bytes[8], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15]
        ))
    }
}

impl fmt::Display for UUID {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
                fmt,
                "{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
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

/// Version represents the type of UUID, and is in the most significant 4 bits of the Timestamp.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
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

/// Variant is a type field determines the layout of the UUID.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
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

/// Represented by Coordinated Universal Time (UTC).
///
/// NOTE: `Timestamp` used as a `u64`. For this reason,
/// dates prior to gregorian calendar are not supported.
#[derive(Clone, Copy)]
pub struct Timestamp(pub u64);

impl Timestamp {
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
        AtomicU16::new(rand).fetch_add(1, atomic::Ordering::SeqCst)
    }
}

pub(crate) fn clock_seq_high_and_reserved() -> u16 {
    ClockSeq::new(WyRand::new().generate::<u16>())
}

pub(crate) fn get_random() -> u128 {
    WyRand::new().generate::<u128>()
}

#[macro_export]
macro_rules! layout {
    ($b0:expr, $b1:expr, $b2:expr, $b3:expr,
        $b4:expr, $b5:expr, $b6:expr, $b7:expr,
        $b8:expr, $b9:expr, $b10:expr, $b11:expr,
        $b12:expr, $b13:expr, $b14:expr, $b15:expr) => {
        Layout {
            field_low: $b0 as u32 | ($b1 as u32) << 8 | ($b2 as u32) << 16 | ($b3 as u32) << 24,
            field_mid: ($b4 as u16) | (($b5 as u16) << 8),
            field_high_and_version: ($b6 as u16) | ($b7 as u16) << 8,
            clock_seq_high_and_reserved: ($b8 & 0xf) as u8 | (Variant::RFC as u8) << 4,
            clock_seq_low: $b9,
            node: MacAddress::new([$b10, $b11, $b12, $b13, $b14, $b15]),
        }
    };
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn uuid_default() {
        let uuid = UUID::default();
        assert_eq!(uuid, UUID([0; 16]));
    }

    #[test]
    fn parse_string() {
        // v1
        assert_eq!(
            Ok(Version::TIME),
            UUID::from_str("ab720268-b83f-11ec-b909-0242ac120002")
                .unwrap()
                .get_version()
        );
        // println!(
        //     "{:02x?}",
        //     UUID::from_str("ab720268-b83f-11ec-b909-0242ac120002").unwrap()
        // );
        assert_eq!(
            Ok(Variant::RFC),
            UUID::from_str("ab720268b83f11ecb9090242ac120002")
                .unwrap()
                .get_variant()
        );

        // // v2
        assert_eq!(
            Ok(Version::DCE),
            UUID::from_str("000003e8-c22b-21ec-bd01-d4bed9408ecc")
                .unwrap()
                .get_version()
        );
        assert_eq!(
            Ok(Variant::RFC),
            UUID::from_str("000003e8c22a21ecb600d4bed9408ecc")
                .unwrap()
                .get_variant()
        );

        // // v3
        assert_eq!(
            Ok(Version::MD5),
            UUID::from_str("2448bd95-00ca-3650-160f-3301a691b26c")
                .unwrap()
                .get_version()
        );
        assert_eq!(
            Ok(Variant::RFC),
            UUID::from_str("2448bd9500ca3650160f3301a691b26c")
                .unwrap()
                .get_variant(),
        );

        // // v4
        assert_eq!(
            Ok(Version::RAND),
            UUID::from_str("6a665038-24cf-4cf6-9b61-05f0c2fc6c08")
                .unwrap()
                .get_version()
        );
        assert_eq!(
            Ok(Variant::RFC),
            UUID::from_str("6a66503824cf4cf69b6105f0c2fc6c08")
                .unwrap()
                .get_variant()
        );

        // // v5
        assert_eq!(
            Ok(Version::SHA1),
            UUID::from_str("991da866-83b0-5550-1bef-37a1a5b1fb30")
                .unwrap()
                .get_version()
        );
        assert_eq!(
            Ok(Variant::RFC),
            UUID::from_str("991da86683b055501bef37a1a5b1fb30")
                .unwrap()
                .get_variant()
        );
    }
}
