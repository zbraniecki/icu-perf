use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;

use icu_locid::LanguageIdentifier;
use icu_perf_test_locid::icu4c;
use intl_harness::data::locale::get_test_set;

fn language_identifier(c: &mut Criterion) {
    let data_set = get_test_set("../../../harness/data");
    let strings: Vec<_> = data_set.0.iter().map(|test| &test.input).collect();

    // c.bench_function("language_identifier_parsing", |b| {
    //     b.iter(|| {
    //         for s in &strings {
    //             let _: Result<LanguageIdentifier, _> = black_box(s).parse();
    //         }
    //     })
    // });

    // c.bench_function("language_identifier_matches", |b| {
    //     let locales: Vec<LanguageIdentifier> = strings.iter().map(|s| s.parse().unwrap()).collect();
    //     let reference_locale: LanguageIdentifier = "en-US".parse().unwrap();
    //     let mut matches = 0;
    //
    //     b.iter(|| {
    //         for loc in &locales {
    //             if loc == &reference_locale {
    //               matches += 1;
    //             }
    //         }
    //     })
    // });
    //
    // c.bench_function("language_identifier_serialize", |b| {
    //     let locales: Vec<LanguageIdentifier> = strings.iter().map(|s| s.parse().unwrap()).collect();
    //
    //     b.iter(|| {
    //         for loc in &locales {
    //             let _ = black_box(loc).to_string();
    //         }
    //     })
    // });

    c.bench_function("icu4x/components/locid/langid/canonicalize", |b| {
        b.iter(|| {
            for s in &strings {
                let _ = LanguageIdentifier::canonicalize(black_box(s));
            }
        })
    });

    c.bench_function("icu4c/components/locid/langid/canonicalize", |b| {
        b.iter(|| {
            for s in &strings {
                let _ = icu4c::LanguageIdentifier::canonicalize(black_box(s));
            }
        })
    });
}

criterion_group!(benches, language_identifier);
criterion_main!(benches);
