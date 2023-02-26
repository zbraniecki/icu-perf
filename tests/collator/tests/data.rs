use std::cmp::Ordering;

mod data {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/structs.rs"));
}

const TEST_DATA: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/collator.json"));

fn get_expected<'s>(langid: &str, output: &Box<[(&str, &'s str)]>) -> Option<&'s str> {
    for case in output.iter() {
        if case.0 == langid {
            return Some(case.1);
        }
    }
    None
}

fn get_output_value(input: &str) -> Ordering {
    match input {
        "Less" => Ordering::Less,
        "Greater" => Ordering::Greater,
        "Equal" => Ordering::Equal,
        _ => unreachable!(),
    }
}

#[cfg(feature = "icu4x-static")]
#[test]
fn test_data_icu4x() {
    let tests: data::TestData = serde_json::from_str(TEST_DATA).expect("Failed to parse test JSON");
    use icu_perf_test_collator::icu4x;

    let provider = icu4x::Collator::get_static_provider();

    for test in tests.0.iter() {
        for lid in test.langids.iter() {
            let langid: icu_locid::LanguageIdentifier = lid.parse().unwrap();
            for case in test.cases.iter() {
                let col = icu4x::Collator::new_static(&provider, &langid);
                for value in case.values.iter() {
                    let result = col.compare(value.left, value.right);
                    if let Some(expected) = get_expected(lid, &value.output) {
                        assert_eq!(result, get_output_value(expected), "{}-{}", value.left, value.right);
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
    use icu_perf_test_collator::icu4c;

    for test in tests.0.iter() {
        for lid in test.langids.iter() {
            for case in test.cases.iter() {
                let col = icu4c::Collator::new(lid);
                for value in case.values.iter() {
                    let result = col.compare(value.left, value.right);
                    if let Some(expected) = get_expected(lid, &value.output) {
                        assert_eq!(result, get_output_value(expected), "{}-{}", value.left, value.right);
                    }
                }
            }
        }
    }
}
