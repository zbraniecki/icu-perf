#[cfg(feature = "icu4c")]
use icu_perf_test_datetime::icu4c;

use std::time::Instant;

mod data {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/datetime.rs"));
}

const TEST_DATA: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/datetime.json"));

fn main() {
    let tests: data::TestData = serde_json::from_str(TEST_DATA).expect("Failed to parse test JSON");

    #[cfg(feature = "icu4c")]
    {
        let now = Instant::now();
        for test in &tests.0 {
            let dtf = icu4c::DateTimeFormatter::new(&test.langid);
            for case in &test.values {
                let result = dtf.format(case.input);
                // erintln!("{}", result);
            }
        }
        let measured_us = now.elapsed().as_micros();
        println!("Time: {} ns", measured_us);
    }
}
