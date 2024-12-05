#[cfg(feature = "icu4c")]
use icu_perf_test_collator::icu4c;

#[cfg(feature = "icu4x")]
use icu_perf_test_collator::icu4x;

fn main() {
    let langid = "und";
    let left = "hello";
    let right = "Hello";

    #[cfg(feature = "icu4c")]
    {
        let col = icu4c::Collator::new(langid);
        let result = col.compare(left, right);
        println!("ICU4C: {:?}", result);
    }

    #[cfg(feature = "icu4x")]
    {
        use icu_locale::LanguageIdentifier;

        let en: LanguageIdentifier = langid.parse().unwrap();
        #[cfg(feature = "icu4x-static")]
        {
            let provider = icu4x::Collator::get_static_provider();
            let col = icu4x::Collator::new_static(&provider, &en);
            let result = col.compare(left, right);
            println!("ICU4X (static): {:?}", result);
        }

        #[cfg(feature = "icu4x-baked")]
        {
            let col = icu4x::Collator::new_baked(&en);
            let result = col.compare(left, right);
            println!("ICU4X (static): {:?}", result);
        }
    }
}
