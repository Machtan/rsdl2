extern crate sdl2_sys;
extern crate libc;

mod common;
mod init;
mod rect;
mod rwops;
mod surface;
mod texture;
pub mod events;
mod video;
mod render;

mod image;

pub use init::{init, InitBuilder, Context};
pub use events::EventContext;
pub use common::{Error, Result, get_error};
pub use video::{VideoContext, WindowBuilder, Window};
pub use render::{Color, Renderer};
pub use rect::Rect;
pub use surface::Surface;
pub use texture::Texture;
pub use image::{init_sdl2_image, ImageInitBuilder, LoadImageExt};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
