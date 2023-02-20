//! This crate defines a powerful uniform resource name namespace for UUIDs
//! (Universally Unique Identifier), which are suited for modern use.
//!
//! This lib can be used to create unique and reasonably short values without
//! requiring extra knowledge.
//!
//! A [`UUID`] is 128 bits long, and can guarantee uniqueness across space and time.

#![doc(html_root_url = "https://docs.rs/unik")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![feature(decl_macro)]
#![feature(box_syntax)]
#![feature(strict_provenance)]

pub mod rfc4122;

use core::fmt;
use std::{convert, sync::Mutex};

#[cfg(feature = "utc")]
pub use chrono::Utc;

/// The `IEEE-802` network address.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Node(pub [u8; 6]);

impl fmt::Display for Node {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            fmt,
            "{:02x}-{:02x}-{:02x}-{:02x}-{:02x}-{:02x}",
            self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5],
        )
    }
}

impl std::default::Default for Node {
    #[allow(unreachable_code)]
    fn default() -> Self {
        #[cfg(feature = "mac")]
        return Self::from(Node(
            mac_address::get_mac_address().unwrap().unwrap().bytes(),
        ));

        #[cfg(feature = "rand")]
        {
            use nanorand::Rng;
            let buf: [u8; 6] = [0u8; 6];
            nanorand::ChaCha::<6>::new().fill_bytes(buf);
            return Self::from(Node(buf));
        }

        Self::from(Node([0u8; 6]))
    }
}

impl From<[u8; 6]> for Node {
    fn from(node: [u8; 6]) -> Self {
        Node(node)
    }
}

/// Is a coordinated Universal Time (UTC).
#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Timestamp {
    UTC(u64),
}

impl Timestamp {
    /// Returns [`Timestamp`] value.
    pub fn get(&self) -> u64 {
        match self {
            Timestamp::UTC(t) => *t,
        }
    }
}

impl std::default::Default for Timestamp {
    #[allow(unreachable_code)]
    fn default() -> Self {
        #[cfg(feature = "utc")]
        return Self::UTC(Utc::now().timestamp_nanos() as u64);

        #[cfg(feature = "rand")]
        {
            use nanorand::Rng;

            let buf = [0u8; 8];
            nanorand::ChaCha::<6>::new().fill_bytes(buf);
            return Self::UTC(u64::from_le_bytes(buf));
        }

        Self::UTC(u64::MAX)
    }
}

/// The simplified version of [`UUID`] in terms of fields that are integral numbers of octets.
pub struct Layout {
    timestamp: Timestamp,
    clock_seq: ClockSeq,
    node: Node,
}

impl Layout {
    /// Returns [`Layout`] from sequence of integral numbers.
    pub fn from_raw_bytes(uuid: UUID) -> UUID {
        let bytes = u128::from_le_bytes(uuid.0).to_le_bytes();
        UUID([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15],
        ])
    }

    /// New instance of [`UUID`].
    pub fn new(&mut self) -> UUID {
        let ts = self.timestamp.get().to_le_bytes();
        let cs = self.clock_seq.to_le_bytes();
        let n = self.node.0;

        UUID([
            ts[0], ts[1], ts[2], ts[3], ts[4], ts[5], ts[6], ts[7], cs[0], cs[1], n[0], n[1], n[2],
            n[3], n[4], n[5],
        ])
    }
}

impl convert::From<UUID> for Layout {
    fn from(uuid: UUID) -> Self {
        layout!(uuid)
    }
}

impl convert::From<Timestamp> for Layout {
    fn from(ts: Timestamp) -> Self {
        Self {
            timestamp: ts,
            clock_seq: ClockSeq::default(),
            node: Node::default(),
        }
    }
}

impl convert::From<Node> for Layout {
    fn from(node: Node) -> Self {
        Self {
            timestamp: Timestamp::UTC(0),
            clock_seq: ClockSeq::default(),
            node: node,
        }
    }
}

/// A Universally Unique Identifier [`UUID`].
#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub struct UUID(pub [u8; 16]);

impl UUID {
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

