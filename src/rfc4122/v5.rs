use crate::{layout, Layout, MacAddress, Version, UUID};

impl UUID {
    /// Create `UUID` by hashing a namespace identifier and name using SHA1 algorithm.
    pub fn v5<'a>(data: &str, ns: UUID) -> Layout {
        let hash = md5::compute(format!("{:x}", ns) + data).0;
        layout!(
            hash[0],
            hash[1],
            hash[2],
            hash[3],
            hash[4],
            hash[5],
            hash[6],
            ((Version::SHA1 as u8) << 0x4) | (hash[7] & 0xf),
            hash[8],
            hash[9],
            hash[10],
            hash[11],
            hash[12],
            hash[13],
            hash[14],
            hash[15]
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Variant;

    #[test]
    fn new_uuid_using_md5() {
        let nss = [
            UUID::NAMESPACE_DNS,
            UUID::NAMESPACE_OID,
            UUID::NAMESPACE_URL,
            UUID::NAMESPACE_X500,
        ];

        for &ns in nss.iter() {
            assert_eq!(UUID::v5("test", ns).version(), Ok(Version::SHA1));
            assert_eq!(UUID::v5("test", ns).variant(), Ok(Variant::RFC4122));
        }
    }
}
