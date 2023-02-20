#![cfg(feature = "rand")]

use crate::{layout, Layout, Version, UUID};

use nanorand::{Rng, WyRand};

impl UUID {
    /// Creates a random `UUID`.
    pub fn v4() -> Layout {
        let rand = get_random().to_le_bytes();
        layout!(
            rand[0],
            rand[1],
            rand[2],
            rand[3],
            rand[4],
            rand[5],
            ((Version::RAND as u8) << 0x4) | (rand[6] & 0xf),
            rand[7],
            rand[8],
            rand[9],
            rand[10],
            rand[11],
            rand[12],
            rand[13],
            rand[14],
            rand[15]
        )
    }
}

pub(crate) fn get_random() -> u128 {
    WyRand::new().generate::<u128>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Variant;

    #[test]
    fn uuid_from_random() {
        let uuid = UUID::v4().new();
        assert_eq!(uuid.get_version(), Ok(Version::RAND));
        assert_eq!(uuid.get_variant(), Ok(Variant::RFC4122));
    }
}
