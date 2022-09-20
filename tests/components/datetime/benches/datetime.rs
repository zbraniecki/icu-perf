use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;

#[cfg(feature = "icu4x")]
use icu_perf_test_datetime::icu4x;
#[cfg(feature = "icu4c")]
use icu_perf_test_datetime::icu4c;

fn datetime(c: &mut Criterion) {
    // let data_set = get_test_set("../../../harness/data");
    // let strings: Vec<_> = data_set.0.iter().map(|test| &test.input).collect();

    #[cfg(feature = "icu4x")]
    {
        const ICU4X_DATA: &[u8] = include_bytes!(concat!("../../../../data/icu4x-1.0.postcard"));
        use icu_provider_blob::StaticDataProvider;
        let provider =
            StaticDataProvider::try_new_from_static_blob(&ICU4X_DATA).expect("Failed to load data");
        c.bench_function("icu4x/components/datetime/datetime/format", |b| {
            b.iter(|| {
                let dtf = icu4x::DateTimeFormatter::new(&provider);
                dtf.format();
            })
        });
    }

    #[cfg(feature = "icu4c")]
    {
        c.bench_function("icu4c/components/datetime/datetime/format", |b| {
            b.iter(|| {
                let dtf = icu4c::DateTimeFormatter::new();
                dtf.format();
            })
        });
    }

    // c.bench_function("icu4c/components/datetime/datetime/format", |b| {
    //     b.iter(|| {
    //         for s in &strings {
    //             let _ = icu4c::LanguageIdentifier::canonicalize(black_box(s));
    //         }
    //     })
    // });
}

criterion_group!(benches, datetime);
criterion_main!(benches);
