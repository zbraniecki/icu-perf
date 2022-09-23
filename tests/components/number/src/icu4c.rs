use std::ffi::CString;
use std::os::raw::c_char;

#[link(name = "icui18n")]
extern "C" {
    pub fn unum_open_72(
        style: libc::c_int,
        pattern: *const u16,
        patternLength: libc::c_int,
        locale: *const c_char,
        parseErr: *const libc::c_void,
        status: *mut libc::c_int,
    ) -> *mut libc::c_void;
    pub fn unum_formatInt64_72(
        formatter: *mut libc::c_void,
        number: i64,
        result: *mut u16,
        resultLength: libc::c_int,
        pos: *const i32,
        status: *mut libc::c_int,
    ) -> libc::c_int;
    pub fn unum_close_72(format: *mut libc::c_void);
}

pub struct NumberFormatter {
    ptr: *mut libc::c_void,
}

impl NumberFormatter {
    pub fn new(langid: &str) -> Self {
        let locale = CString::new(langid).unwrap();

        let mut status = 0;
        let ptr = unsafe {
            unum_open_72(
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

    pub fn format(&self, input: i64) -> String {
        let mut status = 0;
        let mut storage = vec![0u16; 255];

        let new_capacity = unsafe {
            unum_formatInt64_72(
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
        unsafe { unum_close_72(self.ptr) }
    }
}
