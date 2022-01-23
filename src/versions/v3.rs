use sha1::Sha1;

use crate::{Layout, MacAddress, Variant, Version, UUID};

impl Layout {
    fn using_sha1(hash: [u8; 16]) -> Self {
        Self {
            timestamp: None,
            field_low: ((hash[0] as u32) << 24)
                | (hash[1] as u32) << 16
                | (hash[2] as u32) << 8
                | hash[3] as u32,
            field_mid: (hash[4] as u16) << 8 | (hash[5] as u16),
            field_high_and_version: ((hash[6] as u16) << 8 | (hash[7] as u16)) & 0xfff
                | (Version::SHA1 as u16) << 12,
            clock_seq_high_and_reserved: (hash[8] & 0xf) | (Variant::RFC as u8) << 4,
            clock_seq_low: hash[9] as u8,
            node: MacAddress::new([hash[10], hash[11], hash[12], hash[13], hash[14], hash[15]]),
        }
    }
}

impl UUID {
    pub fn v3<'a>(data: &str, ns: UUID) -> Layout {
        let hash = Sha1::from(format!("{:x}", ns) + data).digest().bytes()[..16]
            .try_into()
            .unwrap();

        Layout::using_sha1(hash)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_uuid_using_sha1() {
        let namespace = [UUID::DNS, UUID::OID, UUID::URL, UUID::X500];

        for &ns in namespace.iter() {
            assert_eq!(UUID::v3("pack", ns).timestamp, None);
            assert_eq!(UUID::v3("hack", ns).version(), Ok(Version::SHA1));
            assert_eq!(UUID::v3("jack", ns).variant(), Ok(Variant::RFC));
        }
    }
}
