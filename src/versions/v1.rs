#![cfg(feature = "v1")]
// #![doc(cfg(feature = "v1"))]

use mac_address::MacAddress;

use crate::{Layout, Timestamp, Variant, Version, UUID};

impl Layout {
    fn from_fields(ts: Timestamp, clock_seq: u16, node: MacAddress) -> Self {
        Self {
            timestamp: Some(ts.0),
            field_low: (ts.0 & 0xffff_ffff) as u32,
            field_mid: ((ts.0 >> 32 & 0xffff) as u16),
            field_high_and_version: (ts.0 >> 48 & 0xfff) as u16 | (Version::TIME as u16) << 12,
            clock_seq_high_and_reserved: (clock_seq >> 8 & 0xf) as u8 | (Variant::RFC as u8) << 4,
            clock_seq_low: (clock_seq & 0xff) as u8,
            node: node,
        }
    }
}

impl UUID {
    pub fn v1(ts: Timestamp, node: MacAddress) -> Layout {
        Layout::from_fields(ts, crate::clock_seq_high_and_reserved(), node)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uuid_new_v1() {
        let layout = UUID::v1(Timestamp(1234_5678), MacAddress::new([u8::MAX; 6]));

        assert_eq!(layout.timestamp, Some(1234_5678));
        assert_eq!(layout.version(), Ok(Version::TIME));
        assert_eq!(layout.variant(), Ok(Variant::RFC));

        let cloned = layout.clone();
        assert!(cloned == layout)
    }
}
