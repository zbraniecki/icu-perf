#[cfg(feature = "icu4c")]
use icu_perf_test_datetime::icu4c;

#[cfg(feature = "icu4x")]
use icu_perf_test_datetime::icu4x;

fn main() {
    let epoch = 27832853;
    let langid = "en";

    #[cfg(feature = "icu4c")]
    {
        let dtf = icu4c::DateTimeFormatter::new(langid);
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
            let dtf = icu4x::DateTimeFormatter::new_static(&provider, &en, "None", "None");
            let result = dtf.format(epoch);
            println!("ICU4X (static): {}", result);
        }

        #[cfg(feature = "icu4x-baked")]
        {
            let provider = icu4x::DateTimeFormatter::get_baked_provider();
            let dtf = icu4x::DateTimeFormatter::new_baked(&provider, &en, "None", "None");
            let result = dtf.format(epoch);
            println!("ICU4X (baked): {}", result);
        }
    }

    // #[cfg(feature = "rust_icu")]
    // {
    //     use rust_icu_udat::UDateFormat;
    //     use rust_icu_sys::UDateFormatStyle;
    //     use rust_icu_uloc::ULoc;
    //
    //     let loc = ULoc::for_language_tag("en");
    //     let dtf = UDateFormat::new_with_style(
    //         UDateFormatStyle::UDAT_MEDIUM,
    //         UDateFormatStyle::UDAT_MEDIUM,
    //         loc,
    //         "".try_into().unwrap(),
    //     );
    //     let dtf = icu4c::DateTimeFormatter::new(langid);
    //     let result = dtf.format(epoch);
    //     println!("ICU4C: {}", result);
    // }
}
