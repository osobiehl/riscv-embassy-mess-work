#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use core::fmt::Write;

// use esp32c3_hal::{pac::{Peripherals, LEDC, apb_ctrl::peri_backup_config}, prelude::*, RtcCntl, Serial, Timer as old_timer};
use embassy::executor::Spawner;
use embassy::time::{Duration, Timer};
use panic_halt as _;
// use embassy_macros::{main, task};
use core::cell::RefCell;
use embassy_esp32c3::pac::{Peripherals, UART0};
use embassy_esp32c3::Serial;

use embassy::blocking_mutex::CriticalSectionMutex as Mutex;

static mut SERIAL: Mutex<RefCell<Option<Serial<UART0>>>> = Mutex::new(RefCell::new(None));

fn log_interrupt(msg: &str) {
    critical_section::with(|cs| unsafe {
        let mut serial = SERIAL.borrow(cs).borrow_mut();
        let serial = serial.as_mut().unwrap();

        writeln!(serial, "{}", msg).ok();
    })
}

#[embassy::task]
async fn task1() {
    loop {
        Timer::after(Duration::from_millis(50)).await;
        log_interrupt("tick1");
    }
}
#[embassy::task]
async fn task2() {
    loop {
        Timer::after(Duration::from_millis(50)).await;
        log_interrupt("tick2");
    }
}
// #[embassy::task]
// async fn task3(){
//     loop {

//         Timer::after(Duration::from_millis(500)).await;
//         log_interrupt("Medium frequent tick");
//         }
// }

#[embassy::main]
async fn main(spawner: Spawner, _p: Peripherals) {
    let mut serial = Serial::new(_p.UART0).unwrap();
    writeln!(serial, "getting into criticalsection").ok();
    critical_section::with(move |_cs| unsafe {
        SERIAL.get_mut().replace(Some(serial));
    });

    spawner.must_spawn(task1());
    spawner.must_spawn(task2());
    // spawner.must_spawn(task3());
}

//     #[embassy::task]
// async fn __embassy_main(spawner: Spawner, _p: Peripherals){

// }
