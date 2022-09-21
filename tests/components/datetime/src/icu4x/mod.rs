#[cfg(feature = "icu4x-baked")]
pub mod data {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/icu4x/data/mod.rs"
    ));
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/icu4x/data/any.rs"
    ));
}

use icu_calendar::DateTime;
use icu_datetime::options::length;
use icu_locid::langid;

#[cfg(feature = "icu4x-static")]
use icu_provider_blob::StaticDataProvider;

#[cfg(feature = "icu4x-static")]
const ICU4X_DATA: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../../data/icu4x-1.0.postcard"
));

pub struct DateTimeFormatter {
    ptr: icu_datetime::DateTimeFormatter,
}

impl DateTimeFormatter {
    #[cfg(feature = "icu4x-static")]
    pub fn get_static_provider() -> StaticDataProvider {
        StaticDataProvider::try_new_from_static_blob(&ICU4X_DATA).expect("Failed to load data")
    }

    #[cfg(feature = "icu4x-baked")]
    pub fn get_baked_provider() -> data::BakedDataProvider {
        data::BakedDataProvider
    }

    #[cfg(feature = "icu4x-static")]
    pub fn new_static(provider: &StaticDataProvider) -> Self {
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

    #[cfg(feature = "icu4x-baked")]
    pub fn new_baked(provider: &data::BakedDataProvider) -> Self {
        let langid = langid!("en");
        let options =
            length::Bag::from_date_time_style(length::Date::Medium, length::Time::Medium).into();
        let ptr = icu_datetime::DateTimeFormatter::try_new_with_any_provider(
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
