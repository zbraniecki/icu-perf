use icu_calendar::DateTime;
use icu_datetime::options::length;
use icu_locid::langid;
use icu_provider_blob::StaticDataProvider;

pub struct DateTimeFormatter {
    ptr: icu_datetime::DateTimeFormatter,
}

impl DateTimeFormatter {
    pub fn new(provider: &StaticDataProvider) -> Self {
        let langid = langid!("en");
        let options =
            length::Bag::from_date_time_style(length::Date::Medium, length::Time::Medium).into();
        let ptr = icu_datetime::DateTimeFormatter::try_new_with_buffer_provider(
            provider,
            &langid.into(),
            options,
        )
        .unwrap();
        Self { ptr }
    }

    pub fn format(&self) -> String {
        let epoch = 27832853;
        let datetime = DateTime::from_minutes_since_local_unix_epoch(epoch)
            .expect("Failed to construct DateTime.");
        let any_datetime = datetime.to_any();
        self.ptr.format_to_string(&any_datetime).unwrap()
    }
}
