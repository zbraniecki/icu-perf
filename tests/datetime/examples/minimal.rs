use icu_perf_test_datetime::icu4x;
use icu_locid::langid;

fn main() {
    let epoch = 27832853;
    let langid = langid!("en");
    let provider = icu4x::DateTimeFormatter::get_static_provider();
    let dtf = icu4x::DateTimeFormatter::new_static(&provider, &langid, "Medium", "Medium");
    let _ = dtf.format(epoch);
}
