#![allow(unused)]
use sdl2_sys as sys;
use libc::c_int;
use std::ffi::{CStr, CString};
use std::result;

#[derive(Debug)]
pub struct Error {
    message: String,
}

impl Error {
    pub fn new(message: String) -> Error {
        Error { message: message }
    }
}

pub type Result<T> = result::Result<T, Error>;

pub fn get_error() -> Error {
    Error::new(unsafe { CStr::from_ptr(sys::SDL_GetError()).to_string_lossy().into_owned() })
}

pub fn to_cstring(text: &str, error_message: &str) -> Result<CString> {
    CString::new(text).map_err(|e| Error::new(error_message.to_owned()))
}

pub fn assert_zero(result: c_int) -> Result<()> {
    if result != 0 {
        Err(get_error())
    } else {
        Ok(())
    }
}

pub fn assert_nonzero(result: c_int) -> Result<c_int> {
    if result == 0 {
        Err(get_error())
    } else {
        Ok(result)
    }
}

pub fn assert_nonnull<T>(result: *mut T) -> Result<*mut T> {
    if result.is_null() {
        Err(get_error())
    } else {
        Ok(result)
    }
}
