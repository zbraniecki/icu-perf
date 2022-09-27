#[cfg(feature = "icu4c")]
use icu_perf_test_locid::icu4c;

#[cfg(feature = "icu4x")]
use icu_perf_test_locid::icu4x;

fn main() {
    let langid = "en_US";

    #[cfg(feature = "icu4c")]
    {
        let result = icu4c::LanguageIdentifier::canonicalize(langid);
        println!("ICU4C: {:?}", result);
    }

    #[cfg(feature = "icu4x")]
    {
        let result = icu4x::LanguageIdentifier::canonicalize(langid);
        println!("ICU4X: {:?}", result);
    }
}
