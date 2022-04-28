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



#[embassy::main]
async fn main(spawner: Spawner, _p: Peripherals){
    let mut serial0 = Serial::new(_p.UART0).unwrap();
    loop {
        writeln!(serial0, "Hello world!").unwrap();
        Timer::after(Duration::from_micros(1000_000)).await;
    }
}

//     #[embassy::task]
// async fn __embassy_main(spawner: Spawner, _p: Peripherals){

// }
