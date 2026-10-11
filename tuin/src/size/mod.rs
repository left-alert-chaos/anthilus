//! # This is a wrapper over some C logic to get the terminal's width.

use std::os::raw::c_int;

unsafe extern "C" {
    pub fn terminal_width() -> c_int;
}
