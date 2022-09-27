mod data {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/structs.rs"));
    // include!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/datetime.rs"));
}

const TEST_DATA: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/locid.json"));

#[cfg(feature = "icu4x")]
#[test]
fn test_data_icu4x() {
    let tests: data::TestData = serde_json::from_str(TEST_DATA).expect("Failed to parse test JSON");

    use icu_perf_test_locid::icu4x;

    for (input, expected) in tests.0.iter() {
        let result = icu4x::LanguageIdentifier::canonicalize(input);
        assert_eq!(result.as_str(), *expected);
    }
}

#[cfg(feature = "icu4c")]
#[test]
fn test_data_icu4c() {
    let tests: data::TestData = serde_json::from_str(TEST_DATA).expect("Failed to parse test JSON");

    use icu_perf_test_locid::icu4c;

    for (input, expected) in tests.0.iter() {
        let result = icu4c::LanguageIdentifier::canonicalize(input);
        assert_eq!(result.as_str(), *expected);
    }
}
