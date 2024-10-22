use icu_perf_test_datetime::icu4c;

fn main() {
    let langid = "en";
    let date = 27832853;

    let mut s = "".to_string();
    for _ in 0..100000 {
        let dtf = icu4c::DateTimeFormatter::new(langid, "Medium", "None");
        s = dtf.format(date);
    }
    println!("{s}");
}
