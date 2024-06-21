use cortex_m::delay::Delay;
use defmt::*;
use defmt_rtt as _;
use embedded_alloc::Heap;
use embedded_hal::digital::OutputPin;
use panic_probe as _;

#[global_allocator]
static HEAP: Heap = Heap::empty();

use super::bsp;
use bsp::{
    hal::{
        self,
        clocks::{init_clocks_and_plls, Clock},
        gpio::{self, Pin, bank0::Gpio25, FunctionSio, SioOutput, PullDown, Function, PullType},
        pac,
        sio::Sio,
        watchdog::Watchdog,
    },
    Pins,
};

// USB Device support
use usb_device::{class_prelude::*, prelude::*};

// USB Human Interface Device (HID) Class support
use usbd_hid::descriptor::generator_prelude::*;
use usbd_hid::descriptor::MouseReport;
use usbd_hid::hid_class::HIDClass;

/// The USB Device Driver (shared with the interrupt).
static mut USB_DEVICE: Option<UsbDevice<hal::usb::UsbBus>> = None;

/// The USB Bus Driver (shared with the interrupt).
static mut USB_BUS: Option<UsbBusAllocator<hal::usb::UsbBus>> = None;

/// The USB Human Interface Device Driver (shared with the interrupt).
static mut USB_HID: Option<HIDClass<hal::usb::UsbBus>> = None;

// pub struct Device<F,P> 
// where
//     F: Function,
//     P: PullType,
pub struct Device
{
    // led_pin: Pin<Gpio25, F, P>
    delay: Delay,
}

// impl<F, P> Device<F, P> 
// where
//     F: Function,
//     P: PullType,
impl Device 
{
    pub fn new() -> Self{
        let mut pac = pac::Peripherals::take().unwrap();
        let core = pac::CorePeripherals::take().unwrap();
        let mut watchdog = Watchdog::new(pac.WATCHDOG);
        let sio = Sio::new(pac.SIO);

        // External high-speed crystal on the pico board is 12Mhz
        let external_xtal_freq_hz = 12_000_000u32;
        let clocks = init_clocks_and_plls(
            external_xtal_freq_hz,
            pac.XOSC,
            pac.CLOCKS,
            pac.PLL_SYS,
            pac.PLL_USB,
            &mut pac.RESETS,
            &mut watchdog,
        )
        .ok()
        .unwrap();

        let pins = Pins::new(
            pac.IO_BANK0,
            pac.PADS_BANK0,
            sio.gpio_bank0,
            &mut pac.RESETS,
        );

        Self { 
            // led_pin: pins.led.into_push_pull_output(), 
            delay:  Delay::new(core.SYST, clocks.system_clock.freq().to_Hz())
        }
    }
    // usb stuff here
    // https://github.com/rp-rs/rp-hal-boards/blob/HEAD/boards/rp-pico/examples/pico_usb_twitchy_mouse.rs
    
    
    // pub fn led_on(&mut self) {
    //     self.led_pin.set_high().unwrap();
    // }
    // pub fn led_off(&mut self) {
    //     self.led_pin.set_low().unwrap();
    // }
    // pub fn blink_led_once(&mut self, delay: u32) {
    //     self.led_on();
    //     self.delay.delay_ms(delay);
    //     self.led_off();
    //     self.delay.delay_ms(delay);
    // }
    // pub fn blink_led_mult(&mut self, delay: u32, times: u32) {
    //     for _ in 0..times {
    //         self.blink_led_once(delay)
    //     }
    // }
}
