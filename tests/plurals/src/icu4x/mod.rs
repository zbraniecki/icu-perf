use crate::Category;
use fixed_decimal::{DoublePrecision, FixedDecimal};
use icu_plurals::PluralCategory;

#[cfg(feature = "icu4x-baked")]
pub mod data {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../data/icu4x-1.0.rs/mod.rs"
    ));
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../data/icu4x-1.0.rs/any.rs"
    ));
}

#[cfg(feature = "icu4x-static")]
use icu_provider_blob::BlobDataProvider;

#[cfg(feature = "icu4x-static")]
const ICU4X_DATA: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../data/icu4x-1.0-plurals.postcard"
));

pub struct PluralRules {
    ptr: icu_plurals::PluralRules,
}

impl PluralRules {
    #[cfg(feature = "icu4x-static")]
    pub fn get_static_provider() -> BlobDataProvider {
        BlobDataProvider::try_new_from_static_blob(&ICU4X_DATA).expect("Failed to load data")
    }

    #[cfg(feature = "icu4x-baked")]
    pub fn get_baked_provider() -> data::BakedDataProvider {
        data::BakedDataProvider
    }

    #[cfg(feature = "icu4x-static")]
    pub fn new_static(provider: &BlobDataProvider, langid: &icu_locid::LanguageIdentifier) -> Self {
        let ptr = icu_plurals::PluralRules::try_new_cardinal_with_buffer_provider(
            provider,
            &langid.into(),
        )
        .unwrap();
        Self { ptr }
    }

    #[cfg(feature = "icu4x-baked")]
    pub fn new_baked(
        provider: &data::BakedDataProvider,
        langid: &icu_locid::LanguageIdentifier,
    ) -> Self {
        let ptr =
            icu_plurals::PluralRules::try_new_cardinal_with_any_provider(provider, &langid.into())
                .unwrap();
        Self { ptr }
    }

    pub fn select(&self, input: f64) -> Category {
        let decimal = FixedDecimal::try_from_f64(input, DoublePrecision::Floating)
            .expect("Finite quantity with limited precision");

        let category = self.ptr.category_for(&decimal.into());
        match category {
            PluralCategory::Zero => Category::Zero,
            PluralCategory::One => Category::One,
            PluralCategory::Two => Category::Two,
            PluralCategory::Few => Category::Few,
            PluralCategory::Many => Category::Many,
            PluralCategory::Other => Category::Other,
        }
    }
}
