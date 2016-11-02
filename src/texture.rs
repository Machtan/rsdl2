use sdl2_sys as sys;
// use common::*;
use std::rc::Rc;
use render::Renderer;

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
    #[inline]
    pub unsafe fn raw(&self) -> *mut sys::SDL_Texture {
        self.raw
    }
}
