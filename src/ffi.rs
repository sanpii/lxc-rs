macro_rules! string {
    ($e:expr) => {{
        let buffer = ::std::ffi::CString::new($e).unwrap();
        let ptr = buffer.as_ptr();

        ::std::mem::forget(buffer);

        ptr
    }};
}

macro_rules! str {
    ($e:expr) => {{
        let buffer = unsafe {
            ::std::ffi::CStr::from_ptr($e)
        };

        buffer.to_str()
            .unwrap()
            .to_string()
    }};
}

macro_rules! opt_str {
    ($e:expr) => {
        match $e {
            Some(value) => string!(value),
            None => null(),
        }
    };
}
