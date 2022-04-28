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





#[embassy::main]
async fn main(spawner: Spawner, _p: Peripherals){
    let mut serial = Serial::new(_p.UART0).unwrap();
    writeln!(serial, "getting into criticalsection").ok();
    critical_section::with(move |_cs| unsafe {
        SERIAL.get_mut().replace(Some(serial));
    });

    // spawner.spawn(run3());

    let t1 = async {
        loop {
           
            Timer::after(Duration::from_secs(1)).await;
            log_interrupt("frequent tick");
        }
    };
    let t2 = async {    loop {
        
        Timer::after(Duration::from_secs(10)).await;
        log_interrupt("BIG INFREQUENT TICK");
        }
    };
    let _one_time_async = async {
        Timer::after(Duration::from_secs(3)).await;
        log_interrupt("occurring once before join!");
    }.await;

    let one_time_join_async = async {
        Timer::after(Duration::from_secs(3)).await;
        log_interrupt("occurring once in join!");
    };
    log_interrupt("started join!");

    futures::join!(t1,t2, one_time_join_async);
}

//     #[embassy::task]
// async fn __embassy_main(spawner: Spawner, _p: Peripherals){

// }
