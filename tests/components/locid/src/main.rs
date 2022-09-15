use std::ffi::CString;
use std::os::raw::c_char;
// use std::str::FromStr;

#[link(name = "icuuc")]
extern "C" {
    // pub fn uloc_getLanguage_72(localeID: *const c_char, language: *mut c_char, languageCapacity: i32, UErrorCode: *mut libc::c_int) -> i32;
    // pub fn uloc_getScript_72(localeID: *const c_char, script: *mut c_char, scriptCapacity: i32, UErrorCode: *mut libc::c_int) -> i32;
    // pub fn uloc_getCountry_72(localeID: *const c_char, region: *mut c_char, regionCapacity: i32, UErrorCode: *mut libc::c_int) -> i32;
    // pub fn uloc_getVariant_72(localeID: *const c_char, variant: *mut c_char, variantCapacity: i32, UErrorCode: *mut libc::c_int) -> i32;
    pub fn uloc_canonicalize_72(
        localeID: *const c_char,
        name: *mut c_char,
        nameCapacity: i32,
        UErrorCode: *mut libc::c_int,
    ) -> i32;
    pub fn uloc_forLanguageTag_72(
        langtag: *const c_char,
        localeID: *mut c_char,
        localeIDCapacity: i32,
        parsedLength: *mut i32,
        status: *mut libc::c_int,
    ) -> i32;
    pub fn uloc_toLanguageTag_72(
        localeID: *const c_char,
        langtag: *mut c_char,
        langtagCapacity: i32,
        strict: bool,
        status: *mut libc::c_int,
    ) -> i32;
}

fn main() {
    let input = "en_latn_us-u-h231";
    let input = CString::new(input).unwrap();
    let mut err = 0;
    let capacity = 30;

    let output = CString::default();
    let ptr = output.into_raw();
    let mut result_length = 0;
    let mut res_length = 0;
    unsafe {
        // res_length = uloc_forLanguageTag_72(input.as_ptr(), ptr, capacity, &mut result_length, &mut err);
        // uloc_toLanguageTag_72(ptr, ptr, capacity, false, &mut err);
        res_length = uloc_toLanguageTag_72(input.as_ptr(), ptr, capacity, true, &mut err);
        // uloc_canonicalize_72(input.as_ptr(), ptr, capacity, &mut err);
    };
    let result = unsafe { CString::from_raw(ptr) }.into_string().unwrap();
    println!("{}", result);
    println!("res_length: {res_length}");
}
