use mac_address::MacAddress;

use crate::{layout, Layout, Timestamp, Variant, Version, UUID};

impl Layout {
    /// Get timestamp where `UUID` generated in.
    pub fn get_timestamp(&self) -> u64 {
        self.field_low as u64
            | (self.field_mid as u64) << 0x20
            | ((self.field_high_and_version as u64 >> 0x04) & 0xff) << 0x30
    }
}

impl UUID {
    pub fn v1(time: Timestamp, node: MacAddress) -> Layout {
        layout!(
            time.0.to_ne_bytes()[0],
            time.0.to_ne_bytes()[1],
            time.0.to_ne_bytes()[2],
            time.0.to_ne_bytes()[3],
            time.0.to_ne_bytes()[4],
            time.0.to_ne_bytes()[5],
            time.0.to_ne_bytes()[6],
            (Version::TIME as u8) << 4,
            crate::clock_seq_high_and_reserved().to_ne_bytes()[0],
            crate::clock_seq_high_and_reserved().to_ne_bytes()[1],
            node.bytes()[0],
            node.bytes()[1],
            node.bytes()[2],
            node.bytes()[3],
            node.bytes()[4],
            node.bytes()[5]
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uuid_with_predefined_timestamp() {
        let layout = UUID::v1(Timestamp(1234_5678_u64), MacAddress::new([u8::MIN; 6]));

        assert_eq!(layout.get_timestamp(), 1234_5678_u64);
        assert_eq!(layout.get_version(), Ok(Version::TIME));
        assert_eq!(layout.get_variant(), Ok(Variant::RFC));
    }
}
