use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;

mod data {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/structs.rs"));
}

#[cfg(feature = "icu4c")]
use icu_perf_test_locid::icu4c;
#[cfg(feature = "icu4x")]
use icu_perf_test_locid::icu4x;

const TEST_DATA: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/locid.json"));

fn language_identifier(c: &mut Criterion) {
    let tests: data::TestData = serde_json::from_str(TEST_DATA).expect("Failed to parse test JSON");

    c.bench_function("icu4x/components/locid/langid/canonicalize", |b| {
        b.iter(|| {
            for (input, _output) in tests.0.iter() {
                let _ = icu4x::LanguageIdentifier::canonicalize(black_box(input));
            }
        })
    });

    c.bench_function("icu4c/components/locid/langid/canonicalize", |b| {
        b.iter(|| {
            for (input, _output) in tests.0.iter() {
                let _ = icu4c::LanguageIdentifier::canonicalize(black_box(input));
            }
        })
    });
}

criterion_group!(benches, language_identifier);
criterion_main!(benches);
