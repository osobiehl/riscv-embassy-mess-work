#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]


use core::fmt::Write;

// use esp32c3_hal::{pac::{Peripherals, LEDC, apb_ctrl::peri_backup_config}, prelude::*, RtcCntl, Serial, Timer as old_timer};
use futures::task::Spawn;
use nb::block;
use panic_halt as _;
use riscv_rt::entry;
use embassy;
use embassy::executor::Spawner;
use embassy::time::{Duration, Timer};
// use embassy_macros::{main, task};
use embassy_esp32c3::{Serial, init, timer, rtc_cntl};
use embassy_esp32c3::pac::{UART0, Peripherals};
use embedded_hal::prelude::_embedded_hal_watchdog_WatchdogDisable;
use core::cell::{Cell, RefCell};
use critical_section::CriticalSection;
use embassy::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy::blocking_mutex::CriticalSectionMutex as Mutex;

static mut SERIAL: Mutex<RefCell<Option<Serial<UART0>>>> = Mutex::new(RefCell::new(None));

fn log_interrupt(msg: &str) {
    critical_section::with(|cs| unsafe {
        let mut serial = SERIAL.borrow(cs).borrow_mut();
        let mut serial = serial.as_mut().unwrap();

        writeln!(serial, "{}", msg).ok();
    })
}

#[embassy::task]
async fn run1() {
    loop {
        log_interrupt("BIG INFREQUENT TICK");
        Timer::after(Duration::from_secs(10)).await;
    }
}

#[embassy::task]
async fn run2() {
    loop {
        log_interrupt("frequent tick");
        Timer::after(Duration::from_secs(1)).await;
    }
}




#[embassy::main]
async fn main(spawner: Spawner, _p: Peripherals){
    let mut serial = Serial::new(_p.UART0).unwrap();
    writeln!(serial, "getting into criticalsection").ok();
    critical_section::with(move |_cs| unsafe {
        SERIAL.get_mut().replace(Some(serial));
    });
    spawner.must_spawn(run1());
    spawner.must_spawn(run2());
    // spawner.spawn(run3());
    loop{
        
        Timer::after(Duration::from_secs(5)).await;
        log_interrupt("medium frequent tick");

    }
}

//     #[embassy::task]
// async fn __embassy_main(spawner: Spawner, _p: Peripherals){

// }
