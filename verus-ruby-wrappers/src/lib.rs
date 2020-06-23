use libc::c_char;
use std::ffi::CStr;

#[no_mangle]
pub extern fn email(raw_email: *const c_char) -> bool {
    let c_str = unsafe {
        assert!(!raw_email.is_null());

        CStr::from_ptr(raw_email)
    };

    let email = c_str.to_str().unwrap();
    verus_validations::validate_email(email)
}

#[no_mangle]
pub extern fn date(raw_date: *const c_char) -> bool {
    let c_str = unsafe {
        assert!(!raw_date.is_null());

        CStr::from_ptr(raw_date)
    };

    let date = c_str.to_str().unwrap();
    verus_validations::validate_date(date)
}

#[cfg(test)]
mod tests {}
