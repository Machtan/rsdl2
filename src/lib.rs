extern crate sdl2_sys;
extern crate libc;

mod common;
mod init;
pub mod events;

pub use init::{init, InitBuilder, Context};
pub use common::{Error, Result, get_error};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
