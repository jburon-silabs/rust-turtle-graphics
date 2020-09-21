#![no_main]
use libfuzzer_sys::fuzz_target;
extern crate turtle;
use turtle::parse_turtle_commands;


fuzz_target!(|data: &[u8]| {
    // convert data to String
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = parse_turtle_commands(format!("color({}, {}, {})", s.to_string(), s.to_string(), s.to_string()));
    }
});
