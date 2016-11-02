#![allow(non_camel_case_types)]
#![allow(unused)]
use sdl2_sys as sys;
use libc::{c_int, c_char};

// const SDL_version * IMG_Linked_Version(void);

type IMG_InitFlags = c_int;

pub const IMG_INIT_JPG: IMG_InitFlags = 0x00000001;
pub const IMG_INIT_PNG: IMG_InitFlags = 0x00000002;
pub const IMG_INIT_TIF: IMG_InitFlags = 0x00000004;
pub const IMG_INIT_WEBP: IMG_InitFlags = 0x00000008;

// Loads dynamic libraries and prepares them for use.  Flags should be
// one or more flags from IMG_InitFlags OR'd together.
// It returns the flags successfully initialized, or 0 on failure.
//
#[link(name = "SDL2_image")]
extern "C" {
    pub fn IMG_Init(flags: c_int) -> c_int;

    // Unloads libraries loaded with IMG_Init
    pub fn IMG_Quit();

    // Load an image from an SDL data source.
    // The 'type' may be one of: "BMP", "GIF", "PNG", etc.
    //
    // If the image format supports a transparent pixel, SDL will set the
    // colorkey for the surface.  You can enable RLE acceleration on the
    // surface afterwards by calling:
    // SDL_SetColorKey(image, SDL_RLEACCEL, image->format->colorkey);
    //
    pub fn IMG_LoadTyped_RW(src: *mut sys::SDL_RWops,
                            freesrc: c_int,
                            type_: *const c_char)
                            -> *mut sys::SDL_Surface;
    // Convenience functions
    pub fn IMG_Load(file: *const c_char) -> *mut sys::SDL_Surface;
    pub fn IMG_Load_RW(src: *mut sys::SDL_RWops, freesrc: c_int) -> *mut sys::SDL_Surface;

    // Load an image directly into a render texture.
    //
    pub fn IMG_LoadTexture(renderer: *mut sys::SDL_Renderer,
                           file: *const c_char)
                           -> *mut sys::SDL_Texture;
    pub fn IMG_LoadTexture_RW(renderer: *mut sys::SDL_Renderer,
                              src: *mut sys::SDL_RWops,
                              freesrc: c_int)
                              -> *mut sys::SDL_Texture;
    pub fn IMG_LoadTextureTyped_RW(renderer: *mut sys::SDL_Renderer,
                                   src: *mut sys::SDL_RWops,
                                   freesrc: c_int,
                                   type_: *const c_char)
                                   -> *mut sys::SDL_Texture;

    // Functions to detect a file type, given a seekable source
    pub fn IMG_isICO(src: *mut sys::SDL_RWops) -> c_int;
    pub fn IMG_isCUR(src: *mut sys::SDL_RWops) -> c_int;
    pub fn IMG_isBMP(src: *mut sys::SDL_RWops) -> c_int;
    pub fn IMG_isGIF(src: *mut sys::SDL_RWops) -> c_int;
    pub fn IMG_isJPG(src: *mut sys::SDL_RWops) -> c_int;
    pub fn IMG_isLBM(src: *mut sys::SDL_RWops) -> c_int;
    pub fn IMG_isPCX(src: *mut sys::SDL_RWops) -> c_int;
    pub fn IMG_isPNG(src: *mut sys::SDL_RWops) -> c_int;
    pub fn IMG_isPNM(src: *mut sys::SDL_RWops) -> c_int;
    pub fn IMG_isTIF(src: *mut sys::SDL_RWops) -> c_int;
    pub fn IMG_isXCF(src: *mut sys::SDL_RWops) -> c_int;
    pub fn IMG_isXPM(src: *mut sys::SDL_RWops) -> c_int;
    pub fn IMG_isXV(src: *mut sys::SDL_RWops) -> c_int;
    pub fn IMG_isWEBP(src: *mut sys::SDL_RWops) -> c_int;

    // Individual loading functions
    pub fn IMG_LoadICO_RW(src: *mut sys::SDL_RWops) -> *mut sys::SDL_Surface;
    pub fn IMG_LoadCUR_RW(src: *mut sys::SDL_RWops) -> *mut sys::SDL_Surface;
    pub fn IMG_LoadBMP_RW(src: *mut sys::SDL_RWops) -> *mut sys::SDL_Surface;
    pub fn IMG_LoadGIF_RW(src: *mut sys::SDL_RWops) -> *mut sys::SDL_Surface;
    pub fn IMG_LoadJPG_RW(src: *mut sys::SDL_RWops) -> *mut sys::SDL_Surface;
    pub fn IMG_LoadLBM_RW(src: *mut sys::SDL_RWops) -> *mut sys::SDL_Surface;
    pub fn IMG_LoadPCX_RW(src: *mut sys::SDL_RWops) -> *mut sys::SDL_Surface;
    pub fn IMG_LoadPNG_RW(src: *mut sys::SDL_RWops) -> *mut sys::SDL_Surface;
    pub fn IMG_LoadPNM_RW(src: *mut sys::SDL_RWops) -> *mut sys::SDL_Surface;
    pub fn IMG_LoadTGA_RW(src: *mut sys::SDL_RWops) -> *mut sys::SDL_Surface;
    pub fn IMG_LoadTIF_RW(src: *mut sys::SDL_RWops) -> *mut sys::SDL_Surface;
    pub fn IMG_LoadXCF_RW(src: *mut sys::SDL_RWops) -> *mut sys::SDL_Surface;
    pub fn IMG_LoadXPM_RW(src: *mut sys::SDL_RWops) -> *mut sys::SDL_Surface;
    pub fn IMG_LoadXV_RW(src: *mut sys::SDL_RWops) -> *mut sys::SDL_Surface;
    pub fn IMG_LoadWEBP_RW(src: *mut sys::SDL_RWops) -> *mut sys::SDL_Surface;

    pub fn IMG_ReadXPMFromArray(xpm: *mut *mut c_char) -> *mut sys::SDL_Surface;

    // Individual saving functions
    pub fn IMG_SavePNG(surface: *mut sys::SDL_Surface, file: *const c_char) -> c_int;
    pub fn IMG_SavePNG_RW(surface: *mut sys::SDL_Surface,
                          dst: *mut sys::SDL_RWops,
                          freedst: c_int)
                          -> c_int;
}
