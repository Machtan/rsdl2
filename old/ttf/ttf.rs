use ttf::ffi;
use common::*;

pub struct TtfContext {}

pub fn init() -> Result<TtfContext> {
    assert_zero(unsafe { ffi::TTF_Init() })
}
