use icu_perf_test_datetime::icu4x;
use icu_locid::langid;

fn main() {
    let langid = langid!("en");
    let date = 27832853;

    let provider = icu4x::DateTimeFormatter::get_static_provider();
    let dtf = icu4x::DateTimeFormatter::new_static(&provider, &langid);
    let result = dtf.format(date);
    println!("{}", result);
}
