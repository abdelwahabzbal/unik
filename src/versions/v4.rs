use crate::{global_layout, Layout, MacAddress, Variant, Version, UUID};

use nanorand::{Rng, WyRand};

impl UUID {
    pub fn v4() -> Layout {
        let mut rng = WyRand::new();
        let rand = rng.generate::<u128>().to_ne_bytes();

        global_layout!(
            rand[0],
            rand[1],
            rand[2],
            rand[3],
            rand[4],
            rand[5],
            rand[6],
            Version::RAND,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_from_rand() {
        let uuid = UUID::v4();

        // assert_eq!(uuid.timestamp, None);
        assert_eq!(uuid.get_version(), Ok(Version::RAND));
        assert_eq!(uuid.get_variant(), Ok(Variant::RFC));
    }
}
