use sdl2_sys as sys;
use common::*;
use video::InnerWindow;
use std::rc::Rc;
use init::InitGuard;


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
