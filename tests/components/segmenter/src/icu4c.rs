use std::ffi::CString;
use std::os::raw::c_char;

#[link(name = "icuuc")]
extern "C" {
    // U_CAPI UBreakIterator* U_EXPORT2
    // ubrk_open(UBreakIteratorType type,
    //       const char *locale,
    //       const UChar *text,
    //       int32_t textLength,
    //       UErrorCode *status)
    pub fn ubrk_open_72(
        r#type: libc::c_int,
        locale: *const c_char,
        text: *const u16,
        textLength: libc::c_int,
        status: *mut libc::c_int,
    ) -> *mut libc::c_void;

    // U_CAPI int32_t U_EXPORT2
    // ubrk_next(UBreakIterator *bi)
    pub fn ubrk_next_72(bi: *mut libc::c_void) -> libc::c_int;

    // U_CAPI void U_EXPORT2
    // ubrk_close(UBreakIterator *bi)
    pub fn ubrk_close_72(bi: *mut libc::c_void);
}

pub struct Segmenter {
    ptr: *mut libc::c_void,
}

impl Segmenter {
    pub fn new(langid: &str, input: &str) -> Self {
        let locale = CString::new(langid).unwrap();
        let input: Vec<u16> = input.encode_utf16().collect();
        let inputLen = input.len();

        let mut status = 0;
        let ptr = unsafe {
            ubrk_open_72(
                1, // WORD
                locale.as_ptr(),
                input.as_ptr(),
                inputLen as i32,
                &mut status,
            )
        };
        Self { ptr }
    }
}

impl Iterator for Segmenter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let idx = unsafe { ubrk_next_72(self.ptr) };
        if idx > 0 {
            Some(idx)
        } else {
            None
        }
    }
}

impl Drop for Segmenter {
    fn drop(&mut self) {
        unsafe { ubrk_close_72(self.ptr) }
    }
}
