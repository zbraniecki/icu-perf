mod data {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/structs.rs"));
}

const TEST_DATA: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/number.json"));

fn get_expected<'s>(langid: &str, output: &Box<[(&str, &'s str)]>) -> Option<&'s str> {
    for case in output.iter() {
        if case.0 == langid {
            return Some(case.1);
        }
    }
    None
}

#[test]
fn test_data() {
    let tests: data::TestData = serde_json::from_str(TEST_DATA).expect("Failed to parse test JSON");

    #[cfg(feature = "icu4x-baked")]
    {
        use icu_decimal::options::{FixedDecimalFormatterOptions, GroupingStrategy};
        use icu_locid::LanguageIdentifier;
        use icu_perf_test_number::icu4x;

        let provider = icu4x::NumberFormatter::get_baked_provider();
        for test in tests.0.iter() {
            for lid in test.langid.iter() {
                let langid: LanguageIdentifier = lid.parse().unwrap();
                for case in test.cases.iter() {
                    let mut options: FixedDecimalFormatterOptions = Default::default();
                    options.grouping_strategy = GroupingStrategy::Auto;
                    let nf = icu4x::NumberFormatter::new_baked(&provider, &langid, options);
                    for value in case.values.iter() {
                        let result = nf.format(value.input);
                        if let Some(expected) = get_expected(lid, &value.output) {
                            assert_eq!(result, expected);
                        }
                    }
                }
            }
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
