#![no_std]
#![no_main]

use {defmt_rtt as _, panic_probe as _};

extern crate alloc;
use alloc::vec::Vec;
use defmt::info;
use embassy_executor::Spawner;
use embassy_rp::{
    bind_interrupts,
    peripherals::UART0,
    uart::{Config, DataBits, InterruptHandler as UARTInterruptHandler, Parity, StopBits, Uart},
};
use embassy_time::Timer;
use embedded_alloc::Heap;
use postcard::to_allocvec;
use usbd_hid::descriptor::KeyboardReport;

#[global_allocator]
static HEAP: Heap = Heap::empty();

bind_interrupts!(pub struct Irqs {
    UART0_IRQ  => UARTInterruptHandler<UART0>;
});

// mod read;
// use read::read;
use shared::MyKeyboardReport;

fn create_allocator() {
    use core::mem::MaybeUninit;
    const HEAP_SIZE: usize = 1024;
    static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
    unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    create_allocator();
    let p = embassy_rp::init(Default::default());

    let mut config = Config::default();
    config.baudrate = 57600;
    config.stop_bits = StopBits::STOP1;
    config.data_bits = DataBits::DataBits8;
    config.parity = Parity::ParityNone;

    let (uart, tx_pin, tx_dma, rx_pin, rx_dma) =
        (p.UART0, p.PIN_16, p.DMA_CH0, p.PIN_17, p.DMA_CH1);
    let uart = Uart::new(uart, tx_pin, rx_pin, Irqs, tx_dma, rx_dma, config);
    let (mut tx, _) = uart.split();

    let rep: Vec<u8> = to_allocvec(&MyKeyboardReport::from(KeyboardReport {
        modifier: 0,
        reserved: 0,
        leds: 0,
        keycodes: [4, 0, 0, 0, 0, 0],
    }))
    .unwrap();

    loop {
        for buf in rep.iter() {
            tx.blocking_write(&[*buf]).unwrap()
        }
        tx.blocking_flush().unwrap();
        info!("flushed");
        Timer::after_secs(1).await;
    }
}
