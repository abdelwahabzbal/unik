use mac_address::MacAddress;

use crate::{global_layout, Layout, Timestamp, Variant, Version, UUID};

impl UUID {
    pub fn v1(time: Timestamp, node: MacAddress) -> Layout {
        global_layout!(
            time.0.to_ne_bytes()[0],
            time.0.to_ne_bytes()[1],
            time.0.to_ne_bytes()[2],
            time.0.to_ne_bytes()[3],
            time.0.to_ne_bytes()[4],
            time.0.to_ne_bytes()[5],
            time.0.to_ne_bytes()[6],
            Version::TIME,
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
    fn uuid_new_v1() {
        let layout = UUID::v1(Timestamp(1234_5678), MacAddress::new([u8::MAX; 6]));

        assert_eq!(layout.get_timestamp(), 1234_5678_u64);
        assert_eq!(layout.get_version(), Ok(Version::TIME));
        assert_eq!(layout.get_variant(), Ok(Variant::RFC));

        let cloned = layout.clone();
        assert!(cloned == layout);
    }
}
