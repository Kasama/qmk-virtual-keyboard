use strum_macros::{AsRefStr, EnumString, FromRepr};
/*
 * Keys and buttons
 *
 * Most of the keys/buttons are modeled after USB HUT 1.12
 * (see http://www.usb.org/developers/hidpage).
 * Abbreviations in the comments:
 * AC - Application Control
 * AL - Application Launch Button
 * SC - System Control
 */

#[allow(clippy::upper_case_acronyms)]
#[allow(non_camel_case_types)]
#[derive(FromRepr, AsRefStr, EnumString, Copy, Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[strum(ascii_case_insensitive)]
#[repr(u16)]
pub enum KeyCode {
    NONE = 0,
    ESC = 1,
    #[strum(serialize = "1")]
    #[serde(rename = "1")]
    KEY_1 = 2,
    #[strum(serialize = "2")]
    #[serde(rename = "2")]
    KEY_2 = 3,
    #[strum(serialize = "3")]
    #[serde(rename = "3")]
    KEY_3 = 4,
    #[strum(serialize = "4")]
    #[serde(rename = "4")]
    KEY_4 = 5,
    #[strum(serialize = "5")]
    #[serde(rename = "5")]
    KEY_5 = 6,
    #[strum(serialize = "6")]
    #[serde(rename = "6")]
    KEY_6 = 7,
    #[strum(serialize = "7")]
    #[serde(rename = "7")]
    KEY_7 = 8,
    #[strum(serialize = "8")]
    #[serde(rename = "8")]
    KEY_8 = 9,
    #[strum(serialize = "9")]
    #[serde(rename = "9")]
    KEY_9 = 10,
    #[strum(serialize = "0")]
    #[serde(rename = "0")]
    KEY_0 = 11,
    MINUS = 12,
    EQUAL = 13,
    BACKSPACE = 14,
    TAB = 15,
    Q = 16,
    W = 17,
    E = 18,
    R = 19,
    T = 20,
    Y = 21,
    U = 22,
    I = 23,
    O = 24,
    P = 25,
    LEFTBRACE = 26,
    RIGHTBRACE = 27,
    ENTER = 28,
    LEFTCTRL = 29,
    A = 30,
    S = 31,
    D = 32,
    F = 33,
    G = 34,
    H = 35,
    J = 36,
    K = 37,
    L = 38,
    SEMICOLON = 39,
    APOSTROPHE = 40,
    GRAVE = 41,
    LEFTSHIFT = 42,
    BACKSLASH = 43,
    Z = 44,
    X = 45,
    C = 46,
    V = 47,
    B = 48,
    N = 49,
    M = 50,
    COMMA = 51,
    DOT = 52,
    SLASH = 53,
    RIGHTSHIFT = 54,
    KPASTERISK = 55,
    LEFTALT = 56,
    SPACE = 57,
    CAPSLOCK = 58,
    F1 = 59,
    F2 = 60,
    F3 = 61,
    F4 = 62,
    F5 = 63,
    F6 = 64,
    F7 = 65,
    F8 = 66,
    F9 = 67,
    F10 = 68,
    NUMLOCK = 69,
    SCROLLLOCK = 70,
    KP7 = 71,
    KP8 = 72,
    KP9 = 73,
    KPMINUS = 74,
    KP4 = 75,
    KP5 = 76,
    KP6 = 77,
    KPPLUS = 78,
    KP1 = 79,
    KP2 = 80,
    KP3 = 81,
    KP0 = 82,
    KPDOT = 83,

    ZENKAKUHANKAKU = 85,
    #[strum(serialize = "102ND")]
    #[serde(rename = "102ND")]
    KEY_102ND = 86,
    F11 = 87,
    F12 = 88,
    RO = 89,
    KATAKANA = 90,
    HIRAGANA = 91,
    HENKAN = 92,
    KATAKANAHIRAGANA = 93,
    MUHENKAN = 94,
    KPJPCOMMA = 95,
    KPENTER = 96,
    RIGHTCTRL = 97,
    KPSLASH = 98,
    SYSRQ = 99,
    RIGHTALT = 100,
    LINEFEED = 101,
    HOME = 102,
    UP = 103,
    PAGEUP = 104,
    LEFT = 105,
    RIGHT = 106,
    END = 107,
    DOWN = 108,
    PAGEDOWN = 109,
    INSERT = 110,
    DELETE = 111,
    MACRO = 112,
    MUTE = 113,
    VOLUMEDOWN = 114,
    VOLUMEUP = 115,
    POWER = 116, /* SC System Power Down */
    KPEQUAL = 117,
    KPPLUSMINUS = 118,
    PAUSE = 119,
    SCALE = 120, /* AL Compiz Scale (Expose) */

    KPCOMMA = 121,
    HANGEUL = 122,
    // KEY_HANGUEL = KEY_HANGEUL,
    HANJA = 123,
    YEN = 124,
    LEFTMETA = 125,
    RIGHTMETA = 126,
    COMPOSE = 127,

