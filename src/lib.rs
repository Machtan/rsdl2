extern crate sdl2_sys;
extern crate libc;

mod common;
mod init;
mod rect;
pub mod events;
mod video;
mod render;

pub use init::{init, InitBuilder, Context};
pub use events::EventContext;
pub use common::{Error, Result, get_error};
pub use video::{VideoContext, WindowBuilder, Window};
pub use render::{Color, Renderer};
pub use rect::Rect;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
