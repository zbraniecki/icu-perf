use fixed_decimal::{DoublePrecision, FixedDecimal};
use icu_decimal::options::FixedDecimalFormatterOptions;

#[cfg(feature = "icu4x-baked")]
pub mod data {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../data/icu4x-1.2.rs/mod.rs"
    ));
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../data/icu4x-1.2.rs/any.rs"
    ));
}

#[cfg(feature = "icu4x-static")]
use icu_provider_blob::BlobDataProvider;

#[cfg(feature = "icu4x-static")]
const ICU4X_DATA: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../data/icu4x-1.2.postcard"
));

pub struct NumberFormatter {
    ptr: icu_decimal::FixedDecimalFormatter,
}

impl NumberFormatter {
    #[cfg(feature = "icu4x-static")]
    pub fn get_static_provider() -> BlobDataProvider {
        BlobDataProvider::try_new_from_static_blob(&ICU4X_DATA).expect("Failed to load data")
    }

    #[cfg(feature = "icu4x-baked")]
    pub fn get_baked_provider() -> data::BakedDataProvider {
        data::BakedDataProvider
    }

    #[cfg(feature = "icu4x-static")]
    pub fn new_static(provider: &BlobDataProvider, langid: &icu_locid::LanguageIdentifier, options: FixedDecimalFormatterOptions) -> Self {
        let ptr = icu_decimal::FixedDecimalFormatter::try_new_with_buffer_provider(
            provider,
            &langid.into(),
            options,
        )
        .unwrap();
        Self { ptr }
    }

    #[cfg(feature = "icu4x-baked")]
    pub fn new_baked(
        provider: &data::BakedDataProvider,
        langid: &icu_locid::LanguageIdentifier,
        options: FixedDecimalFormatterOptions,
    ) -> Self {
        let ptr = icu_decimal::FixedDecimalFormatter::try_new_with_any_provider(
            provider,
            &langid.into(),
            options,
        )
        .unwrap();
        Self { ptr }
    }

    pub fn format(&self, input: f64) -> String {
        use writeable::Writeable;

        let decimal = FixedDecimal::try_from_f64(input, DoublePrecision::Floating)
            .expect("Finite quantity with limited precision");

        self.ptr.format(&decimal).write_to_string().to_string()
    }
}
