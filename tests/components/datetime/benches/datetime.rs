use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;

#[cfg(feature = "icu4c")]
use icu_perf_test_datetime::icu4c;
#[cfg(feature = "icu4x")]
use icu_perf_test_datetime::icu4x;

fn datetime(c: &mut Criterion) {
    // let data_set = get_test_set("../../../harness/data");
    // let strings: Vec<_> = data_set.0.iter().map(|test| &test.input).collect();

    #[cfg(feature = "icu4x")]
    {
        c.bench_function("icu4x/components/datetime/datetime/format", |b| {
            b.iter(|| {
                // let provider =
                //     icu4x::DateTimeFormatter::get_static_provider();
                // let dtf = icu4x::DateTimeFormatter::new_static(&provider);
                let provider = icu4x::DateTimeFormatter::get_baked_provider();
                let dtf = icu4x::DateTimeFormatter::new_baked(&provider);
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
