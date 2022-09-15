use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;

use icu_datetime::{options::length, DateTimeFormatter};
use icu_locid::langid;
use icu_provider_blob::StaticDataProvider;
use intl_harness::data::datetime::get_test_set;

const ICU4X_DATA: &[u8] = include_bytes!(concat!("../../../../data/icu4x-1.0.postcard"));

fn datetime(c: &mut Criterion) {
    // let data_set = get_test_set("../../../harness/data");
    // let strings: Vec<_> = data_set.0.iter().map(|test| &test.input).collect();

    let provider =
        StaticDataProvider::try_new_from_static_blob(&ICU4X_DATA).expect("Failed to load data");

    c.bench_function("icu4x/components/datetime/datetime/format", |b| {
        b.iter(|| {
            let langid = langid!("en");
            let options =
                length::Bag::from_date_time_style(length::Date::Long, length::Time::Medium).into();
            let dtf =
                DateTimeFormatter::try_new_with_buffer_provider(&provider, &langid.into(), options)
                    .unwrap();
        })
    });

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
