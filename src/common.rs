#![allow(unused)]
use sdl2_sys as sys;
use libc::c_int;
use std::ffi::CStr;
use std::result;

#[derive(Debug)]
pub struct Error {
    message: String,
}

impl Error {
    fn new(message: String) -> Error {
        Error { message: message }
    }
}

pub type Result<T> = result::Result<T, Error>;

pub fn get_error() -> Error {
    Error::new(unsafe { CStr::from_ptr(sys::SDL_GetError()).to_string_lossy().into_owned() })
}

pub fn assert_zero(result: c_int) -> Result<c_int> {
    if result != 0 {
        Err(get_error())
    } else {
        Ok(result)
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
