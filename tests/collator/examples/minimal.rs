use icu_perf_test_collator::icu4x;
use icu_locid::LanguageIdentifier;

fn main() {
    let langid = "und";
    let left = "hello";
    let right = "Hello";

    let en: LanguageIdentifier = langid.parse().unwrap();
    let provider = icu4x::Collator::get_static_provider();
    let col = icu4x::Collator::new_static(&provider, &en);
    let _ = col.compare(left, right);
}