    /// Returns the algorithm number of [`UUID`].
    ///
    /// See [`Version`] .
    pub fn get_version(&self) -> Result<Version, &str> {
        match self.0[6] >> 4 {
            0x1 => Ok(Version::TIME),
            0x2 => Ok(Version::DCE),
            0x3 => Ok(Version::MD5),
            0x4 => Ok(Version::RAND),
            0x5 => Ok(Version::SHA1),
            _ => Err("Invalid version"),
        }
    }

    /// Returns the type field of [`UUID`].
    ///
    /// See [`Variant`]
    pub fn get_variant(&self) -> Result<Variant, &str> {
        match (self.0[8] >> 0x5) & 0x7 {
            0x0..=0x3 => Ok(Variant::NCS),
            0x4 | 0x5 => Ok(Variant::RFC4122),
            0x6 => Ok(Variant::MS),
            0x7 => Ok(Variant::FUT),
            _ => Err("Invalid variant"),
        }
    }

    /// Returns the [`Node`] field.
    ///
    /// See [`Node`]
    pub fn get_node(&self) -> Node {
        let node = self.0;
        [node[10], node[11], node[12], node[13], node[14], node[15]].into()
    }

    /// Parse [`UUID`] from string of hex digits.
    pub fn parse(us: &str) -> Result<UUID, &str> {
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

        Ok(UUID([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15],
        ]))
    }
}

impl convert::From<[u8; 16]> for UUID {
    fn from(bytes: [u8; 16]) -> Self {
        UUID(bytes)
    }
}

impl convert::From<UUID> for [u8; 16] {
    fn from(bytes: UUID) -> Self {
        bytes.0
    }
}

impl convert::From<u128> for UUID {
    fn from(bytes: u128) -> Self {
        UUID(bytes.to_le_bytes())
    }
}

impl fmt::Display for UUID {
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

/// Represent the algorithm use for building the [`Layout`], located in
/// the most significant 4 bits of [`Timestamp`].
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

/// Type field determines the layout of [`UUID`].
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

/// Ensure uniqueness.
pub struct ClockSeq {
    clk_seq_hi_res: u8,
    clk_seq_low: Mutex<u8>,
}

impl ClockSeq {
    pub fn new() -> Self {
        Self {
            clk_seq_hi_res: u8::default(),
            clk_seq_low: Mutex::default(),
        }
    }

