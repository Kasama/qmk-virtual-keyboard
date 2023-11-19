use std::fs::File;
use std::mem::{size_of, transmute};
use std::os::fd::AsRawFd;

use nix::errno::Errno;
use nix::libc::input_event;
use std::ops::{Deref, DerefMut};

use crate::key_event::{IOctlOp, KeyEvent, EVIOCGRAB};

pub struct EventDevice(std::fs::File);
impl Deref for EventDevice {
    type Target = std::fs::File;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for EventDevice {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Drop for EventDevice {
    fn drop(&mut self) {
        unsafe {
            nix::libc::ioctl(self.as_raw_fd(), EVIOCGRAB, IOctlOp::Ungrab);
        }
    }
}

impl EventDevice {
    pub fn new(file: File) -> Result<Self, std::io::Error> {
        unsafe {
            let r = nix::libc::ioctl(file.as_raw_fd(), EVIOCGRAB, IOctlOp::Grab);
            if r < 0 {
                Err(std::io::Error::last_os_error())
            } else {
                Ok(EventDevice(file))
            }
        }
    }

    pub fn from_path<S: AsRef<str>>(path: S) -> Result<Self, anyhow::Error> {
        let f = std::fs::OpenOptions::new().read(true).open(path.as_ref())?;

        Ok(Self::new(f)?)
    }

    pub fn next(&self) -> Result<KeyEvent, Errno> {
        let mut buf = [0u8; size_of::<nix::libc::input_event>()];

        nix::unistd::read(self.as_raw_fd(), &mut buf)?;

        let input_event: input_event = unsafe { transmute(buf) };
        let key_event: KeyEvent = input_event.into();

        let debug_ev: input_event = key_event.into();
        // println!("----------------");
        // println!("{:?}", key_event);
        // println!("{:?}", input_event);
        // println!("{:?}", debug_ev);

        Ok(input_event.into())
    }
}
