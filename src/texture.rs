use sdl2_sys as sys;
// use common::*;
use std::rc::Rc;
use render::Renderer;
use libc::c_int;

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
    // Returns the size of this texture
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
    #[inline]
    pub unsafe fn raw(&self) -> *mut sys::SDL_Texture {
        self.raw
    }
}
