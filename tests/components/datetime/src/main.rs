#[cfg(feature = "icu4c")]
pub mod icu4c;

#[cfg(feature = "icu4x")]
pub mod icu4x;

fn main() {
    #[cfg(feature = "icu4c")]
    {
        let dtf = icu4c::DateTimeFormatter::new();
        let result = dtf.format();
        println!("ICU4C: {}", result);
    }

    #[cfg(feature = "icu4x")]
    {
        const ICU4X_DATA: &[u8] = include_bytes!(concat!("../../../../data/icu4x-1.0.postcard"));
        use icu_provider_blob::StaticDataProvider;
        let provider =
            StaticDataProvider::try_new_from_static_blob(&ICU4X_DATA).expect("Failed to load data");
        let dtf = icu4x::DateTimeFormatter::new(&provider);
        let result = dtf.format();
        println!("ICU4X: {}", result);
    }
}
