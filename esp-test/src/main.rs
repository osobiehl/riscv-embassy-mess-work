#![feature(type_alias_impl_trait)]
#![no_std]
#![no_main]

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
use embassy_esp32c3::{Serial};
use embassy_esp32c3::pac::{UART0, Peripherals};

// use embassy_esp32c3::{}


// #[task]
// async fn run(){
//     static  peripherals: Peripherals = Peripherals::take().unwrap();
//     static serial0: Serial<UART0> = Serial::new(peripherals.UART0).unwrap();
//     writeln!(serial0, "tick!");
//     Timer::after(Duration::from_secs(1)).await;
// }


#[embassy::main]
async fn main(spawner: Spawner, _p: Peripherals) -> ! {
    // Disable watchdog timers
    
    let mut serial0 = Serial::new(_p.UART0).unwrap();

    loop {
        writeln!(serial0, "Hello world!").unwrap();
        Timer::after(Duration::from_micros(1000)).await;
    }
}
