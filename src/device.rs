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
        clocks::Clock,
        gpio::{bank0::Gpio25, DefaultTypeState, FunctionSio, Pin, SioOutput},
        pac,
        sio::Sio,
        watchdog::Watchdog,
    },
    Pins,
};

// The macro for marking our interrupt functions
use rp_pico::hal::pac::interrupt;

/// This function is called whenever the USB Hardware generates an Interrupt
/// Request.
#[allow(non_snake_case)]
#[interrupt]
unsafe fn USBCTRL_IRQ() {
    // Handle USB request
    let usb_dev = USB_DEVICE.as_mut().unwrap();
    let usb_hid = USB_HID.as_mut().unwrap();
    usb_dev.poll(&mut [usb_hid]);
}

// USB Device support
use usb_device::{class_prelude::*, prelude::*};

// USB Human Interface Device (HID) Class support
use usbd_hid::descriptor::generator_prelude::*;
pub use usbd_hid::descriptor::KeyboardReport;
use usbd_hid::hid_class::HIDClass;

/// The USB Device Driver (shared with the interrupt).
static mut USB_DEVICE: Option<UsbDevice<hal::usb::UsbBus>> = None;

/// The USB Bus Driver (shared with the interrupt).
static mut USB_BUS: Option<UsbBusAllocator<hal::usb::UsbBus>> = None;

/// The USB Human Interface Device Driver (shared with the interrupt).
static mut USB_HID: Option<HIDClass<hal::usb::UsbBus>> = None;

// use keebrs::{keycode::KeyCode, translate::*};

pub struct Device {
    led_pin: Pin<Gpio25, FunctionSio<SioOutput>, <Gpio25 as DefaultTypeState>::PullType>,
    delay: Delay,
}

impl Device {
    pub fn new() -> Self {
        let mut pac = pac::Peripherals::take().unwrap();
        let core = pac::CorePeripherals::take().unwrap();
        let mut watchdog = Watchdog::new(pac.WATCHDOG);
        let sio = Sio::new(pac.SIO);

        // Configure the clocks
        //
        // The default is to generate a 125 MHz system clock
        let clocks = hal::clocks::init_clocks_and_plls(
            rp_pico::XOSC_CRYSTAL_FREQ,
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

        // Set up the USB driver
        let usb_bus = UsbBusAllocator::new(hal::usb::UsbBus::new(
            pac.USBCTRL_REGS,
            pac.USBCTRL_DPRAM,
            clocks.usb_clock,
            true,
            &mut pac.RESETS,
        ));
        unsafe {
            // Note (safety): This is safe as interrupts haven't been started yet
            USB_BUS = Some(usb_bus);
        }

        // Grab a reference to the USB Bus allocator. We are promising to the
        // compiler not to take mutable access to this global variable whilst this
        // reference exists!
        let bus_ref = unsafe { USB_BUS.as_ref().unwrap() };

        // Set up the USB HID Class Device driver, providing Keyboard Reports
        let usb_hid = HIDClass::new(bus_ref, KeyboardReport::desc(), 60);
        unsafe {
            // Note (safety): This is safe as interrupts haven't been started yet.
            USB_HID = Some(usb_hid);
        }

        // Create a USB device with a fake VID and PID
        let usb_dev = UsbDeviceBuilder::new(bus_ref, UsbVidPid(0x16c0, 0x27da))
            .strings(&[StringDescriptors::default()
                .product("AutoCorrect MITM")
                .serial_number("TEST")])
            .unwrap()
            .device_class(0)
            .build();
        unsafe {
            // Note (safety): This is safe as interrupts haven't been started yet
            USB_DEVICE = Some(usb_dev);
        }

        unsafe {
            // Enable the USB interrupt
            pac::NVIC::unmask(hal::pac::Interrupt::USBCTRL_IRQ);
        };

        Self {
            led_pin: pins.led.into_push_pull_output(),
            delay: Delay::new(core.SYST, clocks.system_clock.freq().to_Hz()),
        }
    }

    // We do this with interrupts disabled, to avoid a race hazard with the USB IRQ.
    pub fn push_keyboard_report(
        &self,
        report: KeyboardReport,
    ) -> Result<usize, usb_device::UsbError> {
        critical_section::with(|_| unsafe {
            // Now interrupts are disabled, grab the global variable and, if
            // available, send it a HID report
            USB_HID.as_mut().map(|hid| hid.push_input(&report))
        })
        .unwrap()
    }

    // pub fn get_keycode(key: char) -> KeyCode {
    //     match key {
    //         'a' => KeyCode::KbA,
    //         'b' => KeyCode::KbB,
    //         'c' => KeyCode::KbC,
    //         'd' => KeyCode::KbD,
    //         'e' => KeyCode::KbE,
    //         'f' => KeyCode::KbF,
    //         'g' => KeyCode::KbG,
    //         'h' => KeyCode::KbH,
    //         'i' => KeyCode::KbI,
    //         'j' => KeyCode::KbJ,
    //         'k' => KeyCode::KbK,
    //         'l' => KeyCode::KbL,
    //         'm' => KeyCode::KbM,
    //         'n' => KeyCode::KbN,
    //         'o' => KeyCode::KbO,
    //         'p' => KeyCode::KbP,
    //         'q' => KeyCode::KbQ,
    //         'r' => KeyCode::KbR,
    //         's' => KeyCode::KbS,
    //         't' => KeyCode::KbT,
    //         'u' => KeyCode::KbU,
    //         'v' => KeyCode::KbV,
    //         'w' => KeyCode::KbW,
    //         'x' => KeyCode::KbX,
    //         'y' => KeyCode::KbY,
    //         'z' => KeyCode::KbZ,
    //     }
    // }

    pub fn get_key_rep(&self, key: u8) -> KeyboardReport {
        KeyboardReport {
            modifier: 0,
            reserved: 0,
            leds: 0,
            keycodes: [key; 6],
        }
    }

    pub fn led_on(&mut self) {
        self.led_pin.set_high().unwrap();
    }
    pub fn led_off(&mut self) {
        self.led_pin.set_low().unwrap();
    }
    pub fn blink_led_once(&mut self, delay: u32) {
        self.led_on();
        self.delay.delay_ms(delay);
        self.led_off();
        self.delay.delay_ms(delay);
    }
    pub fn blink_led_mult(&mut self, delay: u32, times: u32) {
        for _ in 0..times {
            self.blink_led_once(delay)
        }
    }
}
