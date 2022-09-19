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
        locale: *const c_char,
        tzID: *const libc::c_int,
        tzIDLength: libc::c_int,
        pattern: *const libc::c_char,
        patternLength: libc::c_int,
        status: *mut libc::c_int,
    ) -> *const libc::c_void;
    pub fn udat_close_72(format: *const libc::c_void);
    //
    // U_CAPI int32_t U_EXPORT2
    // udat_format(    const    UDateFormat*    format,
    //         UDate           dateToFormat,
    //         UChar*          result,
    //         int32_t         resultLength,
    //         UFieldPosition* position,
    //         UErrorCode*     status)
    pub fn udat_format_72(
        format: *const libc::c_void,
        dateToFormat: libc::c_float,
        result: *mut c_char,
        resultLength: libc::c_int,
        position: *const i32,
        status: *mut libc::c_int,
    ) -> libc::c_int;
}

// use icu_datetime::{options::length, DateTimeFormatter};
// use icu_locid::langid;
// use icu_provider_blob::StaticDataProvider;

// const ICU4X_DATA: &[u8] = include_bytes!(concat!("../../../../data/icu4x-1.0.postcard"));

fn main() {
    let date_style = 2;
    let time_style = 2;
    let locale = CString::new("en-US").unwrap();
    let mut status = 0;
    let dtf = unsafe {
        udat_open_72(
            &date_style,
            &time_style,
            locale.as_ptr(),
            std::ptr::null(),
            -1,
            std::ptr::null(),
            -1,
            &mut status,
        )
    };

    let result = CString::default();
    let ptr = result.into_raw();
    let result_length = 30;
    let date_to_format = 100000000.0;

    unsafe {
        udat_format_72(
            dtf,
            date_to_format,
            ptr,
            result_length,
            std::ptr::null(),
            &mut status,
        )
    };
    println!("FOO");

    // unsafe {
    //     udat_close_72(dtf);
    // }

    // let result = unsafe { CString::from_raw(ptr) }.into_string().unwrap();
    // println!("{:?}", result_length2);
    // println!("{}", result);
    // let provider =
    //     StaticDataProvider::try_new_from_static_blob(&ICU4X_DATA).expect("Failed to load data");
    // let langid = langid!("en");
    // let options =
    //     length::Bag::from_date_time_style(length::Date::Long, length::Time::Medium).into();
    // let dtf = DateTimeFormatter::try_new_with_buffer_provider(&provider, &langid.into(), options)
    //     .unwrap();
}
