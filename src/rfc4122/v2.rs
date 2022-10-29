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
    pub fn from_dce_person() -> Layout {
        Self::from_domain(Domain::PERSON, 0)
    }

    pub fn from_dce_group() -> Layout {
        Self::from_domain(Domain::GROUP, 0)
    }

    pub fn from_dce_org(id: u32) -> Layout {
        Self::from_domain(Domain::ORG, id)
    }

    fn from_domain(domain: Domain, id: u32) -> Layout {
        let id = if cfg!(unix) {
            if domain == Domain::PERSON {
                unsafe { libc::getuid() }.to_be_bytes()
            } else if domain == Domain::GROUP {
                unsafe { libc::getgid() }.to_be_bytes()
            } else {
                id.to_be_bytes()
            }
        } else if cfg!(windows) {
            if domain == Domain::PERSON || domain == Domain::GROUP {
                unsafe { libc::getpid() as i32 }.to_be_bytes()
            } else {
                id.to_be_bytes()
            }
        } else {
            panic!("Not supported yet!")
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
    fn uuid_dce_person() {
        let dce_person = UUID::from_dce_person();
        assert_eq!(dce_person.version(), Ok(Version::DCE));
        assert_eq!(dce_person.domain(), Ok(Domain::PERSON));
    }

    #[test]
    fn uuid_dce_group() {
        let dce_group = UUID::from_dce_group();
        assert_eq!(dce_group.version(), Ok(Version::DCE));
        assert_eq!(dce_group.domain(), Ok(Domain::GROUP));
    }

    #[test]
    fn uuid_dce_org() {
        let dce_org = UUID::from_dce_org(1234);
        assert_eq!(dce_org.version(), Ok(Version::DCE));
        assert_eq!(dce_org.variant(), Ok(Variant::RFC4122));
        assert_eq!(dce_org.domain(), Ok(Domain::ORG));
        assert_eq!(dce_org.id(), 1234);
    }
}
