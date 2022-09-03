use crate::{Layout, Node, TimeStamp, Version, UUID};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Domain {
    PERSON = 0,
    GROUP,
    ORG,
}

impl Layout {
    pub fn get_domain(&self) -> Result<Domain, &str> {
        match self.clock_seq_low {
            0 => Ok(Domain::PERSON),
            1 => Ok(Domain::GROUP),
            2 => Ok(Domain::ORG),
            _ => Err("Invalid Domain name"),
        }
    }

    pub fn get_id(&self) -> u32 {
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

    fn from_domain(dom: Domain, i: u32) -> Layout {
        let id = if cfg!(unix) {
            if dom == Domain::PERSON {
                unsafe { libc::getpid() }.to_be_bytes()
            } else if dom == Domain::GROUP {
                unsafe { libc::getgid() }.to_be_bytes()
            } else {
                i.to_be_bytes()
            }
        } else if cfg!(windows) {
            if dom == Domain::PERSON || dom == Domain::GROUP {
                unsafe { libc::getpid() as i32 }.to_be_bytes()
            } else {
                i.to_be_bytes()
            }
        } else {
            panic!("Not supported")
        };

        let mut uuid = UUID::v1().generate().as_bytes();

        uuid[0..4].copy_from_slice(&id);
        uuid[6] = (Version::DCE as u8) << 4;
        uuid[9] = dom as u8;

        Layout::from_bytes(uuid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Variant;

    #[test]
    fn uuid_with_person() {
        let dce_person = UUID::from_dce_person();
        assert_eq!(dce_person.get_version(), Ok(Version::DCE));
        assert_eq!(dce_person.get_domain(), Ok(Domain::PERSON));
    }

    #[test]
    fn uuid_with_group() {
        let dce_group = UUID::from_dce_group();
        assert_eq!(dce_group.get_version(), Ok(Version::DCE));
        assert_eq!(dce_group.get_domain(), Ok(Domain::GROUP));
    }

    #[test]
    fn uuid_with_org() {
        let dce_org = UUID::from_dce_org(1234);
        assert_eq!(dce_org.get_version(), Ok(Version::DCE));
        assert_eq!(dce_org.get_variant(), Ok(Variant::RFC4122));
        assert_eq!(dce_org.get_domain(), Ok(Domain::ORG));
        assert_eq!(dce_org.get_id(), 1234);
    }
}
