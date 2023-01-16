use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;

use tss_lib::{keygen, sign};

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
    )
    .unwrap();

    let data = CString::new(data).unwrap();
    data.into_raw()
}

#[no_mangle]
pub extern "C" fn sign(
    server_url: *const c_char,
    room: *const c_char,
    local_share: *const c_char,
    parties: *const c_char,
    data: *const c_char,
) -> *const c_char {
    let parties = parse_string(parties)
        .split(",")
        .map(|c| c.trim().parse::<u16>().unwrap())
        .collect();

    let signature = sign::run(
        parse_string(server_url),
        parse_string(room),
        parse_string(local_share),
        parties,
        parse_string(data).as_bytes(),
    )
    .unwrap();

    let signature = format!(
        r#"{{ "r":"0x{}", "s":"0x{}", "v":"{}" }}"#,
        hex::encode(signature.r.to_bytes().as_ref()),
        hex::encode(signature.s.to_bytes().as_ref()),
        signature.recid,
    );

    let signature = CString::new(signature).unwrap();
    signature.into_raw()
}
