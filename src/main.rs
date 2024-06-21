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

// this imports the TREE
use embedded_bktree::Node;
include!(concat!(env!("OUT_DIR"), "/tree.rs"));

#[bsp::entry]
fn main() -> ! {
    loop {}
}
