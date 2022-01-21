#![doc(cfg(feature = "v4"))]
#![cfg(feature = "v4")]

use crate::{Layout, Node, Variant, Version, UUID};
use rand_core::{OsRng, RngCore};

impl UUID {
    pub fn v4() -> Layout {
        let mut key = [0u8; 128];
        OsRng.fill_bytes(&mut key);

        let random_u64_round_1 = OsRng.next_u64();
        let round_1 = random_u64_round_1.to_ne_bytes();

        let random_u64_round_2 = OsRng.next_u64();
        let round_2 = random_u64_round_2.to_ne_bytes();

        let version = Version::RAND;

        Layout {
            timestamp: None,
            version,
            variant: Variant::RFC,
            field_low: ((round_1[0] as u32) << 24)
                | (round_1[1] as u32) << 16
                | (round_1[2] as u32) << 8
                | round_1[3] as u32,
            field_mid: (round_1[4] as u16) << 8 | (round_1[5] as u16),
            field_high_and_version: ((round_1[6] as u16) << 8 | (round_1[7] as u16)) & 0xfff
                | (version as u16) << 12,
            clock_seq_high_and_reserved: (round_2[0] & 0xf) | (Variant::RFC as u8) << 4,
            clock_seq_low: round_2[1] as u8,
            node: Node([
                round_2[2], round_2[3], round_2[4], round_2[5], round_2[6], round_2[7],
            ]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_from_rand() {
        let uuid = UUID::v4();

        assert_eq!(uuid.timestamp, None);
        assert_eq!(uuid.version, Version::RAND);
        assert_eq!(uuid.variant, Variant::RFC);
    }
}
