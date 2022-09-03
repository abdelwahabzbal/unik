//! This crate defines a powerful uniform resource name namespace for UUIDs
//! (Universally Unique Identifier), which are suited for modern use.
//!
//! This lib can be used to create unique and reasonably short values without
//! requiring extra knowledge.
//!
//! A `UUID` is 128 bits long, and can guarantee uniqueness across space and time.
#![feature(stmt_expr_attributes)]
#![doc(html_root_url = "https://docs.rs/unik")]
#![feature(doc_cfg)]

pub mod rfc4122;

use core::fmt;
use std::sync::atomic::{self, AtomicU16};

pub use mac_address::{get_mac_address, MacAddress};

/// Represent bytes of host address.
pub type Node = MacAddress;

/// Is a 60-bit value. Represented by Coordinated Universal Time (UTC).
///
/// NOTE: `TimeStamp` used as a `u64`. For this reason dates prior to gregorian
/// calendar are not supported.
#[derive(Debug, Clone)]
pub struct TimeStamp(u64);

/// The simplified version of `UUID` in terms of fields that are integral numbers of octets.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Layout {
    /// The low field of the TimeStamp.
    field_low: u32,
    /// The mid field of the TimeStamp.
    field_mid: u16,
    /// The high field of the TimeStamp multiplexed with the version number.
    field_high_and_version: u16,
    /// The high field of the ClockSeq multiplexed with the variant.
    clock_seq_high_and_reserved: u8,
    /// The low field of the ClockSeq.
    clock_seq_low: u8,
    /// IEEE-802 network address.
    pub node: Node,
}

impl Layout {
    /// Returns `Layout` from sequence of integral numbers.
    pub fn from_raw_bytes(bytes: Bytes) -> Layout {
        let field_low = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        let field_mid = u16::from_le_bytes([bytes[4], bytes[5]]);
        let field_high_and_version = u16::from_le_bytes([bytes[6], bytes[7]]);
        Layout {
            field_low,
            field_mid,
            field_high_and_version,
            clock_seq_high_and_reserved: bytes[8],
            clock_seq_low: bytes[9],
            node: MacAddress::new([
                bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15],
            ]),
        }
    }

    /// Returns the memory representation of `UUID`.
    pub fn generate(&self) -> UUID {
        let fl = self.field_low.to_le_bytes();
        let fm = self.field_mid.to_le_bytes();
        let fhv = self.field_high_and_version.to_le_bytes();
        let n = self.node.bytes();
        UUID([
            fl[0],
            fl[1],
            fl[2],
            fl[3],
            fm[0],
            fm[1],
            fhv[0],
            fhv[1],
            self.clock_seq_high_and_reserved,
            self.clock_seq_low,
            n[0],
            n[1],
            n[2],
            n[3],
            n[4],
            n[5],
        ])
    }

    /// Returns the algorithm number of `UUID`.
    pub const fn version(&self) -> Result<Version, &str> {
        match self.field_high_and_version >> 0xc {
            0x1 => Ok(Version::TIME),
            0x2 => Ok(Version::DCE),
            0x3 => Ok(Version::MD5),
            0x4 => Ok(Version::RAND),
            0x5 => Ok(Version::SHA1),
            _ => Err("Invalid version"),
        }
    }

    /// Returns the type field of `UUID`.
    pub fn variant(&self) -> Result<Variant, &str> {
        match (self.clock_seq_high_and_reserved >> 0x5) & 0x7 {
            0x0..=0x3 => Ok(Variant::NCS),
            0x5 | 0x4 => Ok(Variant::RFC4122),
            0x6 => Ok(Variant::MS),
            0x7 => Ok(Variant::FUT),
            _ => Err("Invalid variant"),
        }
    }
}

impl std::convert::Into<Bytes> for Layout {
    fn into(self) -> Bytes {
        let fl = self.field_low.to_le_bytes();
        let fm = self.field_mid.to_le_bytes();
        let fhv = self.field_high_and_version.to_le_bytes();
        let n = self.node.bytes();
        [
            fl[0],
            fl[1],
            fl[2],
            fl[3],
            fm[0],
            fm[1],
            fhv[0],
            fhv[1],
            self.clock_seq_high_and_reserved,
            self.clock_seq_low,
            n[0],
            n[1],
            n[2],
            n[3],
            n[4],
            n[5],
        ]
    }
}

