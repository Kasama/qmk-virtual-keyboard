use std::io::Write;
use std::os::fd::AsRawFd;

use nix::libc::ioctl;

use crate::key_event::{KeyEvent, EV_KEY};

const UI_SET_EVBIT: u64 = 1074025828;
const UI_SET_KEYBIT: u64 = 1074025829;
const UI_DEV_SETUP: u64 = 1079792899;
const UI_DEV_CREATE: u64 = 21761;
const UI_DEV_DESTROY: u64 = 21762;

const BUS_USB: u16 = 3;

pub struct UinputKeyboard(std::fs::File);

impl UinputKeyboard {
    pub fn new(
        name: String,
        vendor: u16,
        product: u16,
        version: u16,
    ) -> Result<Self, std::io::Error> {
        let f = std::fs::OpenOptions::new()
            .write(true)
            .open("/dev/uinput")?;

        let mut uinput_setup = nix::libc::uinput_setup {
            id: nix::libc::input_id {
                bustype: BUS_USB,
                vendor,
                product,
                version,
            },
            name: [0; 80],
            ff_effects_max: 0,
        };

        unsafe {
            let r = ioctl(f.as_raw_fd(), UI_SET_EVBIT, EV_KEY as u32);
            if r < 0 {
                return Err(std::io::Error::last_os_error());
            }

            for key in 0..255 {
                ioctl(f.as_raw_fd(), UI_SET_KEYBIT, key);
                if r < 0 {
                    return Err(std::io::Error::last_os_error());
                }
            }

            nix::libc::strncpy(uinput_setup.name.as_mut_ptr(), name.as_ptr() as *mut i8, 79);

            let r = ioctl(f.as_raw_fd(), UI_DEV_SETUP, &uinput_setup);
            if r < 0 {
                return Err(std::io::Error::last_os_error());
            }

            let r = ioctl(f.as_raw_fd(), UI_DEV_CREATE, 0);
            if r < 0 {
                return Err(std::io::Error::last_os_error());
            }
        }

        Ok(Self(f))
    }
}

impl Drop for UinputKeyboard {
    fn drop(&mut self) {
        unsafe {
            ioctl(self.0.as_raw_fd(), UI_DEV_DESTROY);
        }
    }
}

impl UinputKeyboard {
    // Sends a single event
    pub fn send_event_only(&mut self, event: KeyEvent) -> anyhow::Result<()> {
        let input_event: nix::libc::input_event = event.into();

        let buf = unsafe {
            std::slice::from_raw_parts(
                &input_event as *const _ as *const u8,
                std::mem::size_of::<nix::libc::input_event>(),
            )
        };

        self.0.write_all(buf)?;

        Ok(())
    }

    // Sends an event followed by a sync
    pub fn send_event(&mut self, event: KeyEvent) -> anyhow::Result<()> {
        self.send_event_only(event)?;
        self.send_event_only(KeyEvent::Sync(0, Default::default()))
    }
}
