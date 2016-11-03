// use sdl2_sys as sys;
use libc::c_int;
use common::*;
use texture::{Texture, TexturePrivate};
use render::Renderer;
use image::ffi;

pub fn preload_image_support() -> ImageInitBuilder {
    ImageInitBuilder { flags: 0 }
}

pub struct ImageInitBuilder {
    flags: c_int,
}

impl ImageInitBuilder {
    pub fn everything(self) -> Self {
        self.jpg().png().tif().webp()
    }

    pub fn jpg(mut self) -> Self {
        self.flags |= ffi::IMG_INIT_JPG;
        self
    }

    pub fn png(mut self) -> Self {
        self.flags |= ffi::IMG_INIT_PNG;
        self
    }

    pub fn tif(mut self) -> Self {
        self.flags |= ffi::IMG_INIT_TIF;
        self
    }

    pub fn webp(mut self) -> Self {
        self.flags |= ffi::IMG_INIT_WEBP;
        self
    }

    pub fn finish(self) -> Result<()> {
        if self.flags == 0 {
            let res = unsafe { ffi::IMG_Init(self.flags) };
            println!("Initialized image support flags: {}", res);
            return Ok(()); // If init is run with 0, it returns the current initialized flags
        }
        let res = unsafe { ffi::IMG_Init(self.flags) };
        if res != self.flags {
            let mut message = "Could not support requested image format(s):".to_owned();
            let uninitialized = res ^ self.flags;
            if (uninitialized & ffi::IMG_INIT_JPG) != 0 {
                message.push_str(" jpg,");
            }
            if (uninitialized & ffi::IMG_INIT_PNG) != 0 {
                message.push_str(" png,");
            }
            if (uninitialized & ffi::IMG_INIT_TIF) != 0 {
                message.push_str(" tif,");
            }
            if (uninitialized & ffi::IMG_INIT_WEBP) != 0 {
                message.push_str(" webp,");
            }
            Err(Error::new(message))
        } else {
            Ok(())
        }
    }
}

pub trait LoadImageExt {
    fn load_image(&self, file: &str) -> Result<Texture>;
}

impl LoadImageExt for Renderer {
    fn load_image(&self, file: &str) -> Result<Texture> {
        let cstr = to_cstring(file, "Nul byte in file path")?;
        let raw = assert_nonnull(unsafe { ffi::IMG_LoadTexture(self.raw(), cstr.as_ptr()) })?;
        Ok(Texture::new(raw, self.clone()))
    }
}

pub unsafe fn quit() {
    ffi::IMG_Quit();
}
