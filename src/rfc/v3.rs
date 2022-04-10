use sha1::Sha1;

use crate::{layout, Layout, MacAddress, Variant, Version, UUID};

impl UUID {
    pub fn v3<'a>(data: &str, ns: UUID) -> Layout {
        let hash: [u8; 16] = Sha1::from(format!("{:x}", ns) + data).digest().bytes()[..16]
            .try_into()
            .unwrap();

        layout!(
            hash[0],
            hash[1],
            hash[2],
            hash[3],
            hash[4],
            hash[5],
            hash[6],
            Version::MD5,
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

    #[test]
    fn uuid_using_hash_sha1() {
        let namespace = [UUID::DNS, UUID::OID, UUID::URL, UUID::X500];

        for &ns in namespace.iter() {
            assert_eq!(UUID::v3("test", ns).get_version(), Ok(Version::MD5));
            assert_eq!(UUID::v3("test", ns).get_variant(), Ok(Variant::RFC));
        }
    }
}
