use std::cmp::Ordering;

#[cfg(feature = "icu4x-static")]
use icu_provider_blob::BlobDataProvider;

#[cfg(feature = "icu4x-static")]
const ICU4X_DATA: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../data/icu4x-2.0-alpha.postcard"
));

pub struct Collator {
    ptr: icu_collator::Collator,
}

impl Collator {
    #[cfg(feature = "icu4x-static")]
    pub fn get_static_provider() -> BlobDataProvider {
        BlobDataProvider::try_new_from_static_blob(&ICU4X_DATA).expect("Failed to load data")
    }

    #[cfg(feature = "icu4x-static")]
    pub fn new_static(provider: &BlobDataProvider, langid: &icu_locale::LanguageIdentifier) -> Self {
        let options = icu_collator::CollatorOptions::new();
        let ptr =
            icu_collator::Collator::try_new_with_buffer_provider(provider, &langid.into(), options)
                .unwrap();
        Self { ptr }
    }

    #[cfg(feature = "icu4x-baked")]
    pub fn new_baked(
        langid: &icu_locale::LanguageIdentifier,
    ) -> Self {
        let options = icu_collator::CollatorOptions::new();
        let ptr =
            icu_collator::Collator::try_new(&langid.into(), options)
                .unwrap().static_to_owned();
        Self { ptr }
    }

    pub fn compare(&self, left: &str, right: &str) -> Ordering {
        self.ptr.as_borrowed().compare(left, right)
    }
}
