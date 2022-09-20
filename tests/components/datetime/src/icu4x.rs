use icu_calendar::DateTime;
use icu_datetime::options::length;
use icu_locid::langid;
use icu_provider_blob::StaticDataProvider;

const ICU4X_DATA: &[u8] = include_bytes!(concat!("../../../../data/icu4x-1.0.postcard"));

pub struct DateTimeFormatter {
    ptr: icu_datetime::DateTimeFormatter,
}

impl DateTimeFormatter {
    pub fn new() -> Self {
        let provider =
            StaticDataProvider::try_new_from_static_blob(&ICU4X_DATA).expect("Failed to load data");
        let langid = langid!("en");
        let options =
            length::Bag::from_date_time_style(length::Date::Long, length::Time::Medium).into();
        let ptr = icu_datetime::DateTimeFormatter::try_new_with_buffer_provider(
            &provider,
            &langid.into(),
            options,
        )
        .unwrap();
        Self { ptr }
    }

    pub fn format(&self) -> String {
        let datetime = DateTime::new_iso_datetime(2020, 9, 1, 12, 34, 28)
            .expect("Failed to construct DateTime.");
        let any_datetime = datetime.to_any();
        self.ptr.format_to_string(&any_datetime).unwrap()
    }
}
