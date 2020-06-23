use libc::c_char;
use std::ffi::CStr;

#[no_mangle]
#[link_name = "validate_email"]
pub extern fn validate_email_c_string(raw_email: *const c_char) -> bool {
    let c_str = unsafe {
        assert!(!raw_email.is_null());

        CStr::from_ptr(raw_email)
    };

    let email = c_str.to_str().unwrap();
    verus_validations::validate_email(email)
}

#[no_mangle]
#[link_name = "validate_date"]
pub extern fn validate_date_c_string(raw_date: *const c_char) -> bool {
    let c_str = unsafe {
        assert!(!raw_date.is_null());

        CStr::from_ptr(raw_date)
    };

    let date = c_str.to_str().unwrap();
    verus_validations::validate_date(date)
}

#[cfg(test)]
mod tests {}
