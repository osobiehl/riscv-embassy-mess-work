#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use core::fmt::Write;

// use esp32c3_hal::{pac::{Peripherals, LEDC, apb_ctrl::peri_backup_config}, prelude::*, RtcCntl, Serial, Timer as old_timer};
use embassy;
use embassy::executor::Spawner;
use embassy::time::{Duration, Timer};
use panic_halt as _;
// use embassy_macros::{main, task};
use core::cell::RefCell;
use embassy::blocking_mutex::CriticalSectionMutex as Mutex;
use embassy_esp32c3::pac::{Peripherals, UART0};
use embassy_esp32c3::Serial;

static mut SERIAL: Mutex<RefCell<Option<Serial<UART0>>>> = Mutex::new(RefCell::new(None));

fn log_interrupt(msg: &str) {
    critical_section::with(|cs| unsafe {
        let mut serial = SERIAL.borrow(cs).borrow_mut();
        let serial = serial.as_mut().unwrap();

        writeln!(serial, "{}", msg).ok();
    })
}

#[embassy::main]
async fn main(_spawner: Spawner, _p: Peripherals) {
    let serial = Serial::new(_p.UART0).unwrap();
    critical_section::with(move |_cs| unsafe {
        SERIAL.get_mut().replace(Some(serial));
    });

    let async1 = async {
        loop {
            Timer::after(Duration::from_secs(10)).await;
            log_interrupt("BIG INFREQUENT TICK");
        }
    };

    let async2 = async {
        loop {
            log_interrupt("frequent tick");
            Timer::after(Duration::from_secs(1)).await;
        }
    };

    let async3 = async {
        loop {
            Timer::after(Duration::from_secs(5)).await;
            log_interrupt("medium frequent tick");
        }
    };

    futures::join!(async1, async2, async3);
}

//     #[embassy::task]
// async fn __embassy_main(spawner: Spawner, _p: Peripherals){

// }
