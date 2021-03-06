extern crate sdl2_sys as sys;

// This *should* work. (The values should be within valid range)
primitive_enum! {
    pub enum Scancode: u32: u32 {
        A = 4,
        B = 5,
        C = 6,
        D = 7,
        E = 8,
        F = 9,
        G = 10,
        H = 11,
        I = 12,
        J = 13,
        K = 14,
        L = 15,
        M = 16,
        N = 17,
        O = 18,
        P = 19,
        Q = 20,
        R = 21,
        S = 22,
        T = 23,
        U = 24,
        V = 25,
        W = 26,
        X = 27,
        Y = 28,
        Z = 29,
        One = 30,
        Two = 31,
        Three = 32,
        Four = 33,
        Five = 34,
        Six = 35,
        Seven = 36,
        Eight = 37,
        Nine = 38,
        Zero = 39,
        Return = 40,
        Escape = 41,
        Backspace = 42,
        Tab = 43,
        Space = 44,
        Minus = 45,
        Equals = 46,
        Leftbracket = 47,
        Rightbracket = 48,
        Backslash = 49,
        Nonushash = 50,
        Semicolon = 51,
        Apostrophe = 52,
        Grave = 53,
        Comma = 54,
        Period = 55,
        Slash = 56,
        Capslock = 57,
        F1 = 58,
        F2 = 59,
        F3 = 60,
        F4 = 61,
        F5 = 62,
        F6 = 63,
        F7 = 64,
        F8 = 65,
        F9 = 66,
        F10 = 67,
        F11 = 68,
        F12 = 69,
        Printscreen = 70,
        Scrolllock = 71,
        Pause = 72,
        Insert = 73,
        Home = 74,
        Pageup = 75,
        Delete = 76,
        End = 77,
        Pagedown = 78,
        Right = 79,
        Left = 80,
        Down = 81,
        Up = 82,
        Numlockclear = 83,
        KpDivide = 84,
        KpMultiply = 85,
        KpMinus = 86,
        KpPlus = 87,
        KpEnter = 88,
        Kp1 = 89,
        Kp2 = 90,
        Kp3 = 91,
        Kp4 = 92,
        Kp5 = 93,
        Kp6 = 94,
        Kp7 = 95,
        Kp8 = 96,
        Kp9 = 97,
        Kp0 = 98,
        KpPeriod = 99,
        Nonusbackslash = 100,
        Application = 101,
        Power = 102,
        KpEquals = 103,
        F13 = 104,
        F14 = 105,
        F15 = 106,
        F16 = 107,
        F17 = 108,
        F18 = 109,
        F19 = 110,
        F20 = 111,
        F21 = 112,
        F22 = 113,
        F23 = 114,
        F24 = 115,
        Execute = 116,
        Help = 117,
        Menu = 118,
        Select = 119,
        Stop = 120,
        Again = 121,
        Undo = 122,
        Cut = 123,
        Copy = 124,
        Paste = 125,
        Find = 126,
        Mute = 127,
        Volumeup = 128,
        Volumedown = 129,
        KpComma = 133,
        KpEqualsas400 = 134,
        International1 = 135,
        International2 = 136,
        International3 = 137,
        International4 = 138,
        International5 = 139,
        International6 = 140,
        International7 = 141,
        International8 = 142,
        International9 = 143,
        Lang1 = 144,
        Lang2 = 145,
        Lang3 = 146,
        Lang4 = 147,
        Lang5 = 148,
        Lang6 = 149,
        Lang7 = 150,
        Lang8 = 151,
        Lang9 = 152,
        Alterase = 153,
        Sysreq = 154,
        Cancel = 155,
        Clear = 156,
        Prior = 157,
        Return2 = 158,
        Separator = 159,
        Out = 160,
        Oper = 161,
        Clearagain = 162,
        Crsel = 163,
        Exsel = 164,
        Kp00 = 176,
        Kp000 = 177,
        Thousandsseparator = 178,
        Decimalseparator = 179,
        Currencyunit = 180,
        Currencysubunit = 181,
        KpLeftparen = 182,
        KpRightparen = 183,
        KpLeftbrace = 184,
        KpRightbrace = 185,
        KpTab = 186,
        KpBackspace = 187,
        KpA = 188,
        KpB = 189,
        KpC = 190,
        KpD = 191,
        KpE = 192,
        KpF = 193,
        KpXor = 194,
        KpPower = 195,
        KpPercent = 196,
        KpLess = 197,
        KpGreater = 198,
        KpAmpersand = 199,
        KpDblampersand = 200,
        KpVerticalbar = 201,
        KpDblverticalbar = 202,
        KpColon = 203,
        KpHash = 204,
        KpSpace = 205,
        KpAt = 206,
        KpExclam = 207,
        KpMemstore = 208,
        KpMemrecall = 209,
        KpMemclear = 210,
        KpMemadd = 211,
        KpMemsubtract = 212,
        KpMemmultiply = 213,
        KpMemdivide = 214,
        KpPlusminus = 215,
        KpClear = 216,
        KpClearentry = 217,
        KpBinary = 218,
        KpOctal = 219,
        KpDecimal = 220,
        KpHexadecimal = 221,
        Lctrl = 224,
        Lshift = 225,
        Lalt = 226,
        Lgui = 227,
        Rctrl = 228,
        Rshift = 229,
        Ralt = 230,
        Rgui = 231,
        Mode = 257,
        Audionext = 258,
        Audioprev = 259,
        Audiostop = 260,
        Audioplay = 261,
        Audiomute = 262,
        Mediaselect = 263,
        Www = 264,
        Mail = 265,
        Calculator = 266,
        Computer = 267,
        AcSearch = 268,
        AcHome = 269,
        AcBack = 270,
        AcForward = 271,
        AcStop = 272,
        AcRefresh = 273,
        AcBookmarks = 274,
        Brightnessdown = 275,
        Brightnessup = 276,
        Displayswitch = 277,
        Kbdillumtoggle = 278,
        Kbdillumdown = 279,
        Kbdillumup = 280,
        Eject = 281,
        Sleep = 282,
        App1 = 283,
        App2 = 284
    }
}

