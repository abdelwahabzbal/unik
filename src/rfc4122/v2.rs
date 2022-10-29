use crate::{Layout, Version, UUID};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Domain {
    PERSON = 0,
    GROUP,
    ORG,
}

impl Layout {
    pub fn domain(&self) -> Result<Domain, &str> {
        match self.clock_seq_low {
            0 => Ok(Domain::PERSON),
            1 => Ok(Domain::GROUP),
            2 => Ok(Domain::ORG),
            _ => Err("Invalid Domain name"),
        }
    }

    pub fn id(&self) -> u32 {
        self.field_low
    }
}

impl UUID {
    pub fn v2(domain: Domain, id: u32) -> Layout {
        let id = {
            #[cfg(all(windows))]
            unsafe { libc::getpid() as u32 }.to_be_bytes();

            #[cfg(all(unix))]
            if domain == Domain::PERSON {
                unsafe { libc::getuid() }.to_be_bytes()
            } else if domain == Domain::GROUP {
                unsafe { libc::getgid() }.to_be_bytes()
            } else {
                id.to_be_bytes()
            }
        };

        let mut uuid = UUID::v1().generate().as_bytes();

        uuid[0..4].copy_from_slice(&id);
        uuid[6] = (Version::DCE as u8) << 4;
        uuid[9] = domain as u8;

        Layout::from_raw_bytes(uuid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Variant;

    #[test]
    fn uuid_dce_org() {
        let dce = UUID::v2(Domain::PERSON, 1234);
        assert_eq!(dce.version(), Ok(Version::DCE));
        assert_eq!(dce.variant(), Ok(Variant::RFC4122));
        assert_eq!(dce.domain(), Ok(Domain::PERSON));
    }
}
