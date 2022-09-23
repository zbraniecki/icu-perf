mod data {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/structs.rs"));
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/datetime.rs"));
}

const TEST_DATA: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/datetime.json"));

#[test]
fn test_data() {
    let tests: data::TestData = serde_json::from_str(TEST_DATA).expect("Failed to parse test JSON");

    #[cfg(feature = "icu4x-baked")]
    {
        use icu_locid::LanguageIdentifier;
        use icu_perf_test_datetime::icu4x;

        let provider = icu4x::DateTimeFormatter::get_baked_provider();
        for test in tests.0.iter() {
            let langid: LanguageIdentifier = test.langid.parse().unwrap();
            let dtf = icu4x::DateTimeFormatter::new_baked(&provider, &langid);
            for case in test.values.iter() {
                let result = dtf.format(case.input);
                assert_eq!(result, case.output);
            }
        }
    }
    #[cfg(not(feature = "icu4x-baked"))]
    {
        panic!("Enable icu4x-baked feature to run tests");
    }
}

#[test]
fn test_baked_data() {
    let tests: data::TestData = serde_json::from_str(TEST_DATA).expect("Failed to parse test JSON");
    let baked_tests = data::get_data();

    assert_eq!(tests, baked_tests);
}