primitive_enum! {
    pub enum Keycode: i32: i32 {
        Backspace = 8,
        Tab = 9,
        Return = 13,
        Escape = 27,
        Space = 32,
        Exclaim = 33,
        Quotedbl = 34,
        Hash = 35,
        Dollar = 36,
        Percent = 37,
        Ampersand = 38,
        Quote = 39,
        Leftparen = 40,
        Rightparen = 41,
        Asterisk = 42,
        Plus = 43,
        Comma = 44,
        Minus = 45,
        Period = 46,
        Slash = 47,
        Zero = 48,
        One = 49,
        Two = 50,
        Three = 51,
        Four = 52,
        Five = 53,
        Six = 54,
        Seven = 55,
        Eight = 56,
        Nine = 57,
        Colon = 58,
        Semicolon = 59,
        Less = 60,
        Equals = 61,
        Greater = 62,
        Question = 63,
        At = 64,
        Leftbracket = 91,
        Backslash = 92,
        Rightbracket = 93,
        Caret = 94,
        Underscore = 95,
        Backquote = 96,
        A = 97,
        B = 98,
        C = 99,
        D = 100,
        E = 101,
        F = 102,
        G = 103,
        H = 104,
        I = 105,
        J = 106,
        K = 107,
        L = 108,
        M = 109,
        N = 110,
        O = 111,
        P = 112,
        Q = 113,
        R = 114,
        S = 115,
        T = 116,
        U = 117,
        V = 118,
        W = 119,
        X = 120,
        Y = 121,
        Z = 122,
        Delete = 127,
        Capslock = 1073741881,
        F1 = 1073741882,
        F2 = 1073741883,
        F3 = 1073741884,
        F4 = 1073741885,
        F5 = 1073741886,
        F6 = 1073741887,
        F7 = 1073741888,
        F8 = 1073741889,
        F9 = 1073741890,
        F10 = 1073741891,
        F11 = 1073741892,
        F12 = 1073741893,
        Printscreen = 1073741894,
        Scrolllock = 1073741895,
        Pause = 1073741896,
        Insert = 1073741897,
        Home = 1073741898,
        Pageup = 1073741899,
        End = 1073741901,
        Pagedown = 1073741902,
        Right = 1073741903,
        Left = 1073741904,
        Down = 1073741905,
        Up = 1073741906,
        Numlockclear = 1073741907,
        KpDivide = 1073741908,
        KpMultiply = 1073741909,
        KpMinus = 1073741910,
        KpPlus = 1073741911,
        KpEnter = 1073741912,
        Kp1 = 1073741913,
        Kp2 = 1073741914,
        Kp3 = 1073741915,
        Kp4 = 1073741916,
        Kp5 = 1073741917,
        Kp6 = 1073741918,
        Kp7 = 1073741919,
        Kp8 = 1073741920,
        Kp9 = 1073741921,
        Kp0 = 1073741922,
        KpPeriod = 1073741923,
        Application = 1073741925,
        Power = 1073741926,
        KpEquals = 1073741927,
        F13 = 1073741928,
        F14 = 1073741929,
        F15 = 1073741930,
        F16 = 1073741931,
        F17 = 1073741932,
        F18 = 1073741933,
        F19 = 1073741934,
        F20 = 1073741935,
        F21 = 1073741936,
        F22 = 1073741937,
        F23 = 1073741938,
        F24 = 1073741939,
        Execute = 1073741940,
        Help = 1073741941,
        Menu = 1073741942,
        Select = 1073741943,
        Stop = 1073741944,
        Again = 1073741945,
        Undo = 1073741946,
        Cut = 1073741947,
        Copy = 1073741948,
        Paste = 1073741949,
        Find = 1073741950,
        Mute = 1073741951,
        Volumeup = 1073741952,
        Volumedown = 1073741953,
        KpComma = 1073741957,
        KpEqualsas400 = 1073741958,
        Alterase = 1073741977,
        Sysreq = 1073741978,
        Cancel = 1073741979,
        Clear = 1073741980,
        Prior = 1073741981,
        Return2 = 1073741982,
        Separator = 1073741983,
        Out = 1073741984,
        Oper = 1073741985,
        Clearagain = 1073741986,
        Crsel = 1073741987,
        Exsel = 1073741988,
        Kp00 = 1073742000,
        Kp000 = 1073742001,
        Thousandsseparator = 1073742002,
        Decimalseparator = 1073742003,
        Currencyunit = 1073742004,
        Currencysubunit = 1073742005,
        KpLeftparen = 1073742006,
        KpRightparen = 1073742007,
        KpLeftbrace = 1073742008,
        KpRightbrace = 1073742009,
        KpTab = 1073742010,
        KpBackspace = 1073742011,
        KpA = 1073742012,
        KpB = 1073742013,
        KpC = 1073742014,
        KpD = 1073742015,
        KpE = 1073742016,
        KpF = 1073742017,
        KpXor = 1073742018,
        KpPower = 1073742019,
        KpPercent = 1073742020,
        KpLess = 1073742021,
        KpGreater = 1073742022,
        KpAmpersand = 1073742023,
        KpDblampersand = 1073742024,
        KpVerticalbar = 1073742025,
        KpDblverticalbar = 1073742026,
        KpColon = 1073742027,
        KpHash = 1073742028,
        KpSpace = 1073742029,
        KpAt = 1073742030,
        KpExclam = 1073742031,
        KpMemstore = 1073742032,
        KpMemrecall = 1073742033,
        KpMemclear = 1073742034,
        KpMemadd = 1073742035,
        KpMemsubtract = 1073742036,
        KpMemmultiply = 1073742037,
        KpMemdivide = 1073742038,
        KpPlusminus = 1073742039,
        KpClear = 1073742040,
        KpClearentry = 1073742041,
        KpBinary = 1073742042,
        KpOctal = 1073742043,
        KpDecimal = 1073742044,
        KpHexadecimal = 1073742045,
        Lctrl = 1073742048,
        Lshift = 1073742049,
        Lalt = 1073742050,
        Lgui = 1073742051,
        Rctrl = 1073742052,
        Rshift = 1073742053,
        Ralt = 1073742054,
        Rgui = 1073742055,
        Mode = 1073742081,
        Audionext = 1073742082,
        Audioprev = 1073742083,
        Audiostop = 1073742084,
        Audioplay = 1073742085,
        Audiomute = 1073742086,
        Mediaselect = 1073742087,
        Www = 1073742088,
        Mail = 1073742089,
        Calculator = 1073742090,
        Computer = 1073742091,
        AcSearch = 1073742092,
        AcHome = 1073742093,
        AcBack = 1073742094,
        AcForward = 1073742095,
        AcStop = 1073742096,
        AcRefresh = 1073742097,
        AcBookmarks = 1073742098,
        Brightnessdown = 1073742099,
        Brightnessup = 1073742100,
        Displayswitch = 1073742101,
        Kbdillumtoggle = 1073742102,
        Kbdillumdown = 1073742103,
        Kbdillumup = 1073742104,
        Eject = 1073742105,
        Sleep = 1073742106
    }
}
