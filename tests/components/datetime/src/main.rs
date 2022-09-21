#[cfg(feature = "icu4c")]
use icu_perf_test_datetime::icu4c;

#[cfg(feature = "icu4x")]
use icu_perf_test_datetime::icu4x;

fn main() {
    #[cfg(feature = "icu4c")]
    {
        let dtf = icu4c::DateTimeFormatter::new();
        let result = dtf.format();
        println!("ICU4C: {}", result);
    }

    #[cfg(feature = "icu4x")]
    {
        #[cfg(feature = "icu4x-static")]
        {
            let provider = icu4x::DateTimeFormatter::get_static_provider();
            let dtf = icu4x::DateTimeFormatter::new_static(&provider);
            let result = dtf.format();
            println!("ICU4X (static): {}", result);
        }

        #[cfg(feature = "icu4x-baked")]
        {
            let provider = icu4x::DateTimeFormatter::get_baked_provider();
            let dtf = icu4x::DateTimeFormatter::new_baked(&provider);
            let result = dtf.format();
            println!("ICU4X (baked): {}", result);
        }
    }
}
