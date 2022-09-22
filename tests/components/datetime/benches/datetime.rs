use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;

mod data {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/datetime.rs"));
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

        c.bench_function("icu4x/static/components/datetime/datetime/format", |b| {
            b.iter(|| {
                let provider = icu4x::DateTimeFormatter::get_static_provider();
                for test in &tests.0 {
                    let langid: LanguageIdentifier = test.langid.parse().unwrap();
                    let dtf = icu4x::DateTimeFormatter::new_static(&provider, &langid);
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

        c.bench_function("icu4x/baked/components/datetime/datetime/format", |b| {
            b.iter(|| {
                let provider = icu4x::DateTimeFormatter::get_baked_provider();
                for test in &tests.0 {
                    let langid: LanguageIdentifier = test.langid.parse().unwrap();
                    let dtf = icu4x::DateTimeFormatter::new_baked(&provider, &langid);
                    for case in &test.values {
                        let _ = dtf.format(case.input);
                    }
                }
            })
        });
    }

    #[cfg(feature = "icu4c")]
    {
        c.bench_function("icu4c/components/datetime/datetime/format", |b| {
            b.iter(|| {
                for test in &tests.0 {
                    let dtf = icu4c::DateTimeFormatter::new(&test.langid);
                    for case in &test.values {
                        let _ = dtf.format(case.input);
                    }
                }
            })
        });
    }
}

criterion_group!(benches, datetime);
criterion_main!(benches);
