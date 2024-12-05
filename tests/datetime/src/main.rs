#[cfg(feature = "icu4c")]
use icu_perf_test_datetime::icu4c;

#[cfg(feature = "icu4x")]
use icu_perf_test_datetime::icu4x;

fn main() {
    let epoch = 27832853;
    let langid = "en";

    #[cfg(feature = "icu4c")]
    {
        let dtf = icu4c::DateTimeFormatter::new(langid, "Medium", "Medium");
        let result = dtf.format(epoch);
        println!("ICU4C: {}", result);
    }

    #[cfg(feature = "icu4x")]
    {
        use icu_locid::LanguageIdentifier;

        let en: LanguageIdentifier = langid.parse().unwrap();
        #[cfg(feature = "icu4x-static")]
        {
            let provider = icu4x::DateTimeFormatter::get_static_provider();
            let dtf = icu4x::DateTimeFormatter::new_static(&provider, &en, "Medium");
            let result = dtf.format(epoch);
            println!("ICU4X (static): {}", result);
        }

        #[cfg(feature = "icu4x-baked")]
        {
            let provider = icu4x::DateTimeFormatter::get_baked_provider();
            let dtf = icu4x::DateTimeFormatter::new_baked(&provider, &en, "Medium");
            let result = dtf.format(epoch);
            println!("ICU4X (baked): {}", result);
        }
    }
}
