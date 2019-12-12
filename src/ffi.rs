use std::os::raw::c_char;

pub fn to_cstr(s: &str) -> *mut c_char {
    std::ffi::CString::new(s).unwrap().into_raw()
}

pub fn to_nullable_cstr(s: Option<&str>) -> *mut c_char {
    if s.is_none() {
        return std::ptr::null_mut();
    }

    std::ffi::CString::new(s.unwrap()).unwrap().into_raw()
}

pub fn release(p: *mut c_char) {
    if p.is_null() {
        std::mem::forget(p);
    } else {
        unsafe {
            let _ = std::ffi::CString::from_raw(p);
        }
    }
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
