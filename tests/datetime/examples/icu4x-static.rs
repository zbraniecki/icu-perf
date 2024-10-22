use icu_locale::langid;
use icu_perf_test_datetime::icu4x;

fn main() {
    let langid = langid!("en");
    let date = 27832853;
    let length = "Short";

    let provider = icu4x::DateTimeFormatter::get_static_provider();
    let dtf = icu4x::DateTimeFormatter::new_static(&provider, &langid, length);
    let result = dtf.format(date);
    println!("{}", result);
}
