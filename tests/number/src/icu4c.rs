use std::ffi::CString;
use std::os::raw::c_char;

#[link(name = "icui18n")]
extern "C" {
    pub fn unum_open_76(
        style: libc::c_int,
        pattern: *const u16,
        patternLength: libc::c_int,
        locale: *const c_char,
        parseErr: *const libc::c_void,
        status: *mut libc::c_int,
    ) -> *mut libc::c_void;
    pub fn unum_formatInt64_76(
        formatter: *mut libc::c_void,
        number: i64,
        result: *mut u16,
        resultLength: libc::c_int,
        pos: *const i32,
        status: *mut libc::c_int,
    ) -> libc::c_int;
    pub fn unum_formatDouble_76(
        formatter: *mut libc::c_void,
        number: libc::c_double,
        result: *mut u16,
        resultLength: libc::c_int,
        pos: *const i32,
        status: *mut libc::c_int,
    ) -> libc::c_int;
    pub fn unum_close_76(format: *mut libc::c_void);

    pub fn unumf_openResult_76(
        status: *mut libc::c_int,
    ) -> *mut libc::c_void;
    pub fn unumf_closeResult_76(ptr: *mut libc::c_void);

    pub fn usnumf_openForLocale_76(
        locale: *const c_char,
        status: *mut libc::c_int,
    ) -> *mut libc::c_void;
    pub fn usnumf_formatInt64_76(
        formatter: *mut libc::c_void,
        number: i64,
        result: *mut libc::c_void,
        status: *mut libc::c_int,
    );
    pub fn ufmtval_getString_76(
        value: *const libc::c_void,
        len: *mut libc::c_int,
        status: *mut libc::c_int,
    ) -> *const libc::c_void;
    pub fn unumf_resultAsValue_76(
        result: *const libc::c_void,
        status: *mut libc::c_int,
    ) -> *const libc::c_void;
    pub fn usnumf_close_76(ptr: *mut libc::c_void);
}

pub struct NumberFormatter {
    ptr: *mut libc::c_void,
}

impl NumberFormatter {
    pub fn new(langid: &str) -> Self {
        let locale = CString::new(langid).unwrap();

        let mut status = 0;
        let ptr = unsafe {
            unum_open_76(
                1, // DECIMAL
                std::ptr::null(),
                -1,
                locale.as_ptr(),
                std::ptr::null(),
                &mut status,
            )
        };
        Self { ptr }
    }

    pub fn format(&self, input: f64) -> String {
        let mut status = 0;
        let mut storage = vec![0u16; 255];

        let new_capacity = unsafe {
            unum_formatDouble_76(
                self.ptr,
                input,
                storage.as_mut_ptr(),
                255,
                std::ptr::null(),
                &mut status,
            )
        };
        let r = String::from_utf16(&storage[0..new_capacity as usize]).unwrap();
        r
    }
}

impl Drop for NumberFormatter {
    fn drop(&mut self) {
        unsafe { unum_close_76(self.ptr) }
    }
}

pub struct FormattedNumber {
    ptr: *mut libc::c_void,
}

impl FormattedNumber {
    pub fn new() -> Self {
        let mut status = 0;
        let ptr = unsafe {
            unumf_openResult_76(
                &mut status,
            )
        };
        Self { ptr }
    }
}

impl Drop for FormattedNumber {
    fn drop(&mut self) {
        unsafe { unumf_closeResult_76(self.ptr) }
    }
}

pub struct SimpleNumberFormatter {
    ptr: *mut libc::c_void,
}

impl SimpleNumberFormatter {
    pub fn new(langid: &str) -> Self {
        let locale = CString::new(langid).unwrap();

        let mut status = 0;
        let ptr = unsafe {
            usnumf_openForLocale_76(
                locale.as_ptr(),
                &mut status,
            )
        };
        Self { ptr }
    }

    pub fn format_i64(&self, input: i64) -> String {
        let mut status = 0;
        let result = FormattedNumber::new();

        let r = unsafe {
            usnumf_formatInt64_76(
                self.ptr,
                input,
                result.ptr,
                &mut status,
            );
            let mut len = 0i32;
            let output = ufmtval_getString_76(
                unumf_resultAsValue_76(
                    result.ptr,
                    &mut status,
                ),
                &mut len,
                &mut status
            );
            let output_slice = std::slice::from_raw_parts(
                output as *const u16,
                len as usize,
            );
            String::from_utf16(&output_slice).unwrap()
        };
        r
    }
}

impl Drop for SimpleNumberFormatter {
    fn drop(&mut self) {
        unsafe { usnumf_close_76(self.ptr) }
    }
}
