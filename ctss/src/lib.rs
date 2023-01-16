
use std::os::raw::c_char;
use std::ffi::CStr;
use std::ffi::CString;

use tss_lib::keygen;

pub fn parse_string(s: *const c_char) -> &'static str {
    let s = unsafe {
        assert!(!s.is_null());
        CStr::from_ptr(s)
    };
    s.to_str().unwrap()
}

#[no_mangle]
pub extern "C" fn keygen(
    server_url: *const c_char,
    room: *const c_char,
    index: u16,
    threshold: u16,
    number_of_parties: u16,
) -> *const c_char {
    let data = keygen::run(
        parse_string(server_url),
        parse_string(room),
        index,
        threshold,
        number_of_parties,
    ).unwrap();

    let data = CString::new(data).unwrap();
    data.into_raw()
}
