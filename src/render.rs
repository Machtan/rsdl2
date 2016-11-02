use sdl2_sys as sys;
use common::*;
use video::InnerWindow;
use std::rc::Rc;
use init::InitGuard;

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
    window: Rc<InnerWindow>,
    index: Option<i32>,
    flags: u32,
}

impl RendererBuilder {
    pub fn finish(self) -> Result<Renderer> {
        let raw = assert_nonnull(unsafe {
            sys::SDL_CreateRenderer(self.window.raw, self.index.unwrap_or(-1), self.flags)
        })?;
        let inner = InnerRenderer {
            _guard: self.window._guard.clone(),
            raw: raw,
        };
        let renderer = Renderer {
            inner: Rc::new(inner),
            raw: raw,
        };
        *self.window.renderer.borrow_mut() = Some(renderer.clone());
        Ok(renderer)
    }
}

pub trait RendererBuilderPrivate {
    fn new(window: Rc<InnerWindow>) -> RendererBuilder;
}

impl RendererBuilderPrivate for RendererBuilder {
    fn new(window: Rc<InnerWindow>) -> RendererBuilder {
        RendererBuilder {
            window: window,
            index: None,
            flags: 0,
        }
    }
}

#[derive(Debug)]
struct InnerRenderer {
    _guard: InitGuard,
    raw: *mut sys::SDL_Renderer,
}

impl Drop for InnerRenderer {
    fn drop(&mut self) {
        unsafe { sys::SDL_DestroyRenderer(self.raw) }
    }
}

#[derive(Debug, Clone)]
pub struct Renderer {
    inner: Rc<InnerRenderer>,
    raw: *mut sys::SDL_Renderer,
}

impl Renderer {
    pub fn set_draw_color<C: Into<Color>>(&self, color: C) -> Result<()> {
        let c = color.into();
        assert_zero(unsafe { sys::SDL_SetRenderDrawColor(self.raw, c.r, c.g, c.b, c.a) })?;
        Ok(())
    }

    pub fn clear(&self) -> Result<()> {
        assert_zero(unsafe { sys::SDL_RenderClear(self.raw) })?;
        Ok(())
    }

    pub fn present(&self) {
        unsafe { sys::SDL_RenderPresent(self.raw) }
    }
}
