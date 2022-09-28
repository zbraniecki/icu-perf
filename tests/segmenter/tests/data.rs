mod data {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/structs.rs"));
}

const TEST_DATA: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/segmenter.json"));

#[cfg(feature = "icu4x-static")]
#[test]
fn test_data_icu4x() {
    let tests: data::TestData = serde_json::from_str(TEST_DATA).expect("Failed to parse test JSON");
    use icu_perf_test_segmenter::icu4x;

    let provider = icu4x::WordSegmenter::get_static_provider();

    for case in tests.0.iter() {
        let seg = icu4x::LineSegmenter::new_static(&provider);
        let iterator = seg.segment(case.input);
        let result: Vec<_> = iterator.collect();
        assert_eq!(result, case.output.line.to_vec());

        let seg = icu4x::WordSegmenter::new_static(&provider);
        let iterator = seg.segment(case.input);
        let result: Vec<_> = iterator.collect();
        assert_eq!(result, case.output.word.to_vec());
    }
}

#[cfg(feature = "icu4c")]
#[test]
fn test_data_icu4c() {
    let tests: data::TestData = serde_json::from_str(TEST_DATA).expect("Failed to parse test JSON");
    use icu_perf_test_segmenter::icu4c;

    for case in tests.0.iter() {
        let seg = icu4c::Segmenter::new(case.langid, case.input, false);
        let result: Vec<_> = seg.map(|i| i as usize).collect();
        if let Some(o) = &case.output.icu4c {
            assert_eq!(result, o.line.to_vec());
        } else {
            assert_eq!(result, case.output.line.to_vec());
        }

        let seg = icu4c::Segmenter::new(case.langid, case.input, true);
        let result: Vec<_> = seg.map(|i| i as usize).collect();
        if let Some(o) = &case.output.icu4c {
            assert_eq!(result, o.word.to_vec());
        } else {
            assert_eq!(result, case.output.word.to_vec());
        }
    }
}
