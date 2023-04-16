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
        for case in tests.0.iter() {
            for mode in &case.modes {
                c.bench_function(&format!("icu4x/{}/static/segmenter/word/{}/overview", case.langid, mode), |b| {
                    b.iter(|| {
                        let provider = icu4x::WordSegmenter::get_static_provider();
                        let seg = match mode {
                            data::Mode::Auto => icu4x::WordSegmenter::new_auto_static(&provider),
                            data::Mode::Dictionary => icu4x::WordSegmenter::new_dictionary_static(&provider),
                            data::Mode::Lstm => icu4x::WordSegmenter::new_lstm_static(&provider),
                        };
                        for value in case.values.iter() {
                            let _ = seg.segment(black_box(value.input)).count();
                        }
                    })
                });
                c.bench_function(&format!("icu4x/{}/static/segmenter/line/{}/overview", case.langid, mode), |b| {
                    b.iter(|| {
                        let provider = icu4x::LineSegmenter::get_static_provider();
                        let seg = match mode {
                            data::Mode::Auto => icu4x::LineSegmenter::new_auto_static(&provider, case.ja_zh),
                            data::Mode::Dictionary => icu4x::LineSegmenter::new_dictionary_static(&provider, case.ja_zh),
                            data::Mode::Lstm => icu4x::LineSegmenter::new_lstm_static(&provider, case.ja_zh),
                        };
                        for value in case.values.iter() {
                            let _ = seg.segment(black_box(value.input)).count();
                        }
                    })
                });
            }
        }
    }

    #[cfg(feature = "icu4x-baked")]
    {
        for case in tests.0.iter() {
            for mode in &case.modes {
                c.bench_function(&format!("icu4x/{}/baked/segmenter/word/{}/overview", case.langid, mode), |b| {
                    b.iter(|| {
                        let provider = icu4x::WordSegmenter::get_baked_provider();
                        let seg = match mode {
                            data::Mode::Auto => icu4x::WordSegmenter::new_auto_baked(&provider),
                            data::Mode::Dictionary => icu4x::WordSegmenter::new_dictionary_baked(&provider),
                            data::Mode::Lstm => icu4x::WordSegmenter::new_lstm_baked(&provider),
                        };
                        for value in case.values.iter() {
                            let _ = seg.segment(black_box(value.input)).count();
                        }
                    })
                });
                c.bench_function(&format!("icu4x/{}/baked/segmenter/line/{}/overview", case.langid, mode), |b| {
                    b.iter(|| {
                        let provider = icu4x::LineSegmenter::get_baked_provider();
                        let seg = match mode {
                            data::Mode::Auto => icu4x::LineSegmenter::new_auto_baked(&provider, case.ja_zh),
                            data::Mode::Dictionary => icu4x::LineSegmenter::new_dictionary_baked(&provider, case.ja_zh),
                            data::Mode::Lstm => icu4x::LineSegmenter::new_lstm_baked(&provider, case.ja_zh),
                        };
                        for value in case.values.iter() {
                            let _ = seg.segment(black_box(value.input)).count();
                        }
                    })
                });
            }
        }
    }

    #[cfg(feature = "icu4c")]
    {
        for case in tests.0.iter() {
            c.bench_function(&format!("icu4c/{}/static/segmenter/utf8/word/overview", case.langid), |b| {
                b.iter(|| {
                    for value in case.values.iter() {
                        let seg = icu4c::Segmenter::new(case.langid, black_box(value.input), true);
                        let _ = seg.count();
                    }
                })
            });
            c.bench_function(&format!("icu4c/{}/static/segmenter/utf8/line/overview", case.langid), |b| {
                b.iter(|| {
                    for value in case.values.iter() {
                        let seg = icu4c::Segmenter::new(case.langid, black_box(value.input), false);
                        let _ = seg.count();
                    }
                })
            });
        }

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

        for (langid, values) in &tests16 {
            c.bench_function(&format!("icu4c/{}/static/segmenter/utf16/word/overview", langid), |b| {
                b.iter(|| {
                    for value in values {
                        let seg = icu4c::Segmenter::new_utf16(langid, black_box(value), true);
                        let _ = seg.count();
                    }
                })
            });
            c.bench_function(&format!("icu4c/{}/static/segmenter/utf16/line/overview", langid), |b| {
                b.iter(|| {
                    for value in values {
                        let seg = icu4c::Segmenter::new_utf16(langid, black_box(value), false);
                        let _ = seg.count();
                    }
                })
            });
        }
    }
}

criterion_group!(benches, number);
criterion_main!(benches);
