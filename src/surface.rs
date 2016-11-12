use sdl2_sys as sys;
use common::*;
use std::rc::Rc;
use rwops::RWops;
use std::ffi::CStr;

#[derive(Debug)]
pub struct InnerSurface {
    raw: *mut sys::SDL_Surface,
    pixel_buffer: Option<Vec<u8>>,
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
    pub unsafe fn from_raw(raw: *mut sys::SDL_Surface, pixel_buffer: Option<Vec<u8>>) -> Surface {
        Surface {
            inner: Rc::new(InnerSurface {
                raw: raw,
                pixel_buffer: pixel_buffer,
            }),
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
        Ok(unsafe { Surface::from_raw(raw, None) })
    }
    
    pub fn pixel_format_name(&self) -> &str {
        let name = unsafe { sys::SDL_GetPixelFormatName((*(*self.raw).format).format)};
        unsafe {CStr::from_ptr(name).to_str().unwrap()}
    }
}
