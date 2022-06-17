use chrono::Utc;
use mac_address::MacAddress;
use nanorand::{Rng, WyRand};

use crate::{layout, ClockSeq, Layout, Node, TimeStamp, Variant, Version, UUID};

impl Layout {
    /// Returns integral number of `TimeStamp` where `UUID` generated in.
    pub fn get_timestamp(&self) -> u64 {
        self.field_low as u64
            | (self.field_mid as u64) << 32
            | ((self.field_high_and_version as u64 >> 4) & 0xff) << 48
    }

    /// Returns the integral form for `TimeStamp`, `ClockSeq`, `Node` fields of `UUID`.
    pub fn as_fields(&self) -> (u64, u16, u64) {
        (
            self.get_timestamp(),
            u16::from_le_bytes(
                ((self.clock_seq_high_and_reserved as u16) << 8 | self.clock_seq_low as u16)
                    .to_le_bytes(),
            ),
            u64::from_le_bytes([
                (self.node.bytes()[0]),
                (self.node.bytes()[1]),
                (self.node.bytes()[2]),
                (self.node.bytes()[3]),
                (self.node.bytes()[4]),
                (self.node.bytes()[5]),
                /* Make `from_le_bytes` happy
                 */
                0,
                0,
            ]),
        )
    }
}

impl UUID {
    /// Returns the `Layout` for `UUID` generated from `Node` and `TimeStamp`.
    pub fn v1(time: TimeStamp, node: Node) -> Layout {
        layout!(
            time.0.to_le_bytes()[0],
            time.0.to_le_bytes()[1],
            time.0.to_le_bytes()[2],
            time.0.to_le_bytes()[3],
            time.0.to_le_bytes()[4],
            time.0.to_le_bytes()[5],
            time.0.to_le_bytes()[6],
            (Version::TIME as u8) << 4,
            clock_seq_high_and_reserved().to_le_bytes()[0],
            clock_seq_high_and_reserved().to_le_bytes()[1],
            node.bytes()[0],
            node.bytes()[1],
            node.bytes()[2],
            node.bytes()[3],
            node.bytes()[4],
            node.bytes()[5]
        )
    }
}

impl TimeStamp {
    pub fn from_unix() -> u64 {
        Utc::now()
            .checked_sub_signed(chrono::Duration::nanoseconds(0x01B2_1DD2_1381_4000))
            .unwrap()
            .timestamp_nanos() as u64
    }

    /// .
    pub fn from_utc() -> u64 {
        Utc::now().timestamp_nanos() as u64
    }
}

pub(crate) fn clock_seq_high_and_reserved() -> u16 {
    ClockSeq::new(WyRand::new().generate::<u16>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uuid_with_predef_timestamp() {
        let layout = UUID::v1(TimeStamp(1234_5678_u64), MacAddress::new([u8::MIN; 6]));
        assert_eq!(layout.get_timestamp(), 1234_5678_u64);
        assert_eq!(layout.get_version(), Ok(Version::TIME));
        assert_eq!(layout.get_variant(), Ok(Variant::RFC));
    }

    #[test]
    fn get_timestamp_from_parsed_string() {
        let layout = UUID::v1(TimeStamp(1234_5678_u64), MacAddress::new([u8::MIN; 6]));

        assert_eq!(
            1234_5678_u64,
            UUID::from_str(format!("{:x}", layout.generate()).as_str())
                .unwrap()
                .get_timestamp()
        );
    }

    #[test]
    fn layout_from_bytes() {
        let layout = UUID::v1(TimeStamp(1234_5678_u64), MacAddress::new([u8::MIN; 6])).generate();
        let layout = Layout::from_bytes(layout.0);
        assert_eq!(layout.get_version(), Ok(Version::TIME));
        assert_eq!(layout.get_variant(), Ok(Variant::RFC));
        assert_eq!(layout.get_timestamp(), 1234_5678_u64);
        assert_eq!(layout.node, MacAddress::new([u8::MIN; 6]));
    }
}
