mod data {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/structs.rs"));
    // include!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/datetime.rs"));
}

const TEST_DATA: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/segmenter.json"));

#[test]
fn test_data() {
    let tests: data::TestData = serde_json::from_str(TEST_DATA).expect("Failed to parse test JSON");

    #[cfg(feature = "icu4x-static")]
    {
        use icu_locid::LanguageIdentifier;
        use icu_perf_test_segmenter::icu4x;

        for case in tests.0.iter() {
            let seg = icu4x::LineSegmenter::new_static();
            let iterator = seg.segment(case.input);
            let result: Vec<_> = iterator.collect();
            assert_eq!(result, case.output.line.to_vec());

            let seg = icu4x::WordSegmenter::new_static();
            let iterator = seg.segment(case.input);
            let result: Vec<_> = iterator.collect();
            assert_eq!(result, case.output.word.to_vec());
        }
    }
}

// #[test]
// fn test_baked_data() {
//     let tests: data::TestData = serde_json::from_str(TEST_DATA).expect("Failed to parse test JSON");
//     let baked_tests = data::get_data();
//
//     assert_eq!(tests, baked_tests);
// }
