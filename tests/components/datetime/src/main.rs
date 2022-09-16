use std::ffi::CString;
use std::os::raw::c_char;

#[link(name = "icui18n")]
extern "C" {
    // _CAPI UDateFormat* U_EXPORT2
    // udat_open(UDateFormatStyle  timeStyle,
    //           UDateFormatStyle  dateStyle,
    //           const char        *locale,
    //           const UChar       *tzID,
    //           int32_t           tzIDLength,
    //           const UChar       *pattern,
    //           int32_t           patternLength,
    //           UErrorCode        *status)
    pub fn udat_open_72(
        timeStyle: *const libc::c_int,
        dateStyle: *const libc::c_int,
        locale: *const libc::c_char,
        tzID: *const libc::c_int,
        tzIDLength: libc::c_int,
        pattern: *const libc::c_char,
        patternLength: libc::c_int,
        status: *mut libc::c_int,
    ) -> libc::c_int;
    pub fn udat_close_72(format: *const libc::c_int);
    //
    // U_CAPI int32_t U_EXPORT2
    // udat_format(    const    UDateFormat*    format,
    //         UDate           dateToFormat,
    //         UChar*          result,
    //         int32_t         resultLength,
    //         UFieldPosition* position,
    //         UErrorCode*     status)
    pub fn udat_format_72(
        format: *const libc::c_int,
        dateToFormat: *const libc::c_int,
        result: *mut libc::c_char,
        resultLength: *mut libc::c_int,
        position: *const libc::c_int,
        status: *mut libc::c_int,
    ) -> libc::c_int;
}

// use icu_datetime::{options::length, DateTimeFormatter};
// use icu_locid::langid;
// use icu_provider_blob::StaticDataProvider;

// const ICU4X_DATA: &[u8] = include_bytes!(concat!("../../../../data/icu4x-1.0.postcard"));

fn main() {
    let date_style = 0;
    let time_style = 0;
    let locale = CString::new("en-US").unwrap();
    let tz_id = 0;
    let tz_id_length = 0;
    let pattern = 0;
    let pattern_length = 0;
    let mut status = 0;
    let dtf = unsafe {
        udat_open_72(
            &date_style,
            &time_style,
            locale.as_ptr(),
            &tz_id,
            tz_id_length,
            &pattern,
            pattern_length,
            &mut status,
        )
    };

    let result = CString::default();
    let ptr = result.into_raw();
    let mut result_length = 0;

    unsafe {
        let date_to_format = 0;
        let position = 0;
        udat_format_72(
            &dtf,
            &date_to_format,
            ptr,
            &mut result_length,
            &position,
            &mut status,
        );
    }

    unsafe {
        udat_close_72(&dtf);
    }

    let result = unsafe { CString::from_raw(ptr) }.into_string().unwrap();
    println!("{}", result);
    // let provider =
    //     StaticDataProvider::try_new_from_static_blob(&ICU4X_DATA).expect("Failed to load data");
    // let langid = langid!("en");
    // let options =
    //     length::Bag::from_date_time_style(length::Date::Long, length::Time::Medium).into();
    // let dtf = DateTimeFormatter::try_new_with_buffer_provider(&provider, &langid.into(), options)
    //     .unwrap();
}
