use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;

mod data {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/structs.rs"));
}

#[cfg(feature = "icu4c")]
use icu_perf_test_number::icu4c;
#[cfg(feature = "icu4x")]
use icu_perf_test_number::icu4x;

const TEST_DATA: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/number.json"));

fn number(c: &mut Criterion) {
    let tests: data::TestData = serde_json::from_str(TEST_DATA).expect("Failed to parse test JSON");

    #[cfg(feature = "icu4x-static")]
    {
        use icu_locid::LanguageIdentifier;
        use icu_decimal::options::{FixedDecimalFormatterOptions, GroupingStrategy};

        c.bench_function("icu4x/static/number/number/format", |b| {
            b.iter(|| {
                for test in &tests.0 {
                    let provider = icu4x::NumberFormatter::get_static_provider();
                    for lid in test.langid.iter() {
                        let langid: LanguageIdentifier = lid.parse().unwrap();
                        for case in test.cases.iter() {
                            let mut options: FixedDecimalFormatterOptions = Default::default();
                            options.grouping_strategy = GroupingStrategy::Auto;
                            let nf = icu4x::NumberFormatter::new_static(&provider, &langid, options);
                            for value in case.values.iter() {
                                let _ = nf.format(black_box(value.input));
                            }
                        }
                    }
                }
            })
        });
    }

    #[cfg(feature = "icu4x-baked")]
    {
        use icu_locid::LanguageIdentifier;
        use icu_decimal::options::{FixedDecimalFormatterOptions, GroupingStrategy};

        c.bench_function("icu4x/baked/number/number/format", |b| {
            b.iter(|| {
                for test in &tests.0 {
                    let provider = icu4x::NumberFormatter::get_baked_provider();
                    for lid in test.langid.iter() {
                        let langid: LanguageIdentifier = lid.parse().unwrap();
                        for case in test.cases.iter() {
                            let mut options: FixedDecimalFormatterOptions = Default::default();
                            options.grouping_strategy = GroupingStrategy::Auto;
                            let nf = icu4x::NumberFormatter::new_baked(&provider, &langid, options);
                            for value in case.values.iter() {
                                let _ = nf.format(black_box(value.input));
                            }
                        }
                    }
                }
            })
        });
    }

    #[cfg(feature = "icu4c")]
    {
        c.bench_function("icu4c/common/number/number/format", |b| {
            b.iter(|| {
                for test in &tests.0 {
                    for lid in test.langid.iter() {
                        for case in test.cases.iter() {
                            let nf = icu4c::NumberFormatter::new(lid);
                            for value in case.values.iter() {
                                let _ = nf.format(black_box(value.input));
                            }
                        }
                    }
                }
            })
        });
    }
}

criterion_group!(benches, number);
criterion_main!(benches);
