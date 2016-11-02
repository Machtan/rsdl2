use common::*;
use init::InitGuard;
use sdl2_sys as sys;
use std::ffi::{NulError, CString};
use std::result;
use std::rc::Rc;
use render::{RendererBuilder, RendererBuilderPrivate, Renderer, RendererPrivate};

#[derive(Debug)]
pub struct InnerWindow {
    pub raw: *mut sys::SDL_Window,
    pub _guard: InitGuard,
}

impl Drop for InnerWindow {
    fn drop(&mut self) {
        unsafe {
            sys::SDL_DestroyWindow(self.raw);
        }
        println!("Window dropped.");
    }
}

#[derive(Debug, Clone)]
pub struct Window {
    raw: *mut sys::SDL_Window,
    inner: Rc<InnerWindow>,
}

impl Window {
    fn new(raw: *mut sys::SDL_Window, guard: InitGuard) -> Window {
        let inner = InnerWindow {
            raw: raw,
            _guard: guard,
        };
        Window {
            raw: raw,
            inner: Rc::new(inner),
        }
    }

    /// Returns the renderer for this window if it has one.
    pub fn renderer(&self) -> Option<Renderer> {
        let raw = unsafe { sys::SDL_GetRenderer(self.raw) };
        if raw.is_null() {
            None
        } else {
            Some(Renderer::new(raw, self.clone()))
        }
    }

    pub fn build_renderer(&self) -> RendererBuilder {
        RendererBuilder::new(self.clone())
    }

    pub unsafe fn raw(&self) -> *mut sys::SDL_Window {
        self.raw
    }

    pub unsafe fn guard(&self) -> InitGuard {
        self.inner._guard.clone()
    }
}

#[derive(Debug)]
enum WindowPos {
    Centered,
    Undefined,
    Set(i32),
}
impl WindowPos {
    fn value(self) -> i32 {
        use self::WindowPos::*;
        match self {
            Centered => sys::SDL_WINDOWPOS_CENTERED,
            Undefined => sys::SDL_WINDOWPOS_UNDEFINED,
            Set(val) => val,
        }
    }
}

#[derive(Debug)]
#[must_use]
pub struct WindowBuilder {
    title: CString,
    x: WindowPos,
    y: WindowPos,
    w: i32,
    h: i32,
    flags: u32,
    _guard: InitGuard,
}

impl WindowBuilder {
    fn new(guard: InitGuard) -> WindowBuilder {
        WindowBuilder {
            title: CString::new("").unwrap(),
            x: WindowPos::Undefined,
            y: WindowPos::Undefined,
            w: 500,
            h: 500,
            flags: 0,
            _guard: guard,
        }
    }

    pub fn title(mut self, title: &str) -> result::Result<Self, NulError> {
        self.title = CString::new(title)?;
        Ok(self)
    }

    pub fn position(mut self, x: i32, y: i32) -> Self {
        self.x = WindowPos::Set(x);
        self.y = WindowPos::Set(y);
        self
    }

    pub fn center(mut self, x: bool, y: bool) -> Self {
        if x {
            self.x = WindowPos::Centered;
        }
        if y {
            self.y = WindowPos::Centered;
        }
        self
    }

    pub fn x(mut self, x: i32) -> Self {
        self.x = WindowPos::Set(x);
        self
    }

    pub fn y(mut self, y: i32) -> Self {
        self.y = WindowPos::Set(y);
        self
    }

    pub fn size(mut self, width: i32, height: i32) -> Self {
        self.w = width;
        self.h = height;
        self
    }

    pub fn width(mut self, width: i32) -> Self {
        self.w = width;
        self
    }

    pub fn height(mut self, height: i32) -> Self {
        self.h = height;
        self
    }

    pub fn fullscreen(mut self) -> Self {
        self.flags |= sys::SDL_WINDOW_FULLSCREEN as u32;
        self
    }

    pub fn fullscreen_at_desktop_resolution(mut self) -> Self {
        self.flags |= sys::SDL_WINDOW_FULLSCREEN_DESKTOP as u32;
        self
    }

    pub fn opengl(mut self) -> Self {
        self.flags |= sys::SDL_WINDOW_OPENGL as u32;
        self
    }

    pub fn hidden(mut self) -> Self {
        self.flags |= sys::SDL_WINDOW_HIDDEN as u32;
        self
    }

    pub fn borderless(mut self) -> Self {
        self.flags |= sys::SDL_WINDOW_BORDERLESS as u32;
        self
    }

    pub fn resizable(mut self) -> Self {
        self.flags |= sys::SDL_WINDOW_RESIZABLE as u32;
        self
    }

    pub fn minimized(mut self) -> Self {
        self.flags |= sys::SDL_WINDOW_MINIMIZED as u32;
        self
    }

    pub fn maximized(mut self) -> Self {
        self.flags |= sys::SDL_WINDOW_MAXIMIZED as u32;
        self
    }

    pub fn grab_input_focus(mut self) -> Self {
        self.flags |= sys::SDL_WINDOW_INPUT_GRABBED as u32;
        self
    }

    pub fn allow_highdpi(mut self) -> Self {
        self.flags |= sys::SDL_WINDOW_ALLOW_HIGHDPI as u32;
        self
    }

    pub fn finish(self) -> Result<Rc<Window>> {
        let raw = unsafe {
            // TODO: Remember to free the title pointer when a new title is set/the window is dropped
            sys::SDL_CreateWindow(self.title.into_raw(),
                                  self.x.value(),
                                  self.y.value(),
                                  self.w,
                                  self.h,
                                  self.flags)
        };
        Ok(Rc::new(Window::new(raw, self._guard.clone())))
    }
}

#[derive(Debug)]
pub struct VideoContext {
    _guard: InitGuard,
}

pub trait VideoContextPrivate {
    fn new(guard: InitGuard) -> VideoContext;
}

impl VideoContextPrivate for VideoContext {
    fn new(guard: InitGuard) -> VideoContext {
        VideoContext { _guard: guard }
    }
}

impl VideoContext {
    pub fn build_window(&self) -> WindowBuilder {
        WindowBuilder::new(self._guard.clone())
    }
}
