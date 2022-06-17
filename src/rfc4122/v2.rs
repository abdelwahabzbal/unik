use crate::{Layout, Node, TimeStamp, Version, UUID};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Domain {
    PERSON = 0,
    GROUP,
    ORG,
}

impl std::string::ToString for Domain {
    fn to_string(&self) -> String {
        match self {
            Domain::PERSON => "PERSON".to_owned(),
            Domain::GROUP => "GROUP".to_owned(),
            Domain::ORG => "ORG".to_owned(),
        }
    }
}

impl Layout {
    pub fn get_domain(&self) -> Result<Domain, &str> {
        match self.field_low & 0xf {
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
    /// Creates `UUID` from a `Domain` and ID.
    pub fn v2(domain: Domain, id: u32) -> Layout {
        let sysid = {
            #[cfg(all(windows))]
            unsafe {
                libc::getpid() as u32
            }

            #[cfg(all(unix))]
            match domain {
                Domain::PERSON => unsafe { libc::getuid() },
                Domain::GROUP => unsafe { libc::getgid() },
                Domain::ORG => id,
            }
        }
        .to_be_bytes();

        let mut uuid = UUID::v1(TimeStamp(1234), Node::new([u8::MIN; 6]))
            .generate()
            .as_bytes();

        uuid[0..4].copy_from_slice(&sysid);
        uuid[6] = (Version::DCE as u8) << 4;
        uuid[9] = domain as u8;

        Layout::from_bytes(uuid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Variant;

    #[test]
    fn uuid_with_domain() {
        let layout = UUID::v2(Domain::ORG, 1234);
        assert_eq!(layout.get_version(), Ok(Version::DCE));
        assert_eq!(layout.get_variant(), Ok(Variant::RFC));
        assert_eq!(layout.get_domain(), Ok(Domain::ORG));
        assert_eq!(layout.get_id(), 1234);
    }
}
