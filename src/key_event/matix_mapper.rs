use serde::{Serialize, Deserialize};
use serde_tuple::{Serialize_tuple, Deserialize_tuple};

use crate::key_event::key_code::KeyCode;

#[derive(Debug, Clone, Copy, Serialize_tuple, Deserialize_tuple)]
pub struct MatrixPosition {
    pub row: u8,
    pub col: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LayoutItem {
    pub matrix: MatrixPosition,
    pub x: u8,
    pub y: u8,
    pub label: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Layout {
    pub layout: Vec<LayoutItem>,
}

impl From<(u8, u8)> for MatrixPosition {
    fn from(value: (u8, u8)) -> Self {
        Self {
            col: value.0,
            row: value.1,
        }
    }
}

pub fn map(keycode: KeyCode) -> MatrixPosition {
    /*
     * ┌───┐   ┌───┬───┬───┬───┐ ┌───┬───┬───┬───┐ ┌───┬───┬───┬───┐
     * │Esc│   │F1 │F2 │F3 │F4 │ │F5 │F6 │F7 │F8 │ │F9 │F10│F11│F12│
     * └───┘   └───┴───┴───┴───┘ └───┴───┴───┴───┘ └───┴───┴───┴───┘
     * ┌───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───────┐ ┌───┬───┬───┬───┐
     * │ ` │ 1 │ 2 │ 3 │ 4 │ 5 │ 6 │ 7 │ 8 │ 9 │ 0 │ - │ = │ Backsp│ │Num│ / │ * │ - │
     * ├───┴─┬─┴─┬─┴─┬─┴─┬─┴─┬─┴─┬─┴─┬─┴─┬─┴─┬─┴─┬─┴─┬─┴─┬─┴─┬─────┤ ├───┼───┼───┼───┤
     * │ Tab │ Q │ W │ E │ R │ T │ Y │ U │ I │ O │ P │ [ │ ] │  \  │ │ 7 │ 8 │ 9 │   │
     * ├─────┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴─────┤ ├───┼───┼───┤ + │
     * │ Caps │ A │ S │ D │ F │ G │ H │ J │ K │ L │ ; │ ' │  Enter │ │ 4 │ 5 │ 6 │   │
     * ├──────┴─┬─┴─┬─┴─┬─┴─┬─┴─┬─┴─┬─┴─┬─┴─┬─┴─┬─┴─┬─┴─┬─┴────────┤ ├───┼───┼───┼───┤
     * │ Shift  │ Z │ X │ C │ V │ B │ N │ M │ , │ . │ / │    Shift │ │ 1 │ 2 │ 3 │   │
     * ├────┬───┴┬──┴─┬─┴───┴───┴───┴───┴───┴──┬┴───┼───┴┬────┬────┤ ├───┴───┼───┤Ent│
     * │Ctrl│GUI │Alt │                        │ Alt│ GUI│Menu│Ctrl│ │   0   │ . │   │
     * └────┴────┴────┴────────────────────────┴────┴────┴────┴────┘ └───────┴───┴───┘
     */

    match keycode {
        KeyCode::ESC              => (0, 0),
        KeyCode::F1               => (1, 0),
        KeyCode::F2               => (2, 0),
        KeyCode::F3               => (3, 0),
        KeyCode::F4               => (4, 0),
        KeyCode::F5               => (5, 0),
        KeyCode::F6               => (6, 0),
        KeyCode::F7               => (7, 0),
        KeyCode::F8               => (8, 0),
        KeyCode::F9               => (9, 0),
        KeyCode::F10              => (10,0),
        KeyCode::F11              => (11,0),
        KeyCode::F12              => (12,0),
        KeyCode::GRAVE            => (0, 1),
        KeyCode::KEY_1            => (1, 1),
        KeyCode::KEY_2            => (2, 1),
        KeyCode::KEY_3            => (3, 1),
        KeyCode::KEY_4            => (4, 1),
        KeyCode::KEY_5            => (5, 1),
        KeyCode::KEY_6            => (6, 1),
        KeyCode::KEY_7            => (7, 1),
        KeyCode::KEY_8            => (8, 1),
        KeyCode::KEY_9            => (9, 1),
        KeyCode::KEY_0            => (10,1),
        KeyCode::MINUS            => (11,1),
        KeyCode::EQUAL            => (12,1),
        KeyCode::BACKSPACE        => (13,1),
        KeyCode::TAB              => (2, 0),
        KeyCode::Q                => (2, 1),
        KeyCode::W                => (2, 2),
        KeyCode::E                => (2, 3),
        KeyCode::R                => (2, 4),
        KeyCode::T                => (2, 5),
        KeyCode::Y                => (2, 6),
        KeyCode::U                => (2, 7),
        KeyCode::I                => (2, 8),
        KeyCode::O                => (2, 9),
        KeyCode::P                => (2, 10),
        KeyCode::LEFTBRACE        => (2, 11),
        KeyCode::RIGHTBRACE       => (2, 12),
        KeyCode::ENTER            => (2, 13),
        KeyCode::LEFTCTRL         => (0, 0),
        KeyCode::A                => (0, 0),
        KeyCode::S                => (0, 0),
        KeyCode::D                => (0, 0),
        KeyCode::F                => (0, 0),
        KeyCode::G                => (0, 0),
        KeyCode::H                => (0, 0),
        KeyCode::J                => (0, 0),
        KeyCode::K                => (0, 0),
        KeyCode::L                => (0, 0),
        KeyCode::SEMICOLON        => (0, 0),
        KeyCode::APOSTROPHE       => (0, 0),
        KeyCode::LEFTSHIFT        => (0, 0),
        KeyCode::BACKSLASH        => (0, 0),
        KeyCode::Z                => (0, 0),
        KeyCode::X                => (0, 0),
        KeyCode::C                => (0, 0),
        KeyCode::V                => (0, 0),
        KeyCode::B                => (0, 0),
        KeyCode::N                => (0, 0),
        KeyCode::M                => (0, 0),
        KeyCode::COMMA            => (0, 0),
        KeyCode::DOT              => (0, 0),
        KeyCode::SLASH            => (0, 0),
        KeyCode::RIGHTSHIFT       => (0, 0),
        KeyCode::KPASTERISK       => (0, 0),
        KeyCode::LEFTALT          => (0, 0),
        KeyCode::SPACE            => (0, 0),
        KeyCode::CAPSLOCK         => (0, 0),
        KeyCode::NUMLOCK          => (0, 0),
        KeyCode::SCROLLLOCK       => (0, 0),
        KeyCode::KP7              => (0, 0),
        KeyCode::KP8              => (0, 0),
        KeyCode::KP9              => (0, 0),
        KeyCode::KPMINUS          => (0, 0),
        KeyCode::KP4              => (0, 0),
        KeyCode::KP5              => (0, 0),
        KeyCode::KP6              => (0, 0),
        KeyCode::KPPLUS           => (0, 0),
        KeyCode::KP1              => (0, 0),
        KeyCode::KP2              => (0, 0),
        KeyCode::KP3              => (0, 0),
        KeyCode::KP0              => (0, 0),
        KeyCode::KPDOT            => (0, 0),
        KeyCode::ZENKAKUHANKAKU   => (0, 0),
        KeyCode::KEY_102ND        => (0, 0),
        KeyCode::RO               => (0, 0),
        KeyCode::KATAKANA         => (0, 0),
        KeyCode::HIRAGANA         => (0, 0),
        KeyCode::HENKAN           => (0, 0),
        KeyCode::KATAKANAHIRAGANA => (0, 0),
        KeyCode::MUHENKAN         => (0, 0),
        KeyCode::KPJPCOMMA        => (0, 0),
        KeyCode::KPENTER          => (0, 0),
        KeyCode::RIGHTCTRL        => (0, 0),
        KeyCode::KPSLASH          => (0, 0),
        KeyCode::SYSRQ            => (0, 0),
        KeyCode::RIGHTALT         => (0, 0),
        KeyCode::LINEFEED         => (0, 0),
        KeyCode::HOME             => (0, 0),
        KeyCode::UP               => (0, 0),
        KeyCode::PAGEUP           => (0, 0),
        KeyCode::LEFT             => (0, 0),
        KeyCode::RIGHT            => (0, 0),
        KeyCode::END              => (0, 0),
        KeyCode::DOWN             => (0, 0),
        KeyCode::PAGEDOWN         => (0, 0),
        KeyCode::INSERT           => (0, 0),
        KeyCode::DELETE           => (0, 0),
        KeyCode::MACRO            => (0, 0),
        KeyCode::MUTE             => (0, 0),
        KeyCode::VOLUMEDOWN       => (0, 0),
        KeyCode::VOLUMEUP         => (0, 0),
        KeyCode::POWER            => (0, 0),
        KeyCode::KPEQUAL          => (0, 0),
        KeyCode::KPPLUSMINUS      => (0, 0),
        KeyCode::PAUSE            => (0, 0),
        KeyCode::SCALE            => (0, 0),
        KeyCode::KPCOMMA          => (0, 0),
        KeyCode::HANGEUL          => (0, 0),
        KeyCode::HANJA            => (0, 0),
        KeyCode::YEN              => (0, 0),
        KeyCode::LEFTMETA         => (0, 0),
        KeyCode::RIGHTMETA        => (0, 0),
        KeyCode::COMPOSE          => (0, 0),
        KeyCode::STOP             => (0, 0),
        KeyCode::AGAIN            => (0, 0),
        KeyCode::PROPS            => (0, 0),
        KeyCode::UNDO             => (0, 0),
        KeyCode::FRONT            => (0, 0),
        KeyCode::COPY             => (0, 0),
        KeyCode::OPEN             => (0, 0),
        KeyCode::PASTE            => (0, 0),
        KeyCode::FIND             => (0, 0),
        KeyCode::CUT              => (0, 0),
        KeyCode::HELP             => (0, 0),
        KeyCode::MENU             => (0, 0),
        KeyCode::CALC             => (0, 0),
        KeyCode::SETUP            => (0, 0),
        KeyCode::SLEEP            => (0, 0),
        KeyCode::WAKEUP           => (0, 0),
        KeyCode::FILE             => (0, 0),
        KeyCode::SENDFILE         => (0, 0),
        KeyCode::DELETEFILE       => (0, 0),
        KeyCode::XFER             => (0, 0),
        KeyCode::PROG1            => (0, 0),
        KeyCode::PROG2            => (0, 0),
        KeyCode::WWW              => (0, 0),
        KeyCode::MSDOS            => (0, 0),
        KeyCode::COFFEE           => (0, 0),
        KeyCode::ROTATE_DISPLAY   => (0, 0),
        KeyCode::CYCLEWINDOWS     => (0, 0),
        KeyCode::MAIL             => (0, 0),
        KeyCode::BOOKMARKS        => (0, 0),
        KeyCode::COMPUTER         => (0, 0),
        KeyCode::BACK             => (0, 0),
        KeyCode::FORWARD          => (0, 0),
        KeyCode::CLOSECD          => (0, 0),
        KeyCode::EJECTCD          => (0, 0),
        KeyCode::EJECTCLOSECD     => (0, 0),
        KeyCode::NEXTSONG         => (0, 0),
        KeyCode::PLAYPAUSE        => (0, 0),
        KeyCode::PREVIOUSSONG     => (0, 0),
        KeyCode::STOPCD           => (0, 0),
        KeyCode::RECORD           => (0, 0),
        KeyCode::REWIND           => (0, 0),
        KeyCode::PHONE            => (0, 0),
        KeyCode::ISO              => (0, 0),
        KeyCode::CONFIG           => (0, 0),
        KeyCode::HOMEPAGE         => (0, 0),
        KeyCode::REFRESH          => (0, 0),
        KeyCode::EXIT             => (0, 0),
        KeyCode::MOVE             => (0, 0),
        KeyCode::EDIT             => (0, 0),
        KeyCode::SCROLLUP         => (0, 0),
        KeyCode::SCROLLDOWN       => (0, 0),
        KeyCode::KPLEFTPAREN      => (0, 0),
        KeyCode::KPRIGHTPAREN     => (0, 0),
        KeyCode::NEW              => (0, 0),
        KeyCode::REDO             => (0, 0),
        KeyCode::F13              => (0, 0),
        KeyCode::F14              => (0, 0),
        KeyCode::F15              => (0, 0),
        KeyCode::F16              => (0, 0),
        KeyCode::F17              => (0, 0),
        KeyCode::F18              => (0, 0),
        KeyCode::F19              => (0, 0),
        KeyCode::F20              => (0, 0),
        KeyCode::F21              => (0, 0),
        KeyCode::F22              => (0, 0),
        KeyCode::F23              => (0, 0),
        KeyCode::F24              => (0, 0),
        KeyCode::PLAYCD           => (0, 0),
        KeyCode::PAUSECD          => (0, 0),
        KeyCode::PROG3            => (0, 0),
        KeyCode::PROG4            => (0, 0),
        KeyCode::ALL_APPLICATIONS => (0, 0),
        KeyCode::SUSPEND          => (0, 0),
        KeyCode::CLOSE            => (0, 0),
        KeyCode::PLAY             => (0, 0),
        KeyCode::FASTFORWARD      => (0, 0),
        KeyCode::BASSBOOST        => (0, 0),
        KeyCode::HP               => (0, 0),
        KeyCode::CAMERA           => (0, 0),
        KeyCode::SOUND            => (0, 0),
        KeyCode::QUESTION         => (0, 0),
        KeyCode::EMAIL            => (0, 0),
        KeyCode::CHAT             => (0, 0),
        KeyCode::SEARCH           => (0, 0),
        KeyCode::CONNECT          => (0, 0),
        KeyCode::FINANCE          => (0, 0),
        KeyCode::SPORT            => (0, 0),
        KeyCode::SHOP             => (0, 0),
        KeyCode::ALTERASE         => (0, 0),
        KeyCode::CANCEL           => (0, 0),
        KeyCode::BRIGHTNESSDOWN   => (0, 0),
        KeyCode::BRIGHTNESSUP     => (0, 0),
        KeyCode::MEDIA            => (0, 0),
        KeyCode::SWITCHVIDEOMODE  => (0, 0),
        KeyCode::KBDILLUMTOGGLE   => (0, 0),
        KeyCode::KBDILLUMDOWN     => (0, 0),
        KeyCode::KBDILLUMUP       => (0, 0),
        KeyCode::SEND             => (0, 0),
        KeyCode::REPLY            => (0, 0),
        KeyCode::FORWARDMAIL      => (0, 0),
        KeyCode::SAVE             => (0, 0),
        KeyCode::DOCUMENTS        => (0, 0),
        KeyCode::BATTERY          => (0, 0),
        KeyCode::BLUETOOTH        => (0, 0),
        KeyCode::WLAN             => (0, 0),
        KeyCode::UWB              => (0, 0),
        KeyCode::UNKNOWN          => (0, 0),
        KeyCode::VIDEO_NEXT       => (0, 0),
        KeyCode::VIDEO_PREV       => (0, 0),
        KeyCode::BRIGHTNESS_CYCLE => (0, 0),
        KeyCode::BRIGHTNESS_AUTO  => (0, 0),
        KeyCode::DISPLAY_OFF      => (0, 0),
        KeyCode::WWAN             => (0, 0),
        KeyCode::RFKILL           => (0, 0),
        KeyCode::MICMUTE          => (0, 0),
        _ => (0, 0),
    }
    .into()
}
