#![cfg(feature = "v1")]
#![doc(cfg(feature = "v1"))]

use crate::{Layout, Node, Timestamp, Variant, Version, UUID};

impl<'a> Layout {
    fn from_fields(ts: &Timestamp, clock_seq: (u8, u8), node: Node) -> Self {
        let version = Version::TIME;

        Self {
            timastamp: Some(ts.0),
            version: version,
            variant: Variant::RFC,
            field_low: (ts.0 & 0xffff_ffff) as u32,
            field_mid: ((ts.0 >> 32 & 0xffff) as u16),
            field_high_and_version: (ts.0 >> 48 & 0xfff) as u16 | (version as u16) << 12,
            clock_seq_high_and_reserved: clock_seq.0,
            clock_seq_low: clock_seq.1,
            node: node,
        }
    }
}

impl UUID {
    pub fn v1<'a>(ts: &'a Timestamp, node: Node) -> Layout {
        Layout::from_fields(
            ts,
            crate::clock_seq_high_and_reserved(Variant::RFC as u8),
            node,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uuid_new_v1() {
        let layout = UUID::v1(&Timestamp(1234_5678), Node([u8::MAX; 6]));

        assert_eq!(layout.timastamp, Some(1234_5678));
        assert_eq!(layout.version, Version::TIME);
        assert_eq!(layout.variant, Variant::RFC);

        let cloned = layout.clone();
        assert!(cloned == layout)
    }
}
