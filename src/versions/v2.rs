#![cfg(feature = "v2")]
#![doc(cfg(feature = "v2"))]

use crate::{Layout, Node, Timestamp, Variant, Version, UUID};

#[derive(Debug, Copy, Clone)]
pub enum Domain {
    PERSON = 0,
    GROUP,
    ORG,
}

impl Layout {
    fn from_dce(ts: Timestamp, clock_seq: u16, node: Node, domain: Domain) -> Self {
        let id = if cfg!(windows) {
            unsafe { libc::getpid() as u32 }
        } else {
            match domain {
                Domain::PERSON => unsafe { libc::getuid() },
                Domain::GROUP => unsafe { libc::getgid() },
                Domain::ORG => todo!(),
            }
        };

        Self {
            timestamp: None,
            field_low: id,
            field_mid: ((ts.0 >> 32 & 0xffff) as u16),
            field_high_and_version: (ts.0 >> 48 & 0xfff) as u16 | (Version::DCE as u16) << 12,
            clock_seq_high_and_reserved: (clock_seq >> 8 & 0xf) as u8 | (Variant::RFC as u8) << 4,
            clock_seq_low: domain as u8,
            node: node,
        }
    }
}

impl UUID {
    pub fn v2(ts: Timestamp, node: Node, domain: Domain) -> Layout {
        Layout::from_dce(ts, crate::clock_seq_high_and_reserved(), node, domain)
    }
}

#[cfg(test)]
mod tests {
    use crate::Variant;

    use super::*;

    #[test]
    fn test_name() {
        let layout = UUID::v2(Timestamp::from_utc(), Node([u8::MAX; 6]), Domain::PERSON);

        assert_eq!(layout.version(), Ok(Version::DCE));
        assert_eq!(layout.variant(), Ok(Variant::RFC));

        assert_eq!(layout.field_low, unsafe { libc::getuid() });
    }
}
