pub extern crate sdl2_sys;
extern crate libc;
#[macro_use]
extern crate bitflags;

pub mod common;
#[macro_use]
mod macros;
mod init;
mod rect;
mod rwops;
mod surface;
mod texture;
pub mod keyboard;
pub mod keymod;
pub mod events;
mod video;
mod render;

pub use init::{init, InitBuilder, Context};
pub use events::{EventContext, Keysym};
pub use common::{Error, Result, get_error};
pub use video::{VideoContext, WindowBuilder, Window};
pub use render::{Color, Renderer, BlendMode};
pub use rect::Rect;
pub use surface::Surface;
pub use texture::Texture;
pub use keyboard::{Keycode, Scancode};
pub use keymod::{Keymod};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
