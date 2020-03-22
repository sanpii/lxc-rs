use std::os::raw::c_char;

pub fn to_cstr(s: &str) -> std::ffi::CString {
    std::ffi::CString::new(s).unwrap()
}

pub fn to_string(s: *const c_char) -> String {
    let buffer = unsafe { std::ffi::CStr::from_ptr(s) };

    buffer.to_str().unwrap().to_string()
}

pub fn vec_from_nta(raw: *mut *mut i8) -> Vec<*mut i8> {
    let mut vec = Vec::new();

    for x in 0.. {
        unsafe {
            if !(*raw.offset(x)).is_null() {
                vec.push(*raw.offset(x));
            } else {
                break;
            }
        }
    }

    vec
}
