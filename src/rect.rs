use sdl2_sys as sys;
// use common::*;
use libc::c_int;

/// A rectangle with its origin (x, y) in its top-left corner.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Rect {
        Rect {
            x: x,
            y: y,
            w: w,
            h: h,
        }
    }

    #[inline]
    pub fn raw(&self) -> sys::SDL_Rect {
        sys::SDL_Rect {
            x: self.x as c_int,
            y: self.y as c_int,
            w: self.w as c_int,
            h: self.h as c_int,
        }
    }
}
