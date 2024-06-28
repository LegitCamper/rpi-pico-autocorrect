#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::channel::{Channel, Sender};
use embassy_time::Timer;
use embedded_alloc::Heap;
use keycode::{KeyMap, KeyMappingId, KeyState, KeyboardState};

#[global_allocator]
static HEAP: Heap = Heap::empty();

mod write;
use write::write;

// this imports the TREE
// use embedded_bktree::Node;
// include!(concat!(env!("OUT_DIR"), "/tree.rs"));

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    let p = embassy_rp::init(Default::default());

    // this is the communication between the reader and writer
    static mut WRITER_CHANNEL: Channel<NoopRawMutex, [u8; 6], 64> = Channel::new();

    // spawn usb writer (emulates keyboard)
    spawner
        .spawn(write(p.USB, unsafe { WRITER_CHANNEL.receiver() }))
        .unwrap();

    // spawn usb keyboard reader
    // spawner.spawn(write(p.USB)).unwrap();

    let mut keyboard_state = KeyboardState::new(None);
    let mut channel = unsafe { WRITER_CHANNEL.sender() };
    loop {
        hello_world(&mut channel, &mut keyboard_state).await;
    }
}

async fn hello_world(
    channel: &mut Sender<'static, NoopRawMutex, [u8; 6], 64>,
    state: &mut KeyboardState,
) {
    Timer::after_secs(1).await;
    send_single_unmodded_key(channel, state, KeyMap::from(KeyMappingId::UsH)).await;
    send_single_unmodded_key(channel, state, KeyMap::from(KeyMappingId::UsE)).await;
    send_single_unmodded_key(channel, state, KeyMap::from(KeyMappingId::UsL)).await;
    send_single_unmodded_key(channel, state, KeyMap::from(KeyMappingId::UsL)).await;
    send_single_unmodded_key(channel, state, KeyMap::from(KeyMappingId::UsO)).await;
    send_single_unmodded_key(channel, state, KeyMap::from(KeyMappingId::Space)).await;
    send_single_unmodded_key(channel, state, KeyMap::from(KeyMappingId::UsW)).await;
    send_single_unmodded_key(channel, state, KeyMap::from(KeyMappingId::UsO)).await;
    send_single_unmodded_key(channel, state, KeyMap::from(KeyMappingId::UsR)).await;
    send_single_unmodded_key(channel, state, KeyMap::from(KeyMappingId::UsL)).await;
    send_single_unmodded_key(channel, state, KeyMap::from(KeyMappingId::UsD)).await;
    Timer::after_secs(1).await;
    for _ in 0..12 {
        send_single_unmodded_key(channel, state, KeyMap::from(KeyMappingId::Backspace)).await;
    }
}

async fn send_single_unmodded_key(
    channel: &mut Sender<'static, NoopRawMutex, [u8; 6], 64>,
    state: &mut KeyboardState,
    key: KeyMap,
) {
    state.update_key(key, KeyState::Pressed);
    channel
        .send(state.usb_input_report()[2..].try_into().unwrap())
        .await;
    state.update_key(key, KeyState::Released);
    channel
        .send(state.usb_input_report()[2..].try_into().unwrap())
        .await;
}
