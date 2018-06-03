use std::os::raw::c_char;

pub fn to_cstr(s: &str) -> *const c_char {
    let buffer = ::std::ffi::CString::new(s).unwrap();
    let ptr = buffer.as_ptr();

    ::std::mem::forget(buffer);

    ptr
}

#[cfg(feature = "v1_1")]
pub fn to_mut_cstr(s: &str) -> *mut c_char {
    let mut bytes = s.to_string().into_bytes();
    bytes.push(0);

    let mut c_chars: Vec<c_char> = bytes.iter().map(|b| *b as c_char).collect();

    c_chars.as_mut_ptr()
}

pub fn to_nullable_cstr(s: Option<&str>) -> *const c_char {
    if s.is_none() {
        return ::std::ptr::null();
    }

    let buffer = ::std::ffi::CString::new(s.unwrap()).unwrap();
    let ptr = buffer.as_ptr();

    ::std::mem::forget(buffer);

    ptr
}

pub fn to_string(s: *const c_char) -> String {
    let buffer = unsafe { ::std::ffi::CStr::from_ptr(s) };

    buffer.to_str().unwrap().to_string()
}
