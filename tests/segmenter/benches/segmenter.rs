use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;

mod data {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/structs.rs"));
}

#[cfg(feature = "icu4c")]
use icu_perf_test_segmenter::icu4c;
#[cfg(feature = "icu4x")]
use icu_perf_test_segmenter::icu4x;

const TEST_DATA: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/segmenter.json"));

fn number(c: &mut Criterion) {
    let tests: data::TestData = serde_json::from_str(TEST_DATA).expect("Failed to parse test JSON");

    #[cfg(feature = "icu4x-static")]
    {
        c.bench_function("icu4x/static/segmenter/word/overview", |b| {
            b.iter(|| {
                let provider = icu4x::WordSegmenter::get_static_provider();
                for case in tests.0.iter() {
                    let seg = icu4x::WordSegmenter::new_static(&provider);
                    for value in case.values.iter() {
                        let _ = seg.segment(value.input).count();
                    }
                }
            })
        });
        c.bench_function("icu4x/static/segmenter/line/overview", |b| {
            b.iter(|| {
                let provider = icu4x::WordSegmenter::get_static_provider();
                for case in tests.0.iter() {
                    let seg = icu4x::LineSegmenter::new_static(&provider);
                    for value in case.values.iter() {
                        let _ = seg.segment(value.input).count();
                    }
                }
            })
        });
    }

    #[cfg(feature = "icu4x-baked")]
    {
        c.bench_function("icu4x/baked/segmenter/word/overview", |b| {
            b.iter(|| {
                let provider = icu4x::WordSegmenter::get_baked_provider();
                for case in tests.0.iter() {
                    let seg = icu4x::WordSegmenter::new_baked(&provider);
                    for value in case.values.iter() {
                        let _ = seg.segment(value.input).count();
                    }
                }
            })
        });
        c.bench_function("icu4x/baked/segmenter/line/overview", |b| {
            b.iter(|| {
                let provider = icu4x::WordSegmenter::get_baked_provider();
                for case in tests.0.iter() {
                    let seg = icu4x::LineSegmenter::new_baked(&provider);
                    for value in case.values.iter() {
                        let _ = seg.segment(value.input).count();
                    }
                }
            })
        });
    }

    #[cfg(feature = "icu4c")]
    {
        c.bench_function("icu4c/static/segmenter/utf8/word/overview", |b| {
            b.iter(|| {
                for case in tests.0.iter() {
                    for value in case.values.iter() {
                        let seg = icu4c::Segmenter::new(case.langid, value.input, true);
                        let _ = seg.count();
                    }
                }
            })
        });
        c.bench_function("icu4c/static/segmenter/utf8/line/overview", |b| {
            b.iter(|| {
                for case in tests.0.iter() {
                    for value in case.values.iter() {
                        let seg = icu4c::Segmenter::new(case.langid, value.input, false);
                        let _ = seg.count();
                    }
                }
            })
        });

        let tests16: Vec<(&str, Vec<Vec<u16>>)> = tests
            .0
            .iter()
            .map(|c| {
                (
                    c.langid,
                    c.values
                        .iter()
                        .map(|v| v.input.encode_utf16().collect())
                        .collect(),
                )
            })
            .collect();

        c.bench_function("icu4c/static/segmenter/utf16/word/overview", |b| {
            b.iter(|| {
                for (langid, values) in &tests16 {
                    for value in values {
                        let seg = icu4c::Segmenter::new_utf16(langid, value, true);
                        let _ = seg.count();
                    }
                }
            })
        });
        c.bench_function("icu4c/static/segmenter/utf16/line/overview", |b| {
            b.iter(|| {
                for (langid, values) in &tests16 {
                    for value in values {
                        let seg = icu4c::Segmenter::new_utf16(langid, value, false);
                        let _ = seg.count();
                    }
                }
            })
        });
    }
}

criterion_group!(benches, number);
criterion_main!(benches);
