use crate::{layout, Layout, MacAddress, TimeStamp, Variant, Version, UUID};

#[derive(Debug, Copy, Clone)]
pub enum Domain {
    PERSON = 0,
    GROUP,
}

impl UUID {
    /// Creates `UUID` from a domain and a name.
    pub fn v2(time: TimeStamp, node: MacAddress, domain: Domain) -> Layout {
        let id = {
            #[cfg(all(windows))]
            unsafe {
                libc::getpid() as u32
            }

            #[cfg(all(unix))]
            match domain {
                Domain::PERSON => unsafe { libc::getuid() },
                Domain::GROUP => unsafe { libc::getgid() },
            }
        };

        layout!(
            id.to_le_bytes()[0],
            id.to_le_bytes()[1],
            id.to_le_bytes()[2],
            id.to_le_bytes()[3],
            time.0.to_le_bytes()[0],
            time.0.to_le_bytes()[1],
            time.0.to_le_bytes()[2],
            (Version::DCE as u8) << 4,
            crate::clock_seq_high_and_reserved().to_le_bytes()[0],
            domain as u8,
            node.bytes()[0],
            node.bytes()[1],
            node.bytes()[2],
            node.bytes()[3],
            node.bytes()[4],
            node.bytes()[5]
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uuid_with_domain() {
        let layout = UUID::v2(
            TimeStamp::from_utc(),
            MacAddress::new([u8::MIN; 6]),
            Domain::PERSON,
        );

        assert_eq!(layout.get_version(), Ok(Version::DCE));
        assert_eq!(layout.get_variant(), Ok(Variant::RFC));
    }
}
