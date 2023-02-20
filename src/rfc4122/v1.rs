#![cfg(any(feature = "utc", feature = "rand"))]

use crate::{layout, ClockSeq, Layout, Node, Timestamp, Version, UUID};

impl UUID {
    /// Returns the [`Layout`] generated from [`Node`] and [`Timestamp`].
    pub fn v1() -> Layout {
        let ts = [0u8; 8];
        #[cfg(any(feature = "utc", feature = "rand"))]
        {
            let ts = Timestamp::default().get().to_le_bytes();
        }

        let cshr = ClockSeq::new().to_le_bytes();

        let n = [0u8; 6];
        #[cfg(feature = "rand")]
        {
            let n = Node::default().0;
        }

        layout!(
            ts[0],
            ts[1],
            ts[2],
            ts[3],
            ts[4],
            ts[5],
            ((Version::TIME as u8) << 0x4) | (ts[6] & 0xf),
            ts[7],
            cshr[0],
            cshr[1],
            n[0],
            n[1],
            n[2],
            n[3],
            n[4],
            n[5]
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Variant;

    #[test]
    fn uuid_default() {
        let uuid = UUID::v1().new();
        assert_eq!(uuid.get_version(), Ok(Version::TIME));
        assert_eq!(uuid.get_variant(), Ok(Variant::RFC4122));
    }

    #[test]
    fn layout_from_raw_bytes() {
        let uuid = UUID::v1().new();
        let layout = Layout::from_raw_bytes(uuid);

        assert_eq!(layout.get_version(), Ok(Version::TIME));
        assert_eq!(layout.get_variant(), Ok(Variant::RFC4122));
    }
}
