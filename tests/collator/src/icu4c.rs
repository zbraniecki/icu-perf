use std::cmp::Ordering;
use std::ffi::CString;
use std::os::raw::c_char;

#[link(name = "icui18n")]
extern "C" {
    pub fn ucol_open_76(locale: *const c_char, status: *mut libc::c_int) -> *mut libc::c_void;
    pub fn ucol_strcollUTF8_76(
        coll: *mut libc::c_void,
        source: *const c_char,
        sourceLength: libc::c_int,
        target: *const c_char,
        targetLength: libc::c_int,
        status: *mut libc::c_int,
    ) -> libc::c_int;
    pub fn ucol_close_76(format: *mut libc::c_void);
}

pub struct Collator {
    ptr: *mut libc::c_void,
}

impl Collator {
    pub fn new(langid: &str) -> Self {
        let locale = CString::new(langid).unwrap();

        let mut status = 0;
        let ptr = unsafe { ucol_open_76(locale.as_ptr(), &mut status) };
        Self { ptr }
    }

    pub fn compare(&self, left: &str, right: &str) -> Ordering {
        let left_length = left.len() as i32;
        let right_length = right.len() as i32;
        let left = CString::new(left).unwrap();
        let right = CString::new(right).unwrap();
        let mut status = 0;

        let result = unsafe {
            ucol_strcollUTF8_76(
                self.ptr,
                left.as_ptr(),
                left_length,
                right.as_ptr(),
                right_length,
                &mut status,
            )
        };
        match result {
            -1 => Ordering::Less,
            0 => Ordering::Equal,
            1 => Ordering::Greater,
            _ => unreachable!(),
        }
    }
}

impl Drop for Collator {
    fn drop(&mut self) {
        unsafe { ucol_close_76(self.ptr) }
    }
}
