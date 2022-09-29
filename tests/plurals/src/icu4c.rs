use crate::Category;
use std::ffi::CString;
use std::os::raw::c_char;

#[link(name = "icui18n")]
extern "C" {
    pub fn uplrules_open_72(locale: *const c_char, status: *mut libc::c_int) -> *mut libc::c_void;
    pub fn uplrules_select_72(
        pr: *mut libc::c_void,
        number: libc::c_double,
        result: *mut u16,
        capacity: libc::c_int,
        status: *mut libc::c_int,
    ) -> libc::c_int;
    pub fn uplrules_close_72(pr: *mut libc::c_void);
}

pub struct PluralRules {
    ptr: *mut libc::c_void,
}

impl PluralRules {
    pub fn new(langid: &str) -> Self {
        let locale = CString::new(langid).unwrap();

        let mut status = 0;
        let ptr = unsafe { uplrules_open_72(locale.as_ptr(), &mut status) };
        Self { ptr }
    }

    pub fn select(&self, input: f64) -> Category {
        let mut status = 0;
        let mut storage = vec![0u16; 10];

        let _new_capacity =
            unsafe { uplrules_select_72(self.ptr, input, storage.as_mut_ptr(), 255, &mut status) };
        match (storage[0], storage[1]) {
            (122, _) => Category::Zero,    // z
            (111, 110) => Category::One,   // on
            (116, _) => Category::Two,     // t
            (102, _) => Category::Few,     // f
            (109, _) => Category::Many,    // m
            (111, 116) => Category::Other, // ot
            _ => unreachable!(),
        }
    }
}

impl Drop for PluralRules {
    fn drop(&mut self) {
        unsafe { uplrules_close_72(self.ptr) }
    }
}
