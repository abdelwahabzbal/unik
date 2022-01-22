#![doc(cfg(feature = "v4"))]
#![cfg(feature = "v4")]

use crate::{Layout, Node, Variant, Version, UUID};
// use rand_core::{OsRng, RngCore};

impl UUID {
    pub fn v4() -> Layout {
        // let mut key = [0u8; 128];
        // OsRng.fill_bytes(&mut key);
        //
        // let random_u64_round_1 = OsRng.next_u64();
        // let round_1 = random_u64_round_1.to_ne_bytes();
        //
        // let random_u64_round_2 = OsRng.next_u64();
        // let round_2 = random_u64_round_2.to_ne_bytes();

        use nanorand::{Rng, WyRand};
        let mut rng = WyRand::new();
        let bytes = rng.generate::<u128>().to_ne_bytes();

        Layout {
            timestamp: None,
            field_low: ((bytes[0] as u32) << 24)
                | (bytes[1] as u32) << 16
                | (bytes[2] as u32) << 8
                | bytes[3] as u32,
            field_mid: (bytes[4] as u16) << 8 | (bytes[5] as u16),
            field_high_and_version: ((bytes[6] as u16) << 8 | (bytes[7] as u16)) & 0xfff
                | (Version::RAND as u16) << 12,
            clock_seq_high_and_reserved: (bytes[0] & 0xf) | (Variant::RFC as u8) << 4,
            clock_seq_low: bytes[1] as u8,
            node: Node([bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]]),
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
        assert_eq!(uuid.version(), Ok(Version::RAND));
        assert_eq!(uuid.variant(), Ok(Variant::RFC));
    }
}
