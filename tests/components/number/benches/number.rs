use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;

mod data {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/number.rs"));
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

        c.bench_function("icu4x/static/number/number/format", |b| {
            b.iter(|| {
                let provider = icu4x::NumberFormatter::get_static_provider();
                for test in &tests.0 {
                    let langid: LanguageIdentifier = test.langid.parse().unwrap();
                    let dtf = icu4x::NumberFormatter::new_static(&provider, &langid);
                    for case in &test.values {
                        let _ = dtf.format(case.input);
                    }
                }
            })
        });
    }

    #[cfg(feature = "icu4x-baked")]
    {
        use icu_locid::LanguageIdentifier;

        c.bench_function("icu4x/baked/number/number/format", |b| {
            b.iter(|| {
                let provider = icu4x::NumberFormatter::get_baked_provider();
                for test in &tests.0 {
                    let langid: LanguageIdentifier = test.langid.parse().unwrap();
                    let nf = icu4x::NumberFormatter::new_baked(&provider, &langid);
                    for case in &test.values {
                        let _ = nf.format(case.input);
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
                    let nf = icu4c::NumberFormatter::new(&test.langid);
                    for case in &test.values {
                        let _ = nf.format(case.input);
                    }
                }
            })
        });
    }
}

criterion_group!(benches, number);
criterion_main!(benches);
