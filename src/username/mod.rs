use crate::AnthilusError;

use std::ffi::c_char;

unsafe extern "C" {
    fn c_user() -> *const c_char;
}

pub fn username() -> Result<String, AnthilusError> {
    unsafe {
        let mut ptr = c_user().as_ref().unwrap() as *const i8 as *const u8;
        let mut username = String::new();
        
        loop {
            let character = *ptr as char;
            if character == '\0' {
                break;
            }

            username.push(*ptr as char);
            ptr = ptr.add(1);
        }

        Ok(username)
    }
}
