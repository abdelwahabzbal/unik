#![no_main]
use libfuzzer_sys::fuzz_target;

use unik::*;

fuzz_target!(|data: [u8; 16]| {
    if let Ok(s) = std::str::from_utf8(&data) {
        let _ = UUID::from_str(s);
    }
});
