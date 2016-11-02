use sdl2_sys as sys;
use common::*;
use std::rc::Rc;
use rwops::RWops;

#[derive(Debug)]
pub struct InnerSurface {
    raw: *mut sys::SDL_Surface,
}

impl Drop for InnerSurface {
    fn drop(&mut self) {
        unsafe { sys::SDL_FreeSurface(self.raw) };
        println!("Surface dropped.");
    }
}

#[derive(Debug, Clone)]
pub struct Surface {
    inner: Rc<InnerSurface>,
    raw: *mut sys::SDL_Surface,
}

impl Surface {
    fn new(raw: *mut sys::SDL_Surface) -> Surface {
        Surface {
            inner: Rc::new(InnerSurface { raw: raw }),
            raw: raw,
        }
    }

    #[inline]
    pub unsafe fn raw(&self) -> *mut sys::SDL_Surface {
        self.raw
    }
}

impl Surface {
    pub fn load_from_bmp(file: &str) -> Result<Surface> {
        let rwops = RWops::from_file(file, "rb")?;
        let raw = assert_nonnull(unsafe { sys::SDL_LoadBMP_RW(rwops.raw(), 0) })?;
        Ok(Surface::new(raw))
    }
}
