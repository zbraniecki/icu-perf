use icu_perf_test_segmenter::icu4x;

fn main() {
    let value = "Hello World";
    let provider = icu4x::WordSegmenter::get_static_provider();
    let seg = icu4x::WordSegmenter::new_static(&provider);
    let _: Vec<_> = seg.segment(value).collect();

    let provider = icu4x::LineSegmenter::get_static_provider();
    let seg = icu4x::LineSegmenter::new_static(&provider);
    let _: Vec<_> = seg.segment(value).collect();

    let provider = icu4x::GraphemeClusterSegmenter::get_static_provider();
    let seg = icu4x::GraphemeClusterSegmenter::new_static(&provider);
    let _: Vec<_> = seg.segment(value).collect();

    let provider = icu4x::GraphemeClusterSegmenter::get_static_provider();
    let seg = icu4x::GraphemeClusterSegmenter::new_static(&provider);
    let _: Vec<_> = seg.segment(value).collect();

    let provider = icu4x::LstmSegmenter::get_static_provider();
    let seg = icu4x::LstmSegmenter::new_static(&provider);
    let _: Vec<_> = seg.segment(value).collect();
}
