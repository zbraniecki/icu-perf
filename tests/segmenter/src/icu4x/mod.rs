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

pub struct LineSegmenter {
    ptr: icu_segmenter::LineSegmenter,
}

impl LineSegmenter {
    #[cfg(feature = "icu4x-static")]
    pub fn get_static_provider() -> BlobDataProvider {
        BlobDataProvider::try_new_from_static_blob(&ICU4X_DATA).expect("Failed to load data")
    }

    #[cfg(feature = "icu4x-baked")]
    pub fn get_baked_provider() -> data::BakedDataProvider {
        data::BakedDataProvider
    }

    #[cfg(feature = "icu4x-static")]
    pub fn new_static(provider: &BlobDataProvider) -> Self {
        let ptr =
            icu_segmenter::LineSegmenter::try_new_auto_with_buffer_provider(provider).unwrap();
        Self { ptr }
    }

    #[cfg(feature = "icu4x-baked")]
    pub fn new_baked(provider: &data::BakedDataProvider) -> Self {
        let ptr = icu_segmenter::LineSegmenter::try_new_auto_with_any_provider(provider).unwrap();
        Self { ptr }
    }

    pub fn segment<'a>(&'a self, input: &'a str) -> impl Iterator<Item = usize> + 'a {
        self.ptr.segment_str(input)
    }
}

pub struct WordSegmenter {
    ptr: icu_segmenter::WordSegmenter,
}

impl WordSegmenter {
    #[cfg(feature = "icu4x-static")]
    pub fn get_static_provider() -> BlobDataProvider {
        BlobDataProvider::try_new_from_static_blob(&ICU4X_DATA).expect("Failed to load data")
    }

    #[cfg(feature = "icu4x-baked")]
    pub fn get_baked_provider() -> data::BakedDataProvider {
        data::BakedDataProvider
    }

    #[cfg(feature = "icu4x-static")]
    pub fn new_static(provider: &BlobDataProvider) -> Self {
        let ptr =
            icu_segmenter::WordSegmenter::try_new_auto_with_buffer_provider(provider).unwrap();
        Self { ptr }
    }

    #[cfg(feature = "icu4x-baked")]
    pub fn new_baked(provider: &data::BakedDataProvider) -> Self {
        let ptr = icu_segmenter::WordSegmenter::try_new_auto_with_any_provider(provider).unwrap();
        Self { ptr }
    }

    pub fn segment<'a>(&'a self, input: &'a str) -> impl Iterator<Item = usize> + 'a {
        self.ptr.segment_str(input)
    }
}

pub struct GraphemeClusterSegmenter {
    ptr: icu_segmenter::GraphemeClusterSegmenter,
}

impl GraphemeClusterSegmenter {
    #[cfg(feature = "icu4x-static")]
    pub fn get_static_provider() -> BlobDataProvider {
        BlobDataProvider::try_new_from_static_blob(&ICU4X_DATA).expect("Failed to load data")
    }

    #[cfg(feature = "icu4x-baked")]
    pub fn get_baked_provider() -> data::BakedDataProvider {
        data::BakedDataProvider
    }

    #[cfg(feature = "icu4x-static")]
    pub fn new_static(provider: &BlobDataProvider) -> Self {
        let ptr =
            icu_segmenter::GraphemeClusterSegmenter::try_new_with_buffer_provider(provider).unwrap();
        Self { ptr }
    }

    #[cfg(feature = "icu4x-baked")]
    pub fn new_baked(provider: &data::BakedDataProvider) -> Self {
        let ptr = icu_segmenter::GraphemeClusterSegmenter::try_new_with_any_provider(provider).unwrap();
        Self { ptr }
    }

    pub fn segment<'a>(&'a self, input: &'a str) -> impl Iterator<Item = usize> + 'a {
        self.ptr.segment_str(input)
    }
}
