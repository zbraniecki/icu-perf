use std::ffi::CString;
use std::os::raw::c_char;

#[link(name = "icui18n")]
extern "C" {
    pub fn udat_open_72(
        timeStyle: libc::c_int,
        dateStyle: libc::c_int,
        locale: *const c_char,
        tzID: *const u16,
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

fn get_length(input: &str) -> libc::c_int {
    match input {
        "None" => -1,
        "Short" => 3,
        "Medium" => 2,
        "Long" => 1,
        "Full" => 0,
        _ => unreachable!(),
    }
}

pub struct DateTimeFormatter {
    ptr: *mut libc::c_void,
}

impl DateTimeFormatter {
    pub fn new(langid: &str, date_style: &str, time_style: &str) -> Self {
        let date_style = get_length(date_style);
        let time_style = get_length(time_style);
        let locale = CString::new(langid).unwrap();
        let tz: Vec<u16> = "GMT".encode_utf16().collect();
        let tz_length = tz.len();

        let mut status = 0;
        let ptr = unsafe {
            udat_open_72(
                time_style,
                date_style,
                locale.as_ptr(),
                tz.as_ptr(),
                tz_length as i32,
                std::ptr::null(),
                -1,
                &mut status,
            )
        };
        Self { ptr }
    }

    pub fn format(&self, input: i32) -> String {
        let mut status = 0;
        let mut storage = vec![0u16; 255]; // May need +1? Not sure if icu null-terminates
        let date_to_format = input as f64 * 60.0 * 1000.0; // ms global unix epoch

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
