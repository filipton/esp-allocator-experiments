#![no_std]
#![no_main]

use alloc::{
    string::{String, ToString},
    vec,
    vec::Vec,
};
use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl, delay::Delay, peripherals::Peripherals, prelude::*, system::SystemControl,
};
use serde::{Deserialize, Serialize};
extern crate alloc;
use core::mem::MaybeUninit;

#[global_allocator]
static ALLOCATOR: esp_alloc::EspHeap = esp_alloc::EspHeap::empty();

fn init_heap() {
    const HEAP_SIZE: usize = 150 * 1024;
    static mut HEAP: MaybeUninit<[u8; HEAP_SIZE]> = MaybeUninit::uninit();

    unsafe {
        ALLOCATOR.init(HEAP.as_mut_ptr() as *mut u8, HEAP_SIZE);
    }
}

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);

    let clocks = ClockControl::max(system.clock_control).freeze();
    let delay = Delay::new(&clocks);
    init_heap();

    esp_println::logger::init_logger_from_env();

    let test_ser = TestEnum::Test2(TestStruct {
        names: vec![
            "Filip".to_string(),
            "cxzczx".to_string(),
            "ewqewqgrhegfhejrwkhgkrfjewhkj".to_string(),
        ],
        id: 582948912849169420,
    });
    let res = serde_json::to_string(&test_ser).unwrap();
    log::info!("Serialized: {:?}", res);

    let test_deser: TestEnum = serde_json::from_str(&res).unwrap();
    log::info!("Deserialized: {:?}", test_deser);

    loop {
        log::info!("Hello world!");
        delay.delay(500.millis());
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "data")]
enum TestEnum {
    Test1 { wow: u128, no: String },
    Test2(TestStruct),
}

#[derive(Serialize, Deserialize, Debug)]
struct TestStruct {
    names: Vec<String>,
    id: u128,
}
