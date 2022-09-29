#[cfg(feature = "icu4c")]
use icu_perf_test_plurals::icu4c;

#[cfg(feature = "icu4x")]
use icu_perf_test_plurals::icu4x;

fn main() {
    let value = 2.0;
    let langid = "en";

    #[cfg(feature = "icu4c")]
    {
        let pr = icu4c::PluralRules::new(langid, true);
        let result = pr.select(value);
        println!("ICU4C: {:?}", result);
    }

    #[cfg(feature = "icu4x")]
    {
        use icu_locid::LanguageIdentifier;

        let en: LanguageIdentifier = langid.parse().unwrap();
        #[cfg(feature = "icu4x-static")]
        {
            let provider = icu4x::PluralRules::get_static_provider();
            let pr = icu4x::PluralRules::new_static(&provider, &en, true);
            let result = pr.select(value);
            println!("ICU4X (static): {:?}", result);
        }

        #[cfg(feature = "icu4x-baked")]
        {
            let provider = icu4x::PluralRules::get_baked_provider();
            let pr = icu4x::PluralRules::new_baked(&provider, &en, true);
            let result = pr.select(value);
            println!("ICU4X (static): {:?}", result);
        }
    }
}
