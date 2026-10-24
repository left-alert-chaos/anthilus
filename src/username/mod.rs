use crate::AnthilusError;

use std::ffi::CString;

unsafe extern "C" {
    fn c_user() -> CString;
}

pub fn username() -> Result<String, AnthilusError> {
    let c_username = unsafe { c_user() };
    if let Ok(string) = c_username.to_str() {
        Ok(string.to_string())
    } else {
        Err(AnthilusError::NoUsername)
    }
}
