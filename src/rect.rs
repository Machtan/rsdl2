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
    #[inline]
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

    #[inline]
    pub fn from_center<C: Into<(i32, i32)>>(center: C, w: i32, h: i32) -> Rect {
        let (cx, cy) = center.into();
        let x = cx - (w / 2);
        let y = cy - (h / 2);
        Rect::new(x, y, w, h)
    }

    #[inline]
    pub fn move_to(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    #[inline]
    pub fn moved_to(&self, x: i32, y: i32) -> Rect {
        Rect::new(x, y, self.w, self.h)
    }

    #[inline]
    pub fn move_by(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }

    #[inline]
    pub fn moved_by(&self, dx: i32, dy: i32) -> Rect {
        Rect::new(self.x + dx, self.y + dy, self.w, self.h)
    }

    #[inline]
    pub fn center_on(&mut self, pos: (i32, i32)) {
        let (x, y) = pos;
        self.x = x - (self.w / 2);
        self.y = y - (self.h / 2);
    }

    #[inline]
    pub fn centered_on(&self, pos: (i32, i32)) -> Rect {
        let (x, y) = pos;
        let x = x - (self.w / 2);
        let y = y - (self.h / 2);
        Rect::new(x, y, self.w, self.h)
    }

    #[inline]
    pub fn resize(&mut self, w: i32, h: i32) {
        self.w = w;
        self.h = h;
    }

    #[inline]
    pub fn resized(&self, w: i32, h: i32) -> Rect {
        Rect::new(self.x, self.y, w, h)
    }

    #[inline]
    pub fn intersects<R: Into<Rect>>(&self, other: R) -> bool {
        let other = other.into();
        self.right() > other.left() && self.left() < other.right() && self.top() < other.bottom() &&
        self.bottom() > other.top()
    }

    #[inline]
    pub fn distance_to_rect(&self, other: Rect) -> Option<(i32, i32)> {
        if self.intersects(other) {
            return None;
        }
        let (cx, cy) = self.center();
        let (ocx, ocy) = other.center();
        let x_dist = if cx < ocx {
            // [s] [o]
            other.left() - self.right()
        } else {
            // [o] [s]
            self.left() - other.right()
        };
        let y_dist = if cy < ocy {
            other.top() - self.bottom()
        } else {
            self.top() - other.bottom()
        };
        Some((x_dist as i32, y_dist as i32))
    }

    #[inline]
    pub fn horizontal_distance(&self, x: i32) -> Option<i32> {
        if x >= self.right() {
            Some((x - self.right()) as i32)
        } else if x <= self.left() {
            Some((self.left() - x) as i32)
        } else {
            None
        }
    }

    #[inline]
    pub fn vertical_distance(&self, y: i32) -> Option<i32> {
        if y <= self.top() {
            Some((self.top() - y) as i32)
        } else if y >= self.bottom() {
            Some((y - self.bottom()) as i32)
        } else {
            None
        }
    }

    #[inline]
    pub fn overlap_size(&self, other: Rect) -> Option<(i32, i32)> {
        if !self.intersects(other) {
            return None;
        }
        let (cx, cy) = self.center();
        let (ocx, ocy) = other.center();
        let x_overlap = if cx < ocx {
            // [s] [o]
            self.right() - other.left()
        } else {
            // [o] [s]
            other.right() - self.left()
        } as i32;
        let y_overlap = if cy < ocy {
            self.bottom() - other.top()
        } else {
            other.bottom() - self.top()
        } as i32;
        Some((x_overlap, y_overlap))
    }

    #[inline]
    pub fn overlap_w(&self, other: Rect) -> Option<i32> {
        if !self.intersects(other) {
            return None;
        }
        let (cx, _) = self.center();
        let (ocx, _) = other.center();
        let x_overlap = if cx < ocx {
            // [s] [o]
            self.right() - other.left()
        } else {
            // [o] [s]
            other.right() - self.left()
        } as i32;
        Some(x_overlap)
    }

    #[inline]
    pub fn overlap_h(&self, other: Rect) -> Option<i32> {
        if !self.intersects(other) {
            return None;
        }
        let (_, cy) = self.center();
        let (_, ocy) = other.center();
        let y_overlap = if cy < ocy {
            self.bottom() - other.top()
        } else {
            other.bottom() - self.top()
        } as i32;
        Some(y_overlap)
    }

    #[inline]
    pub fn size(&self) -> (i32, i32) {
        (self.w, self.h)
    }

    #[inline]
    pub fn pos(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    #[inline]
    pub fn center(&self) -> (i32, i32) {
        (self.x + (self.w / 2), self.y + (self.h / 2))
    }

    #[inline]
    pub fn top(&self) -> i32 {
        self.y
    }

    #[inline]
    pub fn set_top(&mut self, y: i32) {
        self.y = y;
    }

    #[inline]
    pub fn bottom(&self) -> i32 {
        self.y + self.h
    }

    #[inline]
    pub fn set_bottom(&mut self, y: i32) {
        self.y = y - self.h;
    }

    #[inline]
    pub fn left(&self) -> i32 {
        self.x
    }

    #[inline]
    pub fn set_left(&mut self, x: i32) {
        self.x = x;
    }

    #[inline]
    pub fn right(&self) -> i32 {
        self.x + self.w
    }

    #[inline]
    pub fn set_right(&mut self, x: i32) {
        self.x = x - self.w;
    }

    #[inline]
    pub fn top_left(&self) -> (i32, i32) {
        self.pos()
    }

    #[inline]
    pub fn bottom_left(&self) -> (i32, i32) {
        (self.left(), self.right())
    }

    #[inline]
    pub fn top_right(&self) -> (i32, i32) {
        (self.right(), self.top())
    }

    #[inline]
    pub fn bottom_right(&self) -> (i32, i32) {
        (self.right(), self.bottom())
    }
}
