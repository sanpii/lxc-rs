use std::os::raw::c_char;

pub fn to_cstr(s: &str) -> crate::Result<std::ffi::CString> {
    std::ffi::CString::new(s).map_err(Into::into)
}

macro_rules! cstr {
    ( $s:expr ) => {
        $crate::ffi::to_cstr($s)?.as_ptr()
    };
}

#[cfg(feature = "v1_1")]
pub fn to_mut_cstr(s: &str) -> Vec<c_char> {
    let mut bytes = s.to_string().into_bytes();
    bytes.push(0);

    bytes.iter().map(|b| *b as c_char).collect()
}

pub fn to_string(s: *const c_char) -> crate::Result<String> {
    let buffer = unsafe { std::ffi::CStr::from_ptr(s) };

    Ok(buffer.to_str()?.to_string())
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
