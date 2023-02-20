#![cfg(any(feature = "utc", feature = "rand"))]

use crate::{layout, Layout, Version, UUID};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Domain {
    PERSON = 0,
    GROUP,
    ORG,
}

impl UUID {
    pub fn get_domain(&self) -> Result<Domain, &str> {
        match self.0[7] {
            0 => Ok(Domain::PERSON),
            1 => Ok(Domain::GROUP),
            2 => Ok(Domain::ORG),
            _ => Err("Invalid domain name"),
        }
    }
}

impl UUID {
    pub fn v2(domain: Domain) -> Layout {
        let i: [u8; 4];

        #[cfg(all(windows))]
        {
            i = unsafe { libc::getpid() as u32 }.to_le_bytes();
        }

        #[cfg(all(unix))]
        {
            i = {
                if domain == Domain::PERSON {
                    unsafe { libc::getuid() }.to_le_bytes()
                } else if domain == Domain::GROUP {
                    unsafe { libc::getgid() }.to_le_bytes()
                } else {
                    panic!("Not impl'ed yet")
                }
            };
        }

        let mut bytes = UUID::v1().new().0;

        bytes[0..4].copy_from_slice(&i);
        bytes[6] = (Version::DCE as u8) << 4;
        bytes[9] = domain as u8;

        // Layout::from_raw_bytes(uuid)

        layout!(
            bytes[0],
            bytes[1],
            bytes[2],
            bytes[3],
            bytes[4],
            bytes[5],
            (Version::DCE as u8) << 0x4 | bytes[6] & 0xf,
            bytes[7],
            bytes[0],
            bytes[1],
            bytes[0],
            bytes[1],
            bytes[2],
            bytes[3],
            bytes[4],
            bytes[5]
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Variant;

    #[test]
    fn uuid_dce_org() {
        let uuid = UUID::v2(Domain::PERSON).new();
        assert_eq!(uuid.get_version(), Ok(Version::DCE));
        assert_eq!(uuid.get_variant(), Ok(Variant::RFC4122));
        assert_eq!(uuid.get_domain(), Ok(Domain::PERSON));
    }
}
