use sdl2_sys as sys;
use libc::{c_int, uint16_t, uint32_t, c_long, c_char};


// ZERO WIDTH NO-BREAKSPACE (Unicode byte order mark)
pub const UNICODE_BOM_NATIVE: c_int = 0xFEFF;
pub const UNICODE_BOM_SWAPPED: c_int = 0xFFFE;

pub const TTF_STYLE_NORMAL: c_int = 0x00;
pub const TTF_STYLE_BOLD: c_int = 0x01;
pub const TTF_STYLE_ITALIC: c_int = 0x02;
pub const TTF_STYLE_UNDERLINE: c_int = 0x04;
pub const TTF_STYLE_STRIKETHROUGH: c_int = 0x08;

pub const TTF_HINTING_NORMAL: c_int = 0;
pub const TTF_HINTING_LIGHT: c_int = 1;
pub const TTF_HINTING_MONO: c_int = 2;
pub const TTF_HINTING_NONE: c_int = 3;

// The internal structure containing font information
pub type TTF_Font = _TTF_Font;

extern "C" {
    // This function gets the version of the dynamically linked SDL_ttf library.
    // it should NOT be used to fill a version structure, instead you should
    // use the SDL_TTF_VERSION() macro.
    //
    pub fn TTF_Linked_Version() -> *const sys::SDL_version;



    // This function tells the library whether UNICODE text is generally
    // byteswapped.  A UNICODE BOM character in a string will override
    // this setting for the remainder of that string.
    //
    pub fn TTF_ByteSwappedUNICODE(swapped: c_int);

    // Initialize the TTF engine - returns 0 if successful, -1 on error
    pub fn TTF_Init() -> c_int;

    // Open a font file and create a font of the specified point size.
    // Some .fon fonts will have several sizes embedded in the file, so the
    // point size becomes the index of choosing which size.  If the value
    // is too high, the last indexed size will be the default.
    pub fn TTF_OpenFont(file: *const c_char, ptsize: c_int) -> *mut TTF_Font;
    pub fn TTF_OpenFontIndex(file: *const c_char, ptsize: c_int, index: c_long) -> *mut TTF_Font;
    pub fn TTF_OpenFontRW(src: *mut sys::SDL_RWops,
                          freesrc: c_int,
                          ptsize: c_int)
                          -> *mut TTF_Font;
    pub fn TTF_OpenFontIndexRW(src: *mut sys::SDL_RWops,
                               freesrc: c_int,
                               ptsize: c_int,
                               index: c_long)
                               -> *mut TTF_Font;

    // Set and retrieve the font style

    pub fn TTF_GetFontStyle(font: *const TTF_Font) -> c_int;
    pub fn TTF_SetFontStyle(font: *mut TTF_Font, style: c_int);
    pub fn TTF_GetFontOutline(font: *const TTF_Font) -> c_int;
    pub fn TTF_SetFontOutline(font: *mut TTF_Font, outline: c_int);

    // Set and retrieve FreeType hinter settings
    pub fn TTF_GetFontHinting(font: *const TTF_Font) -> c_int;
    pub fn TTF_SetFontHinting(font: *mut TTF_Font, hinting: c_int);

    // Get the total height of the font - usually equal to point size
    pub fn TTF_FontHeight(font: *const TTF_Font) -> c_int;

    // Get the offset from the baseline to the top of the font
    // This is a positive value, relative to the baseline.
    //
    pub fn TTF_FontAscent(font: *const TTF_Font) -> c_int;

    // Get the offset from the baseline to the bottom of the font
    // This is a negative value, relative to the baseline.
    //
    pub fn TTF_FontDescent(font: *const TTF_Font) -> c_int;

    // Get the recommended spacing between lines of text for this font
    pub fn TTF_FontLineSkip(font: *const TTF_Font) -> c_int;

    // Get/Set whether or not kerning is allowed for this font
    pub fn TTF_GetFontKerning(font: *const TTF_Font) -> c_int;
    pub fn TTF_SetFontKerning(font: *mut TTF_Font, allowed: c_int);

    // Get the number of faces of the font
    pub fn TTF_FontFaces(font: *const TTF_Font) -> c_long;

    // Get the font face attributes, if any
    pub fn TTF_FontFaceIsFixedWidth(font: *const TTF_Font) -> c_int;
    pub fn TTF_FontFaceFamilyName(font: *const TTF_Font) -> *mut c_char;
    pub fn TTF_FontFaceStyleName(font: *const TTF_Font) -> *mut c_char;

    // Check wether a glyph is provided by the font or not
    pub fn TTF_GlyphIsProvided(font: *const TTF_Font, ch: uint16_t) -> c_int;

    // Get the metrics (dimensions) of a glyph
    // To understand what these metrics mean, here is a useful link:
    // http://freetype.sourceforge.net/freetype2/docs/tutorial/step2.html
    //
    pub fn TTF_GlyphMetrics(font: *mut TTF_Font,
                            ch: uint16_t,
                            minx: *mut c_int,
                            maxx: *mut c_int,
                            miny: *mut c_int,
                            maxy: *mut c_int,
                            advance: *mut c_int)
                            -> c_int;

    // Get the dimensions of a rendered string of text
    pub fn TTF_SizeText(font: *mut TTF_Font,
                        text: *const c_char,
                        w: *mut c_int,
                        h: *mut c_int)
                        -> c_int;
    pub fn TTF_SizeUTF8(font: *mut TTF_Font,
                        text: *const c_char,
                        w: *mut c_int,
                        h: *mut c_int)
                        -> c_int;
    pub fn TTF_SizeUNICODE(font: *mut TTF_Font,
                           text: *const uint16_t,
                           w: *mut c_int,
                           h: *mut c_int)
                           -> c_int;

    // Create an 8-bit palettized surface and render the given text at
    // fast quality with the given font and color.  The 0 pixel is the
    // colorkey, giving a transparent background, and the 1 pixel is set
    // to the text color.
    // This function returns the new surface, or NULL if there was an error.
    //
    pub fn TTF_RenderText_Solid(font: *mut TTF_Font,
                                text: *const c_char,
                                fg: sys::SDL_Color)
                                -> *mut sys::SDL_Surface;
    pub fn TTF_RenderUTF8_Solid(font: *mut TTF_Font,
                                text: *const c_char,
                                fg: sys::SDL_Color)
                                -> *mut sys::SDL_Surface;
    pub fn TTF_RenderUNICODE_Solid(font: *mut TTF_Font,
                                   text: *const uint16_t,
                                   fg: sys::SDL_Color)
                                   -> *mut sys::SDL_Surface;

    // Create an 8-bit palettized surface and render the given glyph at
    // fast quality with the given font and color.  The 0 pixel is the
    // colorkey, giving a transparent background, and the 1 pixel is set
    // to the text color.  The glyph is rendered without any padding or
    // centering in the X direction, and aligned normally in the Y direction.
    // This function returns the new surface, or NULL if there was an error.
    //
    pub fn TTF_RenderGlyph_Solid(font: *mut TTF_Font,
                                 ch: uint16_t,
                                 fg: sys::SDL_Color)
                                 -> *mut sys::SDL_Surface;

    // Create an 8-bit palettized surface and render the given text at
    // high quality with the given font and colors.  The 0 pixel is background,
    // while other pixels have varying degrees of the foreground color.
    // This function returns the new surface, or NULL if there was an error.
    //
    pub fn TTF_RenderText_Shaded(font: *mut TTF_Font,
                                 text: *const c_char,
                                 fg: sys::SDL_Color,
                                 bg: sys::SDL_Color)
                                 -> *mut sys::SDL_Surface;
    pub fn TTF_RenderUTF8_Shaded(font: *mut TTF_Font,
                                 text: *const c_char,
                                 fg: sys::SDL_Color,
                                 bg: sys::SDL_Color)
                                 -> *mut sys::SDL_Surface;
    pub fn TTF_RenderUNICODE_Shaded(font: *mut TTF_Font,
                                    text: *const uint16_t,
                                    fg: sys::SDL_Color,
                                    bg: sys::SDL_Color)
                                    -> *mut sys::SDL_Surface;

    // Create an 8-bit palettized surface and render the given glyph at
    // high quality with the given font and colors.  The 0 pixel is background,
    // while other pixels have varying degrees of the foreground color.
    // The glyph is rendered without any padding or centering in the X
    // direction, and aligned normally in the Y direction.
    // This function returns the new surface, or NULL if there was an error.
    //
    pub fn TTF_RenderGlyph_Shaded(font: *mut TTF_Font,
                                  ch: uint16_t,
                                  fg: sys::SDL_Color,
                                  bg: sys::SDL_Color)
                                  -> *mut sys::SDL_Surface;

    // Create a 32-bit ARGB surface and render the given text at high quality,
    // using alpha blending to dither the font with the given color.
    // This function returns the new surface, or NULL if there was an error.
    //
    pub fn TTF_RenderText_Blended(font: *mut TTF_Font,
                                  text: *const c_char,
                                  fg: sys::SDL_Color)
                                  -> *mut sys::SDL_Surface;
    pub fn TTF_RenderUTF8_Blended(font: *mut TTF_Font,
                                  text: *const c_char,
                                  fg: sys::SDL_Color)
                                  -> *mut sys::SDL_Surface;
    pub fn TTF_RenderUNICODE_Blended(font: *mut TTF_Font,
                                     text: *const uint16_t,
                                     fg: sys::SDL_Color)
                                     -> *mut sys::SDL_Surface;


    // Create a 32-bit ARGB surface and render the given text at high quality,
    // using alpha blending to dither the font with the given color.
    // Text is wrapped to multiple lines on line endings and on word boundaries
    // if it extends beyond wrapLength in pixels.
    // This function returns the new surface, or NULL if there was an error.
    //
    pub fn TTF_RenderText_Blended_Wrapped(font: *mut TTF_Font,
                                          text: *const c_char,
                                          fg: sys::SDL_Color,
                                          wrapLength: uint32_t)
                                          -> *mut sys::SDL_Surface;
    pub fn TTF_RenderUTF8_Blended_Wrapped(font: *mut TTF_Font,
                                          text: *const c_char,
                                          fg: sys::SDL_Color,
                                          wrapLength: uint32_t)
                                          -> *mut sys::SDL_Surface;
    pub fn TTF_RenderUNICODE_Blended_Wrapped(font: *mut TTF_Font,
                                             text: *const uint16_t,
                                             fg: sys::SDL_Color,
                                             wrapLength: uint32_t)
                                             -> *mut sys::SDL_Surface;

    // Create a 32-bit ARGB surface and render the given glyph at high quality,
    // using alpha blending to dither the font with the given color.
    // The glyph is rendered without any padding or centering in the X
    // direction, and aligned normally in the Y direction.
    // This function returns the new surface, or NULL if there was an error.
    //
    pub fn TTF_RenderGlyph_Blended(font: *mut TTF_Font,
                                   ch: uint16_t,
                                   fg: sys::SDL_Color)
                                   -> *mut sys::SDL_Surface;

    // Close an opened font file
    pub fn TTF_CloseFont(font: *mut TTF_Font);

    // De-initialize the TTF engine
    pub fn TTF_Quit();

    // Check if the TTF engine is initialized
    pub fn TTF_WasInit() -> c_int;

    // Get the kerning size of two glyphs
    pub fn TTF_GetFontKerningSize(font: *mut TTF_Font, prev_index: c_int, index: c_int) -> c_int;
}
