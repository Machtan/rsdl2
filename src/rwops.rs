use sdl2_sys as sys;
use common::*;
use std::marker::PhantomData;
use std::rc::Rc;

#[derive(Debug)]
struct InnerRWops<'a> {
    raw: *mut sys::SDL_RWops,
    _marker: PhantomData<&'a ()>,
}

impl<'a> Drop for InnerRWops<'a> {
    fn drop(&mut self) {
        unsafe { ((*self.raw).close)(self.raw) };
        println!("RWops dropped.");
    }
}

#[derive(Debug, Clone)]
pub struct RWops<'a> {
    inner: Rc<InnerRWops<'a>>,
    raw: *mut sys::SDL_RWops,
}

impl<'a> RWops<'a> {
    pub fn from_file(path: &str, mode: &str) -> Result<RWops<'static>> {
        let cstr = to_cstring(path, "Nul byte in file path")?;
        let mode = to_cstring(mode, "Nul byte in mode")?;
        let raw = assert_nonnull(unsafe { sys::SDL_RWFromFile(cstr.as_ptr(), mode.as_ptr()) })?;
        let inner = InnerRWops {
            raw: raw,
            _marker: PhantomData,
        };
        Ok(RWops {
            inner: Rc::new(inner),
            raw: raw,
        })
    }

    pub unsafe fn raw(&self) -> *mut sys::SDL_RWops {
        self.raw
    }
}
