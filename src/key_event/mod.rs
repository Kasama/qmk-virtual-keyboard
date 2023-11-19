pub mod key_code;
pub mod matix_mapper;

use std::time::{Duration, SystemTime};

use nix::libc::{input_event, timeval};

use self::key_code::KeyCode;

pub enum IOctlOp {
    Ungrab = 0,
    Grab = 1,
}

pub const EVIOCGRAB: u64 = 0x40044590;

// input_event types
pub const EV_SYN: u16 = 0x00;
pub const EV_KEY: u16 = 0x01;
// pub const EV_REL: u16 = 0x02;
// pub const EV_ABS: u16 = 0x03;
pub const EV_MSC: u16 = 0x04;
// pub const EV_SW: u16 = 0x05;
// pub const EV_LED: u16 = 0x11;
// pub const EV_SND: u16 = 0x12;
// pub const EV_REP: u16 = 0x14;
// pub const EV_FF: u16 = 0x15;
// pub const EV_PWR: u16 = 0x16;
// pub const EV_FF_STATUS: u16 = 0x17;

// Sync events
pub const SYN_REPORT: u16 = 0;
// pub const SYN_CONFIG: u16 = 1;
// pub const SYN_MT_REPORT: u16 = 2;
// pub const SYN_DROPPED: u16 = 3;

// Misc events
// pub const MSC_SERIAL: u16 = 0x00;
// pub const MSC_PULSELED: u16 = 0x01;
// pub const MSC_GESTURE: u16 = 0x02;
// pub const MSC_RAW: u16 = 0x03;
pub const MSC_SCAN: u16 = 0x04;
// pub const MSC_TIMESTAMP: u16 = 0x05;

pub const KEY_RELEASE: i32 = 0;
pub const KEY_PRESS: i32 = 1;
pub const KEY_HOLD: i32 = 2;

#[derive(Debug, Clone, Copy)]
pub struct MyTime(SystemTime);

impl Default for MyTime {
    fn default() -> Self {
        Self(SystemTime::now())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum KeyEvent {
    Press(KeyCode, bool, MyTime),
    Release(KeyCode, MyTime),
    Sync(i32, MyTime),
    Scancode(i32, MyTime),
    Unsupported(u16, u16, i32, MyTime),
}

impl From<input_event> for KeyEvent {
    fn from(value: input_event) -> Self {
        match (value.type_, value.code, value.value) {
            (EV_SYN, SYN_REPORT, v) => KeyEvent::Sync(v, value.time.into()),
            (EV_MSC, MSC_SCAN, v) => KeyEvent::Scancode(v, value.time.into()),
            (EV_KEY, code, action) => match (KeyCode::from_repr(code), action) {
                (Some(code), KEY_RELEASE) => KeyEvent::Release(code, value.time.into()),
                (Some(code), KEY_PRESS) => KeyEvent::Press(code, false, value.time.into()),
                (Some(code), KEY_HOLD) => KeyEvent::Press(code, true, value.time.into()),
                (_, _) => KeyEvent::Unsupported(EV_KEY, code, action, value.time.into()),
            },
            (type_, code, v) => KeyEvent::Unsupported(type_, code, v, value.time.into()),
        }
    }
}

impl From<timeval> for MyTime {
    fn from(value: timeval) -> Self {
        Self(SystemTime::UNIX_EPOCH + Duration::new(value.tv_sec as u64, value.tv_usec as u32))
    }
}

impl From<MyTime> for timeval {
    fn from(value: MyTime) -> Self {
        timeval {
            tv_sec: value
                .0
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64,
            tv_usec: value
                .0
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .subsec_micros() as i64,
        }
    }
}

impl From<KeyEvent> for input_event {
    fn from(value: KeyEvent) -> Self {
        match value {
            KeyEvent::Press(code, repeat, time) => input_event {
                time: time.into(),
                type_: EV_KEY,
                code: code.into(),
                value: if repeat { KEY_HOLD } else { KEY_PRESS },
            },
            KeyEvent::Release(code, time) => input_event {
                time: time.into(),
                type_: EV_KEY,
                code: code.into(),
                value: KEY_RELEASE,
            },
            KeyEvent::Sync(value, time) => input_event {
                time: time.into(),
                type_: EV_SYN,
                code: SYN_REPORT,
                value,
            },
            KeyEvent::Scancode(value, time) => input_event {
                time: time.into(),
                type_: EV_MSC,
                code: MSC_SCAN,
                value,
            },
            KeyEvent::Unsupported(type_, code, value, time) => input_event {
                time: time.into(),
                type_,
                code,
                value,
            },
        }
    }
}
