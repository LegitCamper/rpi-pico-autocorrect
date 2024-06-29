#![no_std]

use core::convert::From;
use postcard::{from_bytes, to_allocvec, Error};
use serde::{Deserialize, Serialize};
use usbd_hid::descriptor::KeyboardReport;
extern crate alloc;
use alloc::vec::Vec;

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct MyKeyboardReport {
    pub modifier: u8,
    pub reserved: u8,
    pub leds: u8,
    pub keycodes: [u8; 6],
}
impl From<KeyboardReport> for MyKeyboardReport {
    fn from(item: KeyboardReport) -> Self {
        MyKeyboardReport {
            modifier: item.modifier,
            reserved: item.reserved,
            leds: item.leds,
            keycodes: item.keycodes,
        }
    }
}
impl MyKeyboardReport {
    pub fn new(bytes: &[u8]) -> Result<Self, Error> {
        from_bytes(bytes)
    }
    pub fn to_keyboard_report(&self) -> KeyboardReport {
        KeyboardReport {
            modifier: self.modifier,
            reserved: self.reserved,
            leds: self.leds,
            keycodes: self.keycodes,
        }
    }
    pub fn to_vec(&self) -> Result<Vec<u8>, Error> {
        to_allocvec(self)
    }
}
