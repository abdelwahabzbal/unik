use std::time::{Duration, SystemTime, UNIX_EPOCH};

use mac_address::get_mac_address;
use nanorand::{Rng, WyRand};

use crate::{layout, ClockSeq, Layout, Timestamp, Version, UUID};

impl UUID {
    /// Returns the `Layout` for `UUID` generated from `Node` and `Timestamp`.
    pub fn v1() -> Layout {
        let ts = Timestamp::utc_epoch().to_be_bytes();
        let cshr = clock_seq_high_and_reserved().to_be_bytes();
        let n = get_mac_address().unwrap().unwrap().bytes();
        layout!(
            ts[0],
            ts[1],
            ts[2],
            ts[3],
            ts[4],
            ts[5],
            ts[6],
            ((Version::TIME as u8) << 0x4) | (ts[7] & 0xf),
            cshr[0],
            cshr[1],
            n[0],
            n[1],
            n[2],
            n[3],
            n[4],
            n[5]
        )
    }
}

impl Timestamp {
    /// Returns the elapsed time since 01/01/1970.
    pub fn unix_epoch() -> u64 {
        (SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
            & 0xffff_ffff) as u64
    }

    /// Returns the elapsed time since 15/10/1582.
    pub fn utc_epoch() -> u64 {
        (SystemTime::now()
            .duration_since(UNIX_EPOCH + Duration::from_nanos(0x01B2_1DD2_1381_4000))
            .unwrap()
            .as_nanos()
            & 0xffff_ffff) as u64
    }
}

pub(crate) fn clock_seq_high_and_reserved() -> u16 {
    ClockSeq::new(WyRand::new().generate::<u16>())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Variant;

    // static TIMESTAMP_TEST: u64 = 0;

    #[test]
    fn uuid_from_builder() {
        let layout = UUID::v1();
        assert_eq!(layout.version(), Ok(Version::TIME));
        assert_eq!(layout.variant(), Ok(Variant::RFC4122));
    }

    #[test]
    fn layout_from_raw_bytes() {
        let uuid = UUID::v1().generate();
        let layout = Layout::from_raw_bytes(uuid.as_bytes());

        assert_eq!(layout.version(), Ok(Version::TIME));
        assert_eq!(layout.variant(), Ok(Variant::RFC4122));
    }
}