impl fmt::Display for Layout {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let b = self.generate().0;
        write!(
                fmt,
                "{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
                b[0],
                b[1],
                b[2],
                b[3],
                b[4],
                b[5],
                b[6],
                b[7],
                b[8],
                b[9],
                b[10],
                b[11],
                b[12],
                b[13],
                b[14],
                b[15],
            )
    }
}

impl fmt::LowerHex for Layout {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let b = self.generate().0;
        write!(
                fmt,
                "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
                b[0],
                b[1],
                b[2],
                b[3],
                b[4],
                b[5],
                b[6],
                b[7],
                b[8],
                b[9],
                b[10],
                b[11],
                b[12],
                b[13],
                b[14],
                b[15],
            )
    }
}

impl fmt::UpperHex for Layout {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let b = self.generate().0;
        write!(
                fmt,
                "{:02X}{:02X}{:02X}{:02X}-{:02X}{:02X}-{:02X}{:02X}-{:02X}{:02X}-{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}",
                b[0],
                b[1],
                b[2],
                b[3],
                b[4],
                b[5],
                b[6],
                b[7],
                b[8],
                b[9],
                b[10],
                b[11],
                b[12],
                b[13],
                b[14],
                b[15],
            )
    }
}

/// The `UUID` format is 16 octets.
pub type Bytes = [u8; 16];

/// Is a 128-bit number used to identify information in computer systems.
#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub struct UUID(Bytes);

impl UUID {
    /// Returns the memory representation of `UUID`.
    pub const fn as_bytes(&self) -> [u8; 16] {
        self.0
    }

    /// UUID namespace for domain name system (DNS).
    pub const NAMESPACE_DNS: UUID = UUID([
        0x6b, 0xa7, 0xb8, 0x10, 0x9d, 0xad, 0x11, 0xd1, 0x80, 0xb4, 0x00, 0xc0, 0x4f, 0xd4, 0x30,
        0xc8,
    ]);

    /// UUID namespace for ISO object identifiers (OIDs).
    pub const NAMESPACE_OID: UUID = UUID([
        0x6b, 0xa7, 0xb8, 0x12, 0x9d, 0xad, 0x11, 0xd1, 0x80, 0xb4, 0x00, 0xc0, 0x4f, 0xd4, 0x30,
        0xc8,
    ]);

    /// UUID namespace for uniform resource locators (URLs).
    pub const NAMESPACE_URL: UUID = UUID([
        0x6b, 0xa7, 0xb8, 0x11, 0x9d, 0xad, 0x11, 0xd1, 0x80, 0xb4, 0x00, 0xc0, 0x4f, 0xd4, 0x30,
        0xc8,
    ]);

    /// UUID namespace for X.500 distinguished names (DNs).
    pub const NAMESPACE_X500: UUID = UUID([
        0x6b, 0xa7, 0xb8, 0x14, 0x9d, 0xad, 0x11, 0xd1, 0x80, 0xb4, 0x00, 0xc0, 0x4f, 0xd4, 0x30,
        0xc8,
    ]);

    // Parse `UUID` from string of hex digits.
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

        Ok(Layout {
            field_low: u32::from_le_bytes([bytes[3], bytes[2], bytes[1], bytes[0]]),
            field_mid: u16::from_le_bytes([bytes[5], bytes[4]]),
            field_high_and_version: u16::from_le_bytes([bytes[7], bytes[6]]),
            clock_seq_high_and_reserved: bytes[8],
            clock_seq_low: bytes[9],
            node: MacAddress::new([
                bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15],
            ]),
        })
    }
}
impl fmt::Display for UUID {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let b = self.0;
        write!(
                fmt,
                "{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
                b[0],
                b[1],
                b[2],
                b[3],
                b[4],
                b[5],
                b[6],
                b[7],
                b[8],
                b[9],
                b[10],
                b[11],
                b[12],
                b[13],
                b[14],
                b[15],
            )
    }
}

impl fmt::LowerHex for UUID {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let b = self.0;
        write!(
            fmt,
            "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            b[0],
            b[1],
            b[2],
            b[3],
            b[4],
            b[5],
            b[6],
            b[7],
            b[8],
            b[9],
            b[10],
            b[11],
            b[12],
            b[13],
            b[14],
            b[15],
        )
    }
}

impl fmt::UpperHex for UUID {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let b = self.0;
        write!(
            fmt,
            "{:02X}{:02X}{:02X}{:02X}-{:02X}{:02X}-{:02X}{:02X}-{:02X}{:02X}-{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}",
            b[0],
            b[1],
            b[2],
            b[3],
            b[4],
            b[5],
            b[6],
            b[7],
            b[8],
            b[9],
            b[10],
            b[11],
            b[12],
            b[13],
            b[14],
            b[15],
        )
    }
}

