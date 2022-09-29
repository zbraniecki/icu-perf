use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;

mod data {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/structs.rs"));
}

#[cfg(feature = "icu4c")]
use icu_perf_test_collator::icu4c;
#[cfg(feature = "icu4x")]
use icu_perf_test_collator::icu4x;

const TEST_DATA: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/collator.json"));

fn number(c: &mut Criterion) {
    let tests: data::TestData = serde_json::from_str(TEST_DATA).expect("Failed to parse test JSON");

    #[cfg(feature = "icu4x-static")]
    {
        c.bench_function("icu4x/static/collator/compare", |b| {
            b.iter(|| {
                for test in &tests.0 {
                    let provider = icu4x::Collator::get_static_provider();
                    for lid in test.langids.iter() {
                        let langid: icu_locid::LanguageIdentifier = lid.parse().unwrap();
                        for case in test.cases.iter() {
                            let col = icu4x::Collator::new_static(&provider, &langid);
                            for value in case.values.iter() {
                                let _ = col.compare(black_box(value.left), black_box(value.right));
                            }
                        }
                    }
                }
            })
        });
    }

    #[cfg(feature = "icu4x-baked")]
    {
        c.bench_function("icu4x/baked/collator/compare", |b| {
            b.iter(|| {
                for test in &tests.0 {
                    let provider = icu4x::Collator::get_baked_provider();
                    for lid in test.langids.iter() {
                        let langid: icu_locid::LanguageIdentifier = lid.parse().unwrap();
                        for case in test.cases.iter() {
                            let col = icu4x::Collator::new_baked(&provider, &langid);
                            for value in case.values.iter() {
                                let _ = col.compare(black_box(value.left), black_box(value.right));
                            }
                        }
                    }
                }
            })
        });
    }

    #[cfg(feature = "icu4c")]
    {
        c.bench_function("icu4c/common/collator/compare", |b| {
            b.iter(|| {
                for test in &tests.0 {
                    for lid in test.langids.iter() {
                        for case in test.cases.iter() {
                            let col = icu4c::Collator::new(&lid);
                            for value in case.values.iter() {
                                let _ = col.compare(black_box(value.left), black_box(value.right));
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
