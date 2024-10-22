use icu_calendar::DateTime;
use icu_locale::LanguageIdentifier;
use icu_datetime::neo_skeleton::NeoSkeletonLength;
use icu_datetime::fieldset;

#[cfg(feature = "icu4x-static")]
use icu_provider_blob::BlobDataProvider;

#[cfg(feature = "icu4x-static")]
const ICU4X_DATA: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../data/icu4x-2.0-alpha.postcard"
));

fn get_length(input: &str) -> NeoSkeletonLength {
    match input {
        "Short" => NeoSkeletonLength::Short,
        "Medium" => NeoSkeletonLength::Medium,
        "Long" => NeoSkeletonLength::Long,
        _ => panic!("Unknown length: {input}"),
    }
}

pub struct DateTimeFormatter {
    ptr: icu_datetime::DateTimeFormatter<fieldset::YMD>,
}

impl DateTimeFormatter {
    #[cfg(feature = "icu4x-static")]
    pub fn get_static_provider() -> BlobDataProvider {
        BlobDataProvider::try_new_from_static_blob(&ICU4X_DATA).expect("Failed to load data")
    }

    #[cfg(feature = "icu4x-static")]
    pub fn new_static(
        provider: &BlobDataProvider,
        langid: &LanguageIdentifier,
        length: &str,
    ) -> Self {
        let options = fieldset::YMD::with_length(get_length(length));
        let ptr = icu_datetime::DateTimeFormatter::try_new_with_buffer_provider(
            provider,
            &langid.into(),
            options,
        )
        .unwrap();
        Self { ptr }
    }

    #[cfg(feature = "icu4x-baked")]
    pub fn new_baked(
        langid: &LanguageIdentifier,
        length: &str,
    ) -> Self {
        let options = fieldset::YMD::with_length(get_length(length));
        let ptr = icu_datetime::DateTimeFormatter::try_new(
            &langid.into(),
            options,
        )
        .unwrap();
        Self { ptr }
    }

    pub fn format(&self, input: i32) -> String {
        use writeable::TryWriteable;
        let datetime = DateTime::from_minutes_since_local_unix_epoch(input);
        let any_datetime = datetime.to_any();
        self.ptr.convert_and_format(&any_datetime).try_write_to_string().unwrap().into_owned()
    }
}
