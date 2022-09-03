use std::{
    cell::Cell,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use mac_address::{get_mac_address, MacAddress};
use nanorand::{Rng, WyRand};

use crate::{layout, ClockSeq, Layout, TimeStamp, Variant, Version, UUID};

impl Layout {
    pub fn timestamp(&self) -> TimeStamp {
        TimeStamp {
            dur: self.field_low as u64
                | (self.field_mid as u64) << 32
                | ((self.field_high_and_version as u64 >> 4) & 0xff) << 48,
        }
    }
}

impl UUID {
    /// Returns the `Layout` for `UUID` generated from `Node` and `TimeStamp`.
    pub fn v1() -> Layout {
        let time = TimeStamp::from_utc();
        let node = get_mac_address().unwrap().unwrap().bytes();
        let clock_seq_high_and_reserved = clock_seq_high_and_reserved().to_le_bytes();
        layout!(
            time.to_le_bytes()[0],
            time.to_le_bytes()[1],
            time.to_le_bytes()[2],
            time.to_le_bytes()[3],
            time.to_le_bytes()[4],
            time.to_le_bytes()[5],
            time.to_le_bytes()[6],
            (Version::TIME as u8) << 4,
            clock_seq_high_and_reserved[0],
            clock_seq_high_and_reserved[1],
            node[0],
            node[1],
            node[2],
            node[3],
            node[4],
            node[5]
        )
    }
}

impl TimeStamp {
    /// Returns the elapsed time since 01/01/1970.
    pub fn from_unix() -> u64 {
        (SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
            & 0xffff_ffff) as u64
    }

    /// Returns the elapsed time since 15/10/1582.
    pub fn from_utc() -> u64 {
        (SystemTime::now()
            .duration_since(UNIX_EPOCH + Duration::from_nanos(0x01B2_1DD2_1381_4000))
            .unwrap()
            .as_nanos()
            & 0xffff_ffff) as u64
    }

    pub fn get(&self) -> u64 {
        self.0
    }
}

pub(crate) fn clock_seq_high_and_reserved() -> u16 {
    ClockSeq::new(WyRand::new().generate::<u16>()).to_le()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uuid_builder_patt() {
        let layout = UUID::v1();
        assert_eq!(layout.version(), Ok(Version::TIME));
        assert_eq!(layout.variant(), Ok(Variant::RFC4122));

        layout.timestamp().dur.set(1234_5678u64);
        assert_eq!(layout.timestamp().dur.get(), 1234_5678u64);
    }

    // #[test]
    // fn get_timestamp_from_uuid_string() {
    //     let layout = UUID::v1();
    //     layout.timestamp.set(1234_5678u64);
    //     let timestamp = UUID::from_str(&layout.generate().to_string()).unwrap();

    //     assert_eq!(timestamp, 1234_5678u64);
    // }

    // #[test]
    // fn layout_from_seq_bytes() {
    //     let uuid = UUID::v1().generate();
    //     let layout = Layout::from_bytes(uuid.as_bytes());

    //     assert_eq!(layout.version(), Ok(Version::TIME));
    //     assert_eq!(layout.variant(), Ok(Variant::RFC4122));

    //     layout.timestamp.set(1234_5678u64);
    //     assert_eq!(layout.timestamp.get(), 1234_5678u64);
    // }
}
