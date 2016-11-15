use sdl2_sys as sys;
// use common::*;
use std::rc::Rc;
use render::Renderer;
use libc::c_int;
use rect::Rect;

/// The drop guard for an SDL texture.
#[derive(Debug)]
pub struct InnerTexture {
    raw: *mut sys::SDL_Texture,
    renderer: Renderer,
}

impl Drop for InnerTexture {
    fn drop(&mut self) {
        unsafe { sys::SDL_DestroyTexture(self.raw) };
        println!("Texture dropped.");
    }
}

/// A handle to an image uploaded to the GPU.
#[derive(Debug, Clone)]
pub struct Texture {
    inner: Rc<InnerTexture>,
    raw: *mut sys::SDL_Texture,
}

pub trait TexturePrivate {
    fn new(raw: *mut sys::SDL_Texture, renderer: Renderer) -> Texture;
}

impl TexturePrivate for Texture {
    fn new(raw: *mut sys::SDL_Texture, renderer: Renderer) -> Texture {
        let inner = InnerTexture {
            raw: raw,
            renderer: renderer,
        };
        Texture {
            inner: Rc::new(inner),
            raw: raw,
        }
    }
}

impl Texture {
    /// Returns the size of this texture.
    pub fn query_size(&self) -> (i32, i32) {
        let mut _fmt: u32 = 0;
        let mut _acc: c_int = 0;
        let mut w: c_int = 0;
        let mut h: c_int = 0;
        unsafe {
            sys::SDL_QueryTexture(self.raw, &mut _fmt, &mut _acc, &mut w, &mut h);
        }
        (w as i32, h as i32)
    }

    /// Returns a rect that can fit this texture, at the given position.
    pub fn rect_at(&self, x: i32, y: i32) -> Rect {
        let (w, h) = self.query_size();
        Rect::new(x, y, w, h)
    }

    /// Returns a reference to the underlying SDL_Texture.
    #[inline]
    pub unsafe fn raw(&self) -> *mut sys::SDL_Texture {
        self.raw
    }
}
