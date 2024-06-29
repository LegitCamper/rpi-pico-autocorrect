#![no_std]
#![no_main]

use {defmt_rtt as _, panic_probe as _};

extern crate alloc;
use alloc::vec::Vec;
use defmt::{error, info};
use embassy_executor::Spawner;
use embassy_rp::{
    bind_interrupts,
    peripherals::UART0,
    uart::{Config, DataBits, InterruptHandler as UARTInterruptHandler, Parity, StopBits, Uart},
    Peripheral, Peripherals,
};
use embassy_sync::{
    blocking_mutex::raw::NoopRawMutex,
    channel::{Channel, Sender},
};
use embassy_time::Timer;
use embedded_alloc::Heap;
use keycode::{KeyMap, KeyMappingId, KeyState, KeyboardState};
use postcard::to_allocvec;

#[global_allocator]
static HEAP: Heap = Heap::empty();

bind_interrupts!(pub struct Irqs {
    UART0_IRQ  => UARTInterruptHandler<UART0>;
});

mod write;
use write::write;

use shared::MyKeyboardReport;

// this imports the TREE
// use embedded_bktree::Node;
// include!(concat!(env!("OUT_DIR"), "/tree.rs"));

fn create_allocator() {
    use core::mem::MaybeUninit;
    const HEAP_SIZE: usize = 1024;
    static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
    unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    create_allocator();
    let p = embassy_rp::init(Default::default());

    // this is the communication between the reader and writer
    static mut WRITER_CHANNEL: Channel<NoopRawMutex, [u8; 6], 64> = Channel::new();

    // spawn usb writer (emulates keyboard)
    spawner
        .spawn(unsafe { write(p.USB.clone_unchecked(), WRITER_CHANNEL.receiver()) })
        .unwrap();

    let channel = unsafe { WRITER_CHANNEL.sender() };

    let mut config = Config::default();
    config.baudrate = 57600;
    config.stop_bits = StopBits::STOP1;
    config.data_bits = DataBits::DataBits8;
    config.parity = Parity::ParityNone;

    let (uart, tx_pin, tx_dma, rx_pin, rx_dma) =
        (p.UART0, p.PIN_16, p.DMA_CH0, p.PIN_17, p.DMA_CH1);
    let uart = Uart::new(uart, tx_pin, rx_pin, Irqs, tx_dma, rx_dma, config);
    let (_, mut rx) = uart.split();

    let mut read_buf: [u8; 1] = [0; 1]; // Can only read one byte at a time!
    let mut data_read: Vec<u8> = Vec::new(); // Save buffer.

    loop {
        // We assume that the receiving cannot fail
        rx.read(&mut read_buf).await.unwrap();
        data_read.push(read_buf[0]);

        if let Ok(rep) = MyKeyboardReport::new(&data_read) {
            channel.send(rep.keycodes).await;
            data_read.clear()
        }
    }
}
