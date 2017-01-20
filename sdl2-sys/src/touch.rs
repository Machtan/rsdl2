use libc::{c_int, int64_t, uint32_t};
use std::u32;

pub type SDL_TouchID = int64_t;
pub type SDL_FingerID = int64_t;
pub type SDL_Finger = Finger;
pub type TouchDevice = SDL_TouchID;

pub const SDL_TOUCH_MOUSEID: uint32_t = u32::MAX;

#[derive(PartialEq, Copy, Clone)]
#[repr(C)]
pub struct Finger {
    id: TouchDevice,
    x: f32,
    y: f32,
    pressure: f32,
}

extern "C" {
    pub fn SDL_GetNumTouchDevices() -> c_int;
    pub fn SDL_GetTouchDevice(index: c_int) -> SDL_TouchID;
    pub fn SDL_GetNumTouchFingers(touchID: SDL_TouchID) -> c_int;
    pub fn SDL_GetTouchFinger(touchID: SDL_TouchID, index: c_int) -> *mut SDL_Finger;
}
