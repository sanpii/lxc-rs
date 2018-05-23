use std::os::raw::c_char;

pub fn to_cstr(s: &str) -> *const c_char {
    let buffer = ::std::ffi::CString::new(s)
        .unwrap();
    let ptr = buffer.as_ptr();

    ::std::mem::forget(buffer);

    ptr
}

pub fn to_string(s: *const c_char) -> String {
    let buffer = unsafe {
        ::std::ffi::CStr::from_ptr(s)
    };

    buffer.to_str()
        .unwrap()
        .to_string()
}
