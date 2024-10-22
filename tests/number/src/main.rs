#[cfg(feature = "icu4c")]
use icu_perf_test_number::icu4c;

#[cfg(feature = "icu4x")]
use icu_perf_test_number::icu4x;
#[cfg(feature = "icu4x")]
use icu_decimal::options::{FixedDecimalFormatterOptions, GroupingStrategy};

fn main() {
    let value = 27832853.0;
    let langid = "en";

    #[cfg(feature = "icu4c-unum")]
    {
        let nf = icu4c::NumberFormatter::new(langid);
        let result = nf.format(value);
        println!("ICU4C: {}", result);
    }

    #[cfg(feature = "icu4c-simple")]
    {
        let nf = icu4c::SimpleNumberFormatter::new(langid);
        let result = nf.format_i64(value as i64);
        println!("ICU4C: {}", result);
    }

    #[cfg(feature = "icu4x")]
    {
        use icu_locale::LanguageIdentifier;

        let en: LanguageIdentifier = langid.parse().unwrap();
        #[cfg(feature = "icu4x-static")]
        {
            let mut options: FixedDecimalFormatterOptions = Default::default();
            options.grouping_strategy = GroupingStrategy::Auto;

            let provider = icu4x::NumberFormatter::get_static_provider();
            let nf = icu4x::NumberFormatter::new_static(&provider, &en, options);
            let result = nf.format(value);
            println!("ICU4X (static): {}", result);
        }

        #[cfg(feature = "icu4x-baked")]
        {
            let mut options: FixedDecimalFormatterOptions = Default::default();
            options.grouping_strategy = GroupingStrategy::Auto;

            let nf = icu4x::NumberFormatter::new_baked(&en, options);
            let result = nf.format(value);
            println!("ICU4X (static): {}", result);
        }
    }
}
