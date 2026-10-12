//! # input
//! This module holds logic for handling user input and entering/exiting raw mode.

unsafe extern "C" {
    fn c_enter_raw_mode();
    fn c_exit_raw_mode();
}

pub fn raw_mode() {
    unsafe { c_enter_raw_mode(); }
}

pub fn exit_raw_mode() {
    unsafe { c_exit_raw_mode(); }
}
