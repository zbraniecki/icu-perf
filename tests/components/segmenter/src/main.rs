use std::ffi::CString;
use std::os::raw::c_char;

#[link(name = "icui18n")]
extern "C" {
    pub fn ubrk_open_72(
        r#type: *const libc::c_int,
        text: *const libc::c_char,
        textLength: libc::c_int,
        status: *mut libc::c_int,
    ) -> *const libc::c_void;
}

fn main() {
    // let date_style = 0;
    // let time_style = 0;
    // let locale = CString::new("en-US").unwrap();
    // let tz_id = 0;
    // let tz_id_length = 0;
    // let pattern = 0;
    // let pattern_length = 0;
    // let mut status = 0;
    // let dtf = unsafe {
    //     udat_open_72(
    //         &date_style,
    //         &time_style,
    //         locale.as_ptr(),
    //         &tz_id,
    //         tz_id_length,
    //         &pattern,
    //         pattern_length,
    //         &mut status,
    //     )
    // };
    //
    // let result = CString::default();
    // let ptr = result.into_raw();
    // let mut result_length = 0;
    // let date_to_format = 100000000.0;
    //
    // let result_length2 = unsafe {
    //     udat_format_72(
    //         &dtf,
    //         date_to_format,
    //         ptr,
    //         &mut result_length,
    //         std::ptr::null(),
    //         &mut status,
    //     )
    // };
    // println!("FOO");

    // unsafe {
    //     udat_close_72(&dtf);
    // }

    // let result = unsafe { CString::from_raw(ptr) }.into_string().unwrap();
    // println!("{}", result);
}
