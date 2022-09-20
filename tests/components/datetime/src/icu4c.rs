use std::ffi::CString;
use std::os::raw::c_char;

#[link(name = "icui18n")]
extern "C" {
    pub fn udat_open_72(
        timeStyle: libc::c_int,
        dateStyle: libc::c_int,
        locale: *const c_char,
        tzID: *const libc::c_int,
        tzIDLength: libc::c_int,
        pattern: *const u16,
        patternLength: libc::c_int,
        status: *mut libc::c_int,
    ) -> *mut libc::c_void;
    pub fn udat_close_72(format: *mut libc::c_void);
    pub fn udat_format_72(
        format: *mut libc::c_void,
        dateToFormat: libc::c_double,
        result: *mut u16,
        resultLength: libc::c_int,
        position: *const i32,
        status: *mut libc::c_int,
    ) -> libc::c_int;
}

pub struct DateTimeFormatter {
    ptr: *mut libc::c_void,
}

impl DateTimeFormatter {
    pub fn new() -> Self {
        let date_style = 2;
        let time_style = 2;
        let locale = CString::new("en-US").unwrap();

        let mut status = 0;
        let ptr = unsafe {
            udat_open_72(
                date_style,
                time_style,
                locale.as_ptr(),
                std::ptr::null(),
                -1,
                std::ptr::null(),
                -1,
                &mut status,
            )
        };
        Self { ptr }
    }

    pub fn format(&self) -> String {
        let mut status = 0;
        let mut storage = vec![0u16; 255]; // May need +1? Not sure if icu null-terminates
        let date_to_format = 1670000000000.0;

        let new_capacity = unsafe {
            udat_format_72(
                self.ptr,
                date_to_format,
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

impl Drop for DateTimeFormatter {
    fn drop(&mut self) {
        unsafe { udat_close_72(self.ptr) }
    }
}
