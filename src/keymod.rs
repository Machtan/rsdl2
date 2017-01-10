bitflags! {
    pub flags Keymod: u16 {
        const NONE = 0x0000,
        const LSHIFT = 0x0001,
        const RSHIFT = 0x0002,
        const LCTRL = 0x0040,
        const RCTRL = 0x0080,
        const LALT = 0x0100,
        const RALT = 0x0200,
        const LGUI = 0x0400,
        const RGUI = 0x0800,
        const NUM = 0x1000,
        const CAPS = 0x2000,
        const MODE = 0x4000,
        const RESERVED = 0x8000
    }
}

impl Keymod {
    #[inline]
    pub fn shift(self) -> Keymod {
        self | LSHIFT | RSHIFT
    }

    #[inline]
    pub fn ctrl(self) -> Keymod {
        self | LCTRL | RCTRL
    }

    #[inline]
    pub fn alt(self) -> Keymod {
        self | LALT | LCTRL
    }

    #[inline]
    pub fn gui(self) -> Keymod {
        self | LGUI | RGUI
    }

    #[cfg(target_os = "macos")]
    #[inline]
    pub fn shortcut(self) -> Keymod {
        self.gui()
    }

    #[cfg(not(target_os = "macos"))]
    #[inline]
    pub fn shortcut(self) -> Keymod {
        self.ctrl()
    }
}
