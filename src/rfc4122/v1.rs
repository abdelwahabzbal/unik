use mac_address::MacAddress;

use crate::{layout, Layout, TimeStamp, Variant, Version, UUID};

impl Layout {
    /// Returns integral number of `Timestamp` where `UUID` generated in.
    pub fn get_timestamp(&self) -> u64 {
        self.field_low as u64
            | (self.field_mid as u64) << 32
            | ((self.field_high_and_version as u64 >> 4) & 0xff) << 48
    }
}

impl UUID {
    /// Returns the `Layout` for `UUID` generated from `MacAddress` and `Timestamp`.
    pub fn v1(time: TimeStamp, node: MacAddress) -> Layout {
        layout!(
            time.0.to_le_bytes()[0],
            time.0.to_le_bytes()[1],
            time.0.to_le_bytes()[2],
            time.0.to_le_bytes()[3],
            time.0.to_le_bytes()[4],
            time.0.to_le_bytes()[5],
            time.0.to_le_bytes()[6],
            (Version::TIME as u8) << 4,
            crate::clock_seq_high_and_reserved().to_le_bytes()[0],
            crate::clock_seq_high_and_reserved().to_le_bytes()[1],
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
}