    pub fn to_le_bytes(&mut self) -> [u8; 2] {
        u16::from_le_bytes([
            self.clk_seq_hi_res << 7,
            *self.clk_seq_low.get_mut().unwrap(),
        ])
        .to_le_bytes()
    }
}

impl Default for ClockSeq {
    fn default() -> Self {
        Self {
            clk_seq_hi_res: u8::default(),
            clk_seq_low: Mutex::default(),
        }
    }
}

pub(crate) macro layout {
    ($b0:expr, $b1:expr, $b2:expr, $b3:expr,
                $b4:expr, $b5:expr, $b6:expr, $b7:expr,
                $b8:expr, $b9:expr, $b10:expr, $b11:expr,
                $b12:expr, $b13:expr, $b14:expr, $b15:expr
    ) => {
        Layout {
            timestamp:  Timestamp::UTC($b0 as u64 | ($b1 as u64) << 8 | ($b2 as u64) << 16
            | ($b3 as u64) << 24 | ($b4 as u64) << 32 | ($b5 as u64) << 40
            | ($b6 as u64) << 48 | ($b7 as u64) << 56),
            clock_seq: ClockSeq{clk_seq_hi_res: $b8 | Variant::RFC4122 as u8,
                clk_seq_low: Mutex::new($b9)},
            node: $crate::Node::from([$b10, $b11, $b12, $b13, $b14, $b15]),
        }
    },
    ($arr:tt) => {
        self::layout!($arr.0[0], $arr.0[1], $arr.0[2], $arr.0[3],
            $arr.0[4], $arr.0[5], $arr.0[6], $arr.0[7], $arr.0[8],
            $arr.0[9], $arr.0[10], $arr.0[11], $arr.0[12], $arr.0[13], $arr.0[14],
            $arr.0[15])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Write;

    static UUIDS: [&str; 5] = [
        "ab720268-b83f-11ec-b909-0242ac120002",
        "000003e8-c22b-21ec-bd01-d4bed9408ecc",
        "17d040bd-44d6-3dab-8c26-724978b6a91d",
        "6a665038-24cf-4cf6-9b61-05f0c2fc6c08",
        "498f67a8-ebc5-5299-aa33-7c358b9c60e8",
    ];

    const VERSIONS: [Version; 5] = [
        Version::TIME,
        Version::DCE,
        Version::MD5,
        Version::RAND,
        Version::SHA1,
    ];

    macro check($buf:ident, $format:expr, $target:expr, $len:expr, $cond:expr) {
        $buf.clear();
        write!($buf, $format, $target).unwrap();
        assert!($buf.len() == $len);
        assert!($buf.chars().all($cond), "{}", $buf);
    }

    #[test]
    fn uuid_default_value() {
        let uuid = UUID::default();
        assert_eq!(uuid, UUID([0; 16]));
        assert_eq!(uuid.0, [0u8; 16]);
    }

    #[test]
    fn uuid_convert() {
        let uuid = UUID::default();
        let into: [u8; 16] = uuid.into();
        assert_eq!(into, [0u8; 16]);

        let uuid = UUID::from([0u8; 16]);
        assert_eq!(uuid, UUID::default())
    }

    #[test]
    fn uuid_derive() {
        let default = UUID::default();
        let from = UUID::from([u8::MAX; 16]);

        assert_eq!(default, default);
        assert_eq!(from, from);

        assert_ne!(default, from);
        assert_ne!(from, default);

        let copy = default;
        assert_eq!(copy, UUID([0u8; 16]));

        let clone = default.clone();
        assert_eq!(clone, UUID([0u8; 16]));
    }

    #[test]
    fn uuid_format() {
        let uuid = UUID::default();
        let mut buffer = String::new();

        check!(buffer, "{}", uuid, 36, |c| c.is_lowercase()
            || c.is_digit(10)
            || c == '-');
        check!(buffer, "{}", uuid.to_string().to_lowercase(), 36, |c| c
            .is_lowercase()
            || c.is_digit(10)
            || c == '-');
        check!(buffer, "{}", uuid.to_string().to_uppercase(), 36, |c| c
            .is_uppercase()
            || c.is_digit(10)
            || c == '-');
    }

    #[cfg(feature = "utc")]
    #[test]
    fn layout_from_timstamp() {
        let mut utc = Timestamp::UTC(0x1234);
        let mut layout = Layout::from(utc);
        assert_eq!(layout.timestamp.get(), 0x1234);
        assert_eq!(layout.timestamp.get().to_le_bytes()[0], 0x34);
        assert_eq!(layout.timestamp.get().to_le_bytes()[1], 0x12);

        utc = Timestamp::UTC(u64::MIN);
        layout = Layout::from(utc);
        assert_eq!(layout.timestamp.get(), u64::MIN);

        utc = Timestamp::UTC(u64::MAX);
        layout = Layout::from(utc);
        assert_eq!(layout.timestamp.get(), u64::MAX)
    }

    #[test]
    fn layout_from_node() {
        let layout = Layout::from(Node::from([u8::MIN; 6]));
        assert_eq!(layout.node.0, [u8::MIN; 6]);

        let layout = Layout::from(Node::from([u8::MAX; 6]));
        assert_eq!(layout.node.0, [u8::MAX; 6])
    }

    #[test]
    fn convert_layout_into_raw_bytes() {
        let mut uuid;
        for i in 0..4 {
            uuid = UUID::parse(UUIDS[i]).unwrap();
            assert_eq!(uuid.get_version(), Ok(VERSIONS[i]));
            assert_eq!(uuid.get_variant(), Ok(Variant::RFC4122))
        }
    }

    #[test]
    fn from_raw_bytes() {
        let uuid = UUID::parse("ab720268-b83f-41ec-b909-0242ac120002").unwrap();
        println!("{:?}", uuid.get_version());
        assert_eq!(uuid.get_version(), Ok(Version::RAND));
        assert_eq!(uuid.get_variant().unwrap(), Variant::RFC4122);
    }
}
