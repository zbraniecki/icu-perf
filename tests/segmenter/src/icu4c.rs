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
    // Need to hold onto it if we converted it from utf8
    _input: Option<Vec<u16>>,
}

impl Segmenter {
    pub fn new(langid: &str, input: &str, word: bool) -> Self {
        let locale = CString::new(langid).unwrap();
        let input: Vec<u16> = input.encode_utf16().collect();
        let input_len = input.len();

        let mut status = 0;
        let ptr = unsafe {
            ubrk_open_72(
                if word { 1 } else { 2 }, // WORD or LINE
                locale.as_ptr(),
                input.as_ptr(),
                input_len as i32,
                &mut status,
            )
        };
        Self {
            ptr,
            _input: Some(input),
        }
    }

    pub fn new_utf16(langid: &str, input: &[u16], word: bool) -> Self {
        let locale = CString::new(langid).unwrap();
        let input_len = input.len();

        let mut status = 0;
        let ptr = unsafe {
            ubrk_open_72(
                if word { 1 } else { 2 }, // WORD or LINE
                locale.as_ptr(),
                input.as_ptr(),
                input_len as i32,
                &mut status,
            )
        };
        Self { ptr, _input: None }
    }
}

impl Iterator for Segmenter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let idx = unsafe { ubrk_next_72(self.ptr) };
        if idx == -1 {
            None
        } else {
            Some(idx)
        }
    }
}

impl Drop for Segmenter {
    fn drop(&mut self) {
        unsafe { ubrk_close_72(self.ptr) }
    }
}
