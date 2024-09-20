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

    SimpleLogger::init_logger_from_env();

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

const LOG_TARGETS: Option<&'static str> = option_env!("ESP_LOGTARGETS");
struct SimpleLogger;

impl SimpleLogger {
    pub fn init_logger_from_env() {
        unsafe {
            log::set_logger_racy(&Self).unwrap();
        }

        const LEVEL: Option<&'static str> = option_env!("ESP_LOGLEVEL");

        if let Some(lvl) = LEVEL {
            let level = <log::LevelFilter as core::str::FromStr>::from_str(lvl)
                .unwrap_or_else(|_| log::LevelFilter::Off);
            unsafe { log::set_max_level_racy(level) };
        }
    }
}

impl log::Log for SimpleLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        // check enabled log targets if any
        if let Some(targets) = LOG_TARGETS {
            if targets
                .split(",")
                .find(|v| record.target().starts_with(v))
                .is_none()
            {
                return;
            }
        }

        const RESET: &str = "\u{001B}[0m";
        const RED: &str = "\u{001B}[31m";
        const GREEN: &str = "\u{001B}[32m";
        const YELLOW: &str = "\u{001B}[33m";
        const BLUE: &str = "\u{001B}[34m";
        const CYAN: &str = "\u{001B}[35m";

        let color = match record.level() {
            log::Level::Error => RED,
            log::Level::Warn => YELLOW,
            log::Level::Info => GREEN,
            log::Level::Debug => BLUE,
            log::Level::Trace => CYAN,
        };

        esp_println::println!("{}{} - {}{}", color, record.level(), record.args(), RESET);
    }

    fn flush(&self) {}
}
