use sdl2_sys as sys;
use common::*;
use video::Window;
use std::rc::Rc;
// use init::InitGuard;
use rect::Rect;
use texture::{Texture, TexturePrivate};
use surface::Surface;
use std::ptr;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl From<u32> for Color {
    fn from(val: u32) -> Color {
        Color {
            r: ((val & 0xFF000000) >> 24) as u8,
            g: ((val & 0x00FF0000) >> 16) as u8,
            b: ((val & 0x0000FF00) >> 8) as u8,
            a: (val & 0x000000FF) as u8,
        }
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from(val: (u8, u8, u8)) -> Color {
        let (r, g, b) = val;
        Color {
            r: r,
            g: g,
            b: b,
            a: 255,
        }
    }
}

impl From<(u8, u8, u8, u8)> for Color {
    fn from(val: (u8, u8, u8, u8)) -> Color {
        let (r, g, b, a) = val;
        Color {
            r: r,
            g: g,
            b: b,
            a: a,
        }
    }
}

pub struct RendererBuilder {
    window: Window,
    index: Option<i32>,
    flags: u32,
}

impl RendererBuilder {
    pub fn finish(self) -> Result<Renderer> {
        let raw = assert_nonnull(unsafe {
            sys::SDL_CreateRenderer(self.window.raw(), self.index.unwrap_or(-1), self.flags)
        })?;
        Ok(Renderer::new(raw, self.window))
    }
}

pub trait RendererBuilderPrivate {
    fn new(window: Window) -> RendererBuilder;
}

impl RendererBuilderPrivate for RendererBuilder {
    fn new(window: Window) -> RendererBuilder {
        RendererBuilder {
            window: window,
            index: None,
            flags: 0,
        }
    }
}

#[derive(Debug)]
struct InnerRenderer {
    // Ensure that SDL isn't quit until the renderer is dropped.
    // _guard: InitGuard, // The window has a guard
    // Ensure that the window isn't destroyed before its renderer.
    window: Window,
    raw: *mut sys::SDL_Renderer,
}

impl Drop for InnerRenderer {
    fn drop(&mut self) {
        unsafe { sys::SDL_DestroyRenderer(self.raw) };
        println!("Renderer dropped.");
    }
}

#[derive(Debug, Clone)]
pub struct Renderer {
    inner: Rc<InnerRenderer>,
    raw: *mut sys::SDL_Renderer,
}

pub trait RendererPrivate {
    fn new(raw: *mut sys::SDL_Renderer, window: Window) -> Renderer;
}
impl RendererPrivate for Renderer {
    fn new(raw: *mut sys::SDL_Renderer, window: Window) -> Renderer {
        let inner = InnerRenderer {
            // _guard: unsafe { window.guard() }.clone(),
            window: window,
            raw: raw,
        };
        Renderer {
            inner: Rc::new(inner),
            raw: raw,
        }
    }
}

impl Renderer {
    pub fn set_draw_color<C: Into<Color>>(&self, color: C) {
        let c = color.into();
        unsafe { sys::SDL_SetRenderDrawColor(self.raw, c.r, c.g, c.b, c.a) };
    }

    /// Sets the draw color and returns the renderer.
    pub fn color<C: Into<Color>>(&self, color: C) -> &Renderer {
        self.set_draw_color(color);
        self
    }

    pub fn create_texture_from_surface(&self, surface: &Surface) -> Result<Texture> {
        let raw =
            assert_nonnull(unsafe { sys::SDL_CreateTextureFromSurface(self.raw, surface.raw()) })?;
        Ok(Texture::new(raw, self.clone()))
    }

    pub fn clear(&self) -> Result<()> {
        assert_zero(unsafe { sys::SDL_RenderClear(self.raw) })
    }

    pub fn fill_rect(&self, rect: Rect) -> Result<()> {
        let raw = rect.raw();
        assert_zero(unsafe { sys::SDL_RenderFillRect(self.raw, &raw) })
    }

    pub fn draw_rect(&self, rect: Rect) -> Result<()> {
        let raw = rect.raw();
        assert_zero(unsafe { sys::SDL_RenderDrawRect(self.raw, &raw) })
    }

    pub fn copy(&self, texture: &Texture, from: Option<Rect>, to: Option<Rect>) -> Result<()> {
        assert_zero(match (from.map(|r| r.raw()), to.map(|r| r.raw())) {
            (Some(from), Some(to)) => unsafe {
                sys::SDL_RenderCopy(self.raw, texture.raw(), &from, &to)
            },
            (Some(from), None) => unsafe {
                sys::SDL_RenderCopy(self.raw, texture.raw(), &from, ptr::null())
            },
            (None, Some(to)) => unsafe {
                sys::SDL_RenderCopy(self.raw, texture.raw(), ptr::null(), &to)
            },
            (None, None) => unsafe {
                sys::SDL_RenderCopy(self.raw, texture.raw(), ptr::null(), ptr::null())
            },
        })
    }

    pub fn present(&self) {
        unsafe { sys::SDL_RenderPresent(self.raw) }
    }

    #[inline]
    pub unsafe fn raw(&self) -> *mut sys::SDL_Renderer {
        self.raw
    }
}
