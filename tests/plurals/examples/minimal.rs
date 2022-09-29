use icu_locid::langid;
use icu_perf_test_plurals::icu4x;

fn main() {
    let value = 27832853.0;
    let langid = langid!("en");
    let provider = icu4x::PluralRules::get_static_provider();
    let nf = icu4x::PluralRules::new_static(&provider, &langid);
    let _ = nf.select(value);
}
