use sha1::Sha1;

use crate::{layout, Layout, Version, UUID};

impl UUID {
    /// Creates `UUID` by hashing a namespace identifier and name using MD5 algorithm.
    pub fn v3<'a>(data: &str, ns: UUID) -> Layout {
        let hash: [u8; 16] = Sha1::from(format!("{}", ns) + data).digest().bytes()[..16]
            .try_into()
            .unwrap();

        layout!(
            hash[0],
            hash[1],
            hash[2],
            hash[3],
            hash[4],
            hash[5],
            ((Version::MD5 as u8) << 0x4) | (hash[6] & 0xf),
            hash[7],
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
    fn uuid_using_hash_sha1() {
        let nss = [
            UUID::NAMESPACE_DNS,
            UUID::NAMESPACE_OID,
            UUID::NAMESPACE_URL,
            UUID::NAMESPACE_X500,
        ];

        for &ns in nss.iter() {
            let uuid = UUID::v3("test", ns).new();
            assert_eq!(uuid.get_version(), Ok(Version::MD5));
            assert_eq!(uuid.get_variant(), Ok(Variant::RFC4122));
        }
    }
}
