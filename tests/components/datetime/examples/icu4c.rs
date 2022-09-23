use icu_perf_test_datetime::icu4c;

fn main() {
    let langid = "en";
    let date = 27832853;

    let dtf = icu4c::DateTimeFormatter::new(langid);
    let result = dtf.format(date);
    println!("{}", result);
}
