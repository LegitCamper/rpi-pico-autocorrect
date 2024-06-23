//! Blinks the LED on a Pico board
//!
//! This will blink an LED attached to GP25, which is the pin the Pico uses for the on-board LED.
#![no_std]
#![no_main]

// Provide an alias for our BSP so we can switch targets quickly.
// Uncomment the BSP you included in Cargo.toml, the rest of the code does not need to change.
use rp_pico as bsp;
// use sparkfun_pro_micro_rp2040 as bsp;

pub mod device;
use device::{Device, KeyboardReport};

// this imports the TREE
use embedded_bktree::Node;
include!(concat!(env!("OUT_DIR"), "/tree.rs"));

#[bsp::entry]
fn main() -> ! {
    let mut device = Device::new();

    loop {
        device
            .push_keyboard_report(device.get_key_rep(0x0B))
            .unwrap(); // H
        device
            .push_keyboard_report(device.get_key_rep(0x08))
            .unwrap(); // E
        device
            .push_keyboard_report(device.get_key_rep(0x0F))
            .unwrap(); // L
        device
            .push_keyboard_report(device.get_key_rep(0x0F))
            .unwrap(); // L
        device
            .push_keyboard_report(device.get_key_rep(0x12))
            .unwrap(); // O
        device
            .push_keyboard_report(device.get_key_rep(0x2C))
            .unwrap(); // _
        device
            .push_keyboard_report(device.get_key_rep(0x1A))
            .unwrap(); // W
        device
            .push_keyboard_report(device.get_key_rep(0x12))
            .unwrap(); // O
        device
            .push_keyboard_report(device.get_key_rep(0x15))
            .unwrap(); // R
        device
            .push_keyboard_report(device.get_key_rep(0x0F))
            .unwrap(); // L
        device
            .push_keyboard_report(device.get_key_rep(0x07))
            .unwrap(); // D
        for _ in 0..11 {
            device
                .push_keyboard_report(device.get_key_rep(0x2A))
                .unwrap(); // Back
        }
        device.blink_led_once(3000);
    }
}
