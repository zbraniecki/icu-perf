use icu_locid::langid;
use icu_perf_test_datetime::icu4x;

fn main() {
    let langid = langid!("en");
    let date = 27832853;
    let date_style = "Short";
    let time_style = "Short";

    let provider = icu4x::DateTimeFormatter::get_baked_provider();
    let dtf = icu4x::DateTimeFormatter::new_baked(&provider, &langid, date_style, time_style);
    let result = dtf.format(date);
    println!("{}", result);
}
