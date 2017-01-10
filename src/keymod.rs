bitflags! {
    pub flags Keymod: u16 {
        const NONE = 0x0000,
        const LSHIFT = 0x0001,
        const RSHIFT = 0x0002,
        const SHIFT = LSHIFT.bits | RSHIFT.bits,
        const LCTRL = 0x0040,
        const RCTRL = 0x0080,
        const CTRL = LCTRL.bits | RCTRL.bits,
        const LALT = 0x0100,
        const RALT = 0x0200,
        const ALT = LALT.bits | RALT.bits,
        const LGUI = 0x0400,
        const RGUI = 0x0800,
        const GUI = LGUI.bits | RGUI.bits,
        const NUM = 0x1000,
        const CAPS = 0x2000,
        const MODE = 0x4000,
        const RESERVED = 0x8000
    }
}
