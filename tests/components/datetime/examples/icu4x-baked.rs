use icu_perf_test_datetime::icu4x;
use icu_locid::langid;

fn main() {
    let langid = langid!("en");
    let date = 27832853;

    let provider = icu4x::DateTimeFormatter::get_baked_provider();
    let dtf = icu4x::DateTimeFormatter::new_baked(&provider, &langid);
    let result = dtf.format(date);
    println!("{}", result);
}
