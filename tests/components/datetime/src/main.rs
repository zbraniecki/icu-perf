use icu_datetime::{options::length, DateTimeFormatter};
use icu_locid::langid;
use icu_provider_blob::StaticDataProvider;

const ICU4X_DATA: &[u8] = include_bytes!(concat!("../../../../data/icu4x-1.0.postcard"));

fn main() {
    let provider =
        StaticDataProvider::try_new_from_static_blob(&ICU4X_DATA).expect("Failed to load data");
    let langid = langid!("en");
    let options =
        length::Bag::from_date_time_style(length::Date::Long, length::Time::Medium).into();
    let dtf = DateTimeFormatter::try_new_with_buffer_provider(&provider, &langid.into(), options)
        .unwrap();
}
