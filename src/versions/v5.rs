use crate::{Layout, MacAddress, Variant, Version, UUID};

impl Layout {
    fn using_md5(hash: [u8; 16]) -> Self {
        Self {
            timestamp: None,
            field_low: ((hash[0] as u32) << 24)
                | (hash[1] as u32) << 16
                | (hash[2] as u32) << 8
                | hash[3] as u32,
            field_mid: (hash[4] as u16) << 8 | (hash[5] as u16),
            field_high_and_version: ((hash[6] as u16) << 8 | (hash[7] as u16)) & 0xfff
                | (Version::MD5 as u16) << 12,
            clock_seq_high_and_reserved: (hash[8] & 0xf) | (Variant::RFC as u8) << 4,
            clock_seq_low: hash[9] as u8,
            node: MacAddress::new([hash[10], hash[11], hash[12], hash[13], hash[14], hash[15]]),
        }
    }
}

impl UUID {
    pub fn v5<'a>(data: &str, ns: UUID) -> Layout {
        Layout::using_md5(md5::compute(format!("{:x}", ns) + data).0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_uuid_using_md5() {
        let namespace = [UUID::DNS, UUID::OID, UUID::URL, UUID::X500];

        for &ns in namespace.iter() {
            assert_eq!(UUID::v3("pack", ns).timestamp, None);
            assert_eq!(UUID::v5("hack", ns).version(), Ok(Version::MD5));
            assert_eq!(UUID::v5("jack", ns).variant(), Ok(Variant::RFC));
        }
    }
}
