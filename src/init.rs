use sdl2_sys as sys;
use common::{assert_zero, Result};
use events::{EventContext, EventContextPrivate};
use video::{VideoContext, VideoContextPrivate};
use std::rc::Rc;

#[derive(Debug)]
pub struct InitBuilder {
    flags: u32,
}

impl InitBuilder {
    fn new() -> InitBuilder {
        InitBuilder { flags: 0 }
    }

    pub fn everything(mut self) -> Self {
        self.flags |= sys::SDL_INIT_EVERYTHING;
        self
    }

    pub fn video(mut self) -> Self {
        self.flags |= sys::SDL_INIT_VIDEO;
        self
    }

    pub fn events(mut self) -> Self {
        self.flags |= sys::SDL_INIT_EVENTS;
        self
    }

    pub fn audio(mut self) -> Self {
        self.flags |= sys::SDL_INIT_AUDIO;
        self
    }

    pub fn joystick(mut self) -> Self {
        self.flags |= sys::SDL_INIT_JOYSTICK;
        self
    }

    pub fn game_controller(mut self) -> Self {
        self.flags |= sys::SDL_INIT_GAMECONTROLLER;
        self
    }

    pub fn haptic(mut self) -> Self {
        self.flags |= sys::SDL_INIT_HAPTIC;
        self
    }

    pub fn timer(mut self) -> Self {
        self.flags |= sys::SDL_INIT_TIMER;
        self
    }

    pub fn finish(self) -> Result<Context> {
        assert_zero(unsafe { sys::SDL_Init(self.flags) })?;
        Ok(Context::new(self.flags))
    }
}

/// Starts initializing SDL2.
pub fn init() -> InitBuilder {
    InitBuilder::new()
}

/// Used to ensure destruction of the world.
#[derive(Debug, Clone)]
pub struct InitGuard {
    internal: Rc<InternalGuard>,
}

impl InitGuard {
    fn new(internal: InternalGuard) -> InitGuard {
        InitGuard { internal: Rc::new(internal) }
    }
}

#[derive(Debug)]
struct InternalGuard {
    _priv: (),
}

impl InternalGuard {
    pub unsafe fn new() -> InternalGuard {
        InternalGuard { _priv: () }
    }
}

impl Drop for InternalGuard {
    fn drop(&mut self) {
        unsafe {
            sys::SDL_Quit();
        }
        //println!("InternalGuard dropped: => SDL Quit");
    }
}

#[derive(Debug)]
pub struct Context {
    flags: u32,
    guard: InitGuard,
}

impl Context {
    fn new(flags: u32) -> Context {
        Context {
            flags: flags,
            guard: InitGuard::new(unsafe { InternalGuard::new() }),
        }
    }

    /// Returns an event context if SDL was initialized with event support.
    pub fn events(&self) -> Option<EventContext> {
        if self.has_events() {
            Some(EventContext::new(self.guard()))
        } else {
            None
        }
    }

    pub fn video(&self) -> Option<VideoContext> {
        if self.has_video() {
            Some(VideoContext::new(self.guard()))
        } else {
            None
        }
    }

    #[inline]
    pub fn has_video(&self) -> bool {
        (self.flags & sys::SDL_INIT_VIDEO) != 0
    }

    #[inline]
    pub fn has_events(&self) -> bool {
        (self.flags & sys::SDL_INIT_EVENTS) != 0
    }

    #[inline]
    pub fn has_audio(&self) -> bool {
        (self.flags & sys::SDL_INIT_AUDIO) != 0
    }

    #[inline]
    pub fn has_joystick(&self) -> bool {
        (self.flags & sys::SDL_INIT_JOYSTICK) != 0
    }

    #[inline]
    pub fn has_game_controller(&self) -> bool {
        (self.flags & sys::SDL_INIT_GAMECONTROLLER) != 0
    }

    #[inline]
    pub fn has_haptic(&self) -> bool {
        (self.flags & sys::SDL_INIT_HAPTIC) != 0
    }

    #[inline]
    pub fn has_timer(&self) -> bool {
        (self.flags & sys::SDL_INIT_TIMER) != 0
    }

    pub fn guard(&self) -> InitGuard {
        self.guard.clone()
    }
}
