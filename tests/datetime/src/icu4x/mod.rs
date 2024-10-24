use icu_calendar::DateTime;
use icu_datetime::options::length;
use icu_locid::LanguageIdentifier;

#[cfg(feature = "icu4x-static")]
use icu_provider_blob::BlobDataProvider;

#[cfg(feature = "icu4x-static")]
const ICU4X_DATA: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../data/icu4x-1.5.postcard"
));

fn get_date_length(input: &str) -> Option<length::Date> {
    match input {
        "None" => None,
        "Short" => Some(length::Date::Short),
        "Medium" => Some(length::Date::Medium),
        "Long" => Some(length::Date::Long),
        "Full" => Some(length::Date::Full),
        _ => unreachable!(),
    }
}

fn get_time_length(input: &str) -> Option<length::Time> {
    match input {
        "None" => None,
        "Short" => Some(length::Time::Short),
        "Medium" => Some(length::Time::Medium),
        "Long" => Some(length::Time::Long),
        "Full" => Some(length::Time::Full),
        _ => unreachable!(),
    }
}

pub struct DateTimeFormatter {
    ptr: icu_datetime::DateTimeFormatter,
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
        date_length: &str,
        time_length: &str,
    ) -> Self {
        let mut options = length::Bag::default();
        options.date = get_date_length(date_length);
        options.time = get_time_length(time_length);
        let ptr = icu_datetime::DateTimeFormatter::try_new_with_buffer_provider(
            provider,
            &langid.into(),
            options.into(),
        )
        .unwrap();
        Self { ptr }
    }

    #[cfg(feature = "icu4x-baked")]
    pub fn new_baked(
        langid: &LanguageIdentifier,
        date_length: &str,
        time_length: &str,
    ) -> Self {
        let mut options = length::Bag::default();
        options.date = get_date_length(date_length);
        options.time = get_time_length(time_length);
        let ptr = icu_datetime::DateTimeFormatter::try_new(
            &langid.into(),
            options.into(),
        )
        .unwrap();
        Self { ptr }
    }

    pub fn format(&self, input: i32) -> String {
        let datetime = DateTime::from_minutes_since_local_unix_epoch(input);
        let any_datetime = datetime.to_any();
        self.ptr.format_to_string(&any_datetime).unwrap()
    }
}
