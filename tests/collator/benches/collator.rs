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
        use icu_locid::LanguageIdentifier;

        c.bench_function("icu4x/static/collator/compare", |b| {
            b.iter(|| {
                let provider = icu4x::Collator::get_static_provider();
                for test in &tests.0 {
                    let langid: LanguageIdentifier = test.langid.parse().unwrap();
                    let col = icu4x::Collator::new_static(&provider, &langid);
                    for case in &test.values {
                        let _ = col.compare(black_box(case.left), black_box(case.right));
                    }
                }
            })
        });
    }

    #[cfg(feature = "icu4x-baked")]
    {
        use icu_locid::LanguageIdentifier;

        c.bench_function("icu4x/baked/collator/compare", |b| {
            b.iter(|| {
                let provider = icu4x::Collator::get_baked_provider();
                for test in &tests.0 {
                    let langid: LanguageIdentifier = test.langid.parse().unwrap();
                    let col = icu4x::Collator::new_baked(&provider, &langid);
                    for case in &test.values {
                        let _ = col.compare(black_box(case.left), black_box(case.right));
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
                    let col = icu4c::Collator::new(&test.langid);
                    for case in &test.values {
                        let _ = col.compare(black_box(case.left), black_box(case.right));
                    }
                }
            })
        });
    }
}

criterion_group!(benches, number);
criterion_main!(benches);
