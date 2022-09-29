use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;

mod data {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/structs.rs"));
}

#[cfg(feature = "icu4c")]
use icu_perf_test_plurals::icu4c;
#[cfg(feature = "icu4x")]
use icu_perf_test_plurals::icu4x;

const TEST_DATA: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/plurals.json"));

fn plurals(c: &mut Criterion) {
    let tests: data::TestData = serde_json::from_str(TEST_DATA).expect("Failed to parse test JSON");

    #[cfg(feature = "icu4x-static")]
    {
        use icu_locid::LanguageIdentifier;

        c.bench_function("icu4x/static/plurals/category_for", |b| {
            b.iter(|| {
                for test in &tests.0 {
                    let provider = icu4x::PluralRules::get_static_provider();
                    for lid in test.langid.iter() {
                        let langid: LanguageIdentifier = lid.parse().unwrap();
                        for case in test.cases.iter() {
                            let cardinal = case.style == "cardinal";
                            let pr = icu4x::PluralRules::new_static(&provider, &langid, cardinal);
                            for value in case.values.iter() {
                                let _ = pr.select(black_box(value.input));
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

        c.bench_function("icu4x/baked/plurals/category_for", |b| {
            b.iter(|| {
                for test in &tests.0 {
                    let provider = icu4x::PluralRules::get_baked_provider();
                    for lid in test.langid.iter() {
                        let langid: LanguageIdentifier = lid.parse().unwrap();
                        for case in test.cases.iter() {
                            let cardinal = case.style == "cardinal";
                            let pr = icu4x::PluralRules::new_baked(&provider, &langid, cardinal);
                            for value in case.values.iter() {
                                let _ = pr.select(black_box(value.input));
                            }
                        }
                    }
                }
            })
        });
    }

    #[cfg(feature = "icu4c")]
    {
        c.bench_function("icu4c/common/plurals/category_for", |b| {
            b.iter(|| {
                for test in &tests.0 {
                    for lid in test.langid.iter() {
                        for case in test.cases.iter() {
                            let cardinal = case.style == "cardinal";
                            let pr = icu4c::PluralRules::new(lid, cardinal);
                            for value in case.values.iter() {
                                let _ = pr.select(black_box(value.input));
                            }
                        }
                    }
                }
            })
        });
    }
}

criterion_group!(benches, plurals);
criterion_main!(benches);
