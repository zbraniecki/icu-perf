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

    #[cfg(feature = "icu4x-static")]
    {
        c.bench_function("icu4x/static/components/datetime/datetime/format", |b| {
            b.iter(|| {
                let provider = icu4x::DateTimeFormatter::get_static_provider();
                let dtf = icu4x::DateTimeFormatter::new_static(&provider);
                dtf.format();
            })
        });
    }

    #[cfg(feature = "icu4x-baked")]
    {
        c.bench_function("icu4x/baked/components/datetime/datetime/format", |b| {
            b.iter(|| {
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
}

criterion_group!(benches, datetime);
criterion_main!(benches);
