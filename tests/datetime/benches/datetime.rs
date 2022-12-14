use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;

mod data {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/structs.rs"));
}

#[cfg(feature = "icu4c")]
use icu_perf_test_datetime::icu4c;
#[cfg(feature = "icu4x")]
use icu_perf_test_datetime::icu4x;

const TEST_DATA: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/datetime.json"));

fn datetime(c: &mut Criterion) {
    let tests: data::TestData = serde_json::from_str(TEST_DATA).expect("Failed to parse test JSON");

    #[cfg(feature = "icu4x-static")]
    {
        use icu_locid::LanguageIdentifier;

        c.bench_function("icu4x/static/datetime/datetime/overview", |b| {
            b.iter(|| {
                for test in tests.0.iter() {
                    let provider = icu4x::DateTimeFormatter::get_static_provider();
                    for lid in test.langid.iter() {
                        let langid: LanguageIdentifier = lid.parse().unwrap();
                        for case in test.cases.iter() {
                            let dtf = icu4x::DateTimeFormatter::new_static(&provider, &langid, case.style.0, case.style.1);
                            for value in case.value.iter() {
                                let _ = dtf.format(black_box(value.epoch));
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

        c.bench_function("icu4x/baked/datetime/datetime/overview", |b| {
            b.iter(|| {
                for test in tests.0.iter() {
                    let provider = icu4x::DateTimeFormatter::get_baked_provider();
                    for lid in test.langid.iter() {
                        let langid: LanguageIdentifier = lid.parse().unwrap();
                        for case in test.cases.iter() {
                            let dtf = icu4x::DateTimeFormatter::new_baked(&provider, &langid, case.style.0, case.style.1);
                            for value in case.value.iter() {
                                let _ = dtf.format(black_box(value.epoch));
                            }
                        }
                    }
                }
            })
        });
    }

    #[cfg(feature = "icu4c")]
    {
        c.bench_function("icu4c/common/datetime/datetime/overview", |b| {
            b.iter(|| {
                for test in tests.0.iter() {
                    for lid in test.langid.iter() {
                        for case in test.cases.iter() {
                            let dtf = icu4c::DateTimeFormatter::new(lid, case.style.0, case.style.1);
                            for value in case.value.iter() {
                                let _ = dtf.format(black_box(value.epoch));
                            }
                        }
                    }
                }
            })
        });
    }
}

criterion_group!(benches, datetime);
criterion_main!(benches);