/// Represents the algorithm use for building the `Layout`, located in
/// the most significant 4 bits of `TimeStamp`.
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
    /// The name-based version specified in `rfc4122`document that uses SHA1 hashing.
    SHA1,
}

/// Is a type field determines the layout of `UUID`.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Variant {
    /// Reserved, NCS backward compatibility.
    NCS = 0,
    /// The variant specified in `rfc4122` document.
    RFC4122,
    /// Reserved, Microsoft Corporation backward compatibility.
    MS,
    /// Reserved for future definition.
    FUT,
}

/// Used to avoid duplicates that could arise when the clock is set backwards in time
/// or to fill in timestamp digits beyond the computer's measurement accuracy.
pub struct ClockSeq(pub u16);

impl ClockSeq {
    pub fn new(&self) -> u16 {
        AtomicU16::new(self.0).fetch_add(1, atomic::Ordering::SeqCst)
    }
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
            clock_seq_high_and_reserved: ($b8 & 0xf) as u8 | (Variant::RFC4122 as u8) << 4,
            clock_seq_low: $b9,
            node: crate::Cell::new(MacAddress::new([$b10, $b11, $b12, $b13, $b14, $b15])),
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Write;

    static UUID_TEST: [&str; 5] = [
        "ab720268-b83f-11ec-b909-0242ac120002",
        "000003e8-c22b-21ec-bd01-d4bed9408ecc",
        "17d040bd-44d6-3dab-8c26-724978b6a91d",
        "6a665038-24cf-4cf6-9b61-05f0c2fc6c08",
        "498f67a8-ebc5-5299-aa33-7c358b9c60e8",
    ];

    static VERSION_TEST: [Version; 5] = [
        Version::TIME,
        Version::DCE,
        Version::MD5,
        Version::RAND,
        Version::SHA1,
    ];

    macro_rules! check {
        ($buf:ident, $format:expr, $target:expr, $len:expr, $cond:expr) => {
            $buf.clear();
            write!($buf, $format, $target).unwrap();
            assert!($buf.len() == $len);
            assert!($buf.chars().all($cond), "{}", $buf);
        };
    }

    #[test]
    fn uuid_default() {
        let uuid = UUID::default();
        assert_eq!(uuid, UUID([0; 16]));
    }

    #[test]
    fn uuid_compare() {
        let uuid1 = UUID::from_str(UUID_TEST[0]).unwrap();
        let uuid2 = UUID::from_str(UUID_TEST[1]).unwrap();

        assert_eq!(uuid1, uuid1);
        assert_eq!(uuid2, uuid2);

        assert_ne!(uuid1, uuid2);
        assert_ne!(uuid2, uuid1);
    }

    #[test]
    fn uuid_formatter() {
        let uuid = UUID::from_str(UUID_TEST[0]).unwrap();
        let s = uuid.to_string();
        let mut buffer = String::new();

        assert_eq!(s, uuid.to_string());

        check!(buffer, "{}", uuid, 32, |c| c.is_lowercase()
            || c.is_digit(10)
            || c == '-');

        check!(buffer, "{:x}", uuid, 36, |c| c.is_lowercase()
            || c.is_digit(10)
            || c == '-');

        check!(buffer, "{:X}", uuid, 36, |c| c.is_uppercase()
            || c.is_digit(10)
            || c == '-');
    }

    #[test]
    fn parse_uuid_string() {
        for i in 0..5 {
            assert_eq!(
                UUID::from_str(UUID_TEST[i]).unwrap().version(),
                Ok(VERSION_TEST[i])
            );
            assert_eq!(
                UUID::from_str(UUID_TEST[i]).unwrap().variant(),
                Ok(Variant::RFC4122)
            );
        }
    }

    #[test]
    fn convert_layout_into_raw_bytes() {
        let mut uuid;
        for i in 0..5 {
            uuid = UUID::from_str(UUID_TEST[i]).unwrap();
            let into: Bytes = uuid.into();
            assert_eq!(into, uuid.clone().generate().as_bytes())
        }
    }

    #[test]
    fn from_raw_bytes() {
        let mut bytes: Bytes;
        let mut uuid: Layout;
        for i in 0..5 {
            bytes = UUID::from_str(UUID_TEST[i]).unwrap().into();
            uuid = Layout::from_raw_bytes(bytes);
            assert_eq!(uuid.version().unwrap(), VERSION_TEST[i])
        }
    }
}
