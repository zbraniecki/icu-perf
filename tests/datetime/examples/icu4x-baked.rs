use icu_locid::langid;
use icu_perf_test_datetime::icu4x;

fn main() {
    let langid = langid!("en");
    let date = 27832853;
    let date_style = "Medium";
    let time_style = "None";

    let mut s = "".to_string();
    for _ in 0..100000 {
        let dtf = icu4x::DateTimeFormatter::new_baked(&langid, date_style, time_style);
        s = dtf.format(date);
    }
    println!("{s}");
}
