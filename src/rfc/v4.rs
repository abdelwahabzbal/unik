use crate::{get_random, layout, Layout, MacAddress, Variant, Version, UUID};

impl UUID {
    pub fn v4() -> Layout {
        let rand = get_random().to_ne_bytes();
        layout!(
            rand[0],
            rand[1],
            rand[2],
            rand[3],
            rand[4],
            rand[5],
            rand[6],
            (Version::RAND as u8) << 4,
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
    fn uuid_from_random() {
        let uuid = UUID::v4();

        assert_eq!(uuid.get_version(), Ok(Version::RAND));
        assert_eq!(uuid.get_variant(), Ok(Variant::RFC));
    }
}
