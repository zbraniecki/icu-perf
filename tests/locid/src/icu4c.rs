use std::ffi::CString;
use std::os::raw::c_char;

#[link(name = "icuuc")]
extern "C" {
    pub fn uloc_toLanguageTag_72(
        localeID: *const c_char,
        langtag: *mut c_char,
        langtagCapacity: i32,
        strict: bool,
        status: *mut libc::c_int,
    ) -> i32;
}

pub struct LanguageIdentifier {}

impl LanguageIdentifier {
    pub fn canonicalize(input: &str) -> String {
        let input = CString::new(input).unwrap();
        let mut err = 0;
        let capacity = 30;
        let output = CString::default();
        let ptr = output.into_raw();

        unsafe {
            uloc_toLanguageTag_72(input.as_ptr(), ptr, capacity, true, &mut err);
        };
        unsafe { CString::from_raw(ptr) }.into_string().unwrap()
    }
}
