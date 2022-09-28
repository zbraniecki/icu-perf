mod data {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/structs.rs"));
    // include!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/datetime.rs"));
}

const TEST_DATA: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/datetime.json"));

fn get_expected<'s>(langid: &str, output: &Box<[(&str, &'s str)]>) -> Option<&'s str> {
    for case in output.iter() {
        if case.0 == langid {
            return Some(case.1);
        }
    }
    None
}

#[cfg(feature = "icu4x-baked")]
#[test]
fn test_data_icu4x() {
    let tests: data::TestData = serde_json::from_str(TEST_DATA).expect("Failed to parse test JSON");

    use icu_locid::LanguageIdentifier;
    use icu_perf_test_datetime::icu4x;

    let provider = icu4x::DateTimeFormatter::get_baked_provider();
    for test in tests.0.iter() {
        for lid in test.langid.iter() {
            let langid: LanguageIdentifier = lid.parse().unwrap();
            for case in test.cases.iter() {
                let dtf = icu4x::DateTimeFormatter::new_baked(
                    &provider,
                    &langid,
                    case.style.0,
                    case.style.1,
                );
                for value in case.value.iter() {
                    let result = dtf.format(value.epoch);
                    if let Some(expected) = get_expected(lid, &value.output) {
                        assert_eq!(result, expected);
                    }
                }
            }
        }
    }
}

#[cfg(feature = "icu4c")]
#[test]
fn test_data_icu4c() {
    let tests: data::TestData = serde_json::from_str(TEST_DATA).expect("Failed to parse test JSON");

    use icu_perf_test_datetime::icu4c;

    for test in tests.0.iter() {
        for lid in test.langid.iter() {
            for case in test.cases.iter() {
                let dtf = icu4c::DateTimeFormatter::new(
                    lid,
                    case.style.0,
                    case.style.1,
                );
                for value in case.value.iter() {
                    let result = dtf.format(value.epoch);
                    if let Some(expected) = get_expected(lid, &value.output) {
                        assert_eq!(result, expected);
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