    STOP = 128, /* AC Stop */
    AGAIN = 129,
    PROPS = 130, /* AC Properties */
    UNDO = 131,  /* AC Undo */
    FRONT = 132,
    COPY = 133,  /* AC Copy */
    OPEN = 134,  /* AC Open */
    PASTE = 135, /* AC Paste */
    FIND = 136,  /* AC Search */
    CUT = 137,   /* AC Cut */
    HELP = 138,  /* AL Integrated Help Center */
    MENU = 139,  /* Menu (show menu) */
    CALC = 140,  /* AL Calculator */
    SETUP = 141,
    SLEEP = 142,  /* SC System Sleep */
    WAKEUP = 143, /* System Wake Up */
    FILE = 144,   /* AL Local Machine Browser */
    SENDFILE = 145,
    DELETEFILE = 146,
    XFER = 147,
    PROG1 = 148,
    PROG2 = 149,
    WWW = 150, /* AL Internet Browser */
    MSDOS = 151,
    COFFEE = 152, /* AL Terminal Lock/Screensaver */
    // KEY_SCREENLOCK = KEY_COFFEE,
    ROTATE_DISPLAY = 153, /* Display orientation for e.g. tablets */
    // KEY_DIRECTION = KEY_ROTATE_DISPLAY,
    CYCLEWINDOWS = 154,
    MAIL = 155,
    BOOKMARKS = 156, /* AC Bookmarks */
    COMPUTER = 157,
    BACK = 158,    /* AC Back */
    FORWARD = 159, /* AC Forward */
    CLOSECD = 160,
    EJECTCD = 161,
    EJECTCLOSECD = 162,
    NEXTSONG = 163,
    PLAYPAUSE = 164,
    PREVIOUSSONG = 165,
    STOPCD = 166,
    RECORD = 167,
    REWIND = 168,
    PHONE = 169, /* Media Select Telephone */
    ISO = 170,
    CONFIG = 171,   /* AL Consumer Control Configuration */
    HOMEPAGE = 172, /* AC Home */
    REFRESH = 173,  /* AC Refresh */
    EXIT = 174,     /* AC Exit */
    MOVE = 175,
    EDIT = 176,
    SCROLLUP = 177,
    SCROLLDOWN = 178,
    KPLEFTPAREN = 179,
    KPRIGHTPAREN = 180,
    NEW = 181,  /* AC New */
    REDO = 182, /* AC Redo/Repeat */

    F13 = 183,
    F14 = 184,
    F15 = 185,
    F16 = 186,
    F17 = 187,
    F18 = 188,
    F19 = 189,
    F20 = 190,
    F21 = 191,
    F22 = 192,
    F23 = 193,
    F24 = 194,

    PLAYCD = 200,
    PAUSECD = 201,
    PROG3 = 202,
    PROG4 = 203,
    ALL_APPLICATIONS = 204, /* AC Desktop Show All Applications */
    // KEY_DASHBOARD = KEY_ALL_APPLICATIONS,
    SUSPEND = 205,
    CLOSE = 206, /* AC Close */
    PLAY = 207,
    FASTFORWARD = 208,
    BASSBOOST = 209,
    PRINT = 210, /* AC Print */
    HP = 211,
    CAMERA = 212,
    SOUND = 213,
    QUESTION = 214,
    EMAIL = 215,
    CHAT = 216,
    SEARCH = 217,
    CONNECT = 218,
    FINANCE = 219, /* AL Checkbook/Finance */
    SPORT = 220,
    SHOP = 221,
    ALTERASE = 222,
    CANCEL = 223, /* AC Cancel */
    BRIGHTNESSDOWN = 224,
    BRIGHTNESSUP = 225,
    MEDIA = 226,

    SWITCHVIDEOMODE = 227, /* Cycle between available video
                           outputs (Monitor/LCD/TV-out/etc) */
    KBDILLUMTOGGLE = 228,
    KBDILLUMDOWN = 229,
    KBDILLUMUP = 230,

    SEND = 231,        /* AC Send */
    REPLY = 232,       /* AC Reply */
    FORWARDMAIL = 233, /* AC Forward Msg */
    SAVE = 234,        /* AC Save */
    DOCUMENTS = 235,

    BATTERY = 236,

    BLUETOOTH = 237,
    WLAN = 238,
    UWB = 239,

    UNKNOWN = 240,

    VIDEO_NEXT = 241,       /* drive next video source */
    VIDEO_PREV = 242,       /* drive previous video source */
    BRIGHTNESS_CYCLE = 243, /* brightness up, after max is min */
    BRIGHTNESS_AUTO = 244,  /* Set Auto Brightness: manual,
                            brightness control is off,
                            rely on ambient */
    // KEY_BRIGHTNESS_ZERO = KEY_BRIGHTNESS_AUTO,
    DISPLAY_OFF = 245, /* display device to off state */

    WWAN = 246, /* Wireless WAN (LTE, UMTS, GSM, etc.) */
    // KEY_WIMAX = KEY_WWAN,
    RFKILL = 247, /* Key that controls all radios */

    MICMUTE = 248, /* Mute / unmute the microphone */
}

impl From<KeyCode> for usize {
    fn from(value: KeyCode) -> usize {
        value as usize
    }
}

impl From<KeyCode> for u16 {
    fn from(value: KeyCode) -> u16 {
        value as u16
    }
}

impl From<KeyCode> for u8 {
    fn from(value: KeyCode) -> u8 {
        value as u8
    }
}

#[cfg(test)]
mod test {
    use crate::key_event::key_code::KeyCode;

    #[test]
    fn test_from_string() {
        assert_eq!(KeyCode::KEY_1, "1".parse::<KeyCode>().unwrap());
        assert_eq!(KeyCode::KEY_102ND, "102Nd".parse::<KeyCode>().unwrap());
        assert_eq!(KeyCode::A, "a".parse::<KeyCode>().unwrap());
        assert_eq!(KeyCode::DOCUMENTS, "DoCumEntS".parse::<KeyCode>().unwrap());
        assert_eq!(KeyCode::F12, "f12".parse::<KeyCode>().unwrap());
    }

    #[test]
    fn test_as_ref() {
        assert_eq!(KeyCode::KEY_1.as_ref(), "1");
        assert_eq!(KeyCode::F12.as_ref(), "F12");
    }
}
