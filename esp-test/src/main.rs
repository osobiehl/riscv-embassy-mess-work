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


// use embassy_esp32c3::{}


// #[task]
// async fn run(){
//     static  peripherals: Peripherals = Peripherals::take().unwrap();
//     static serial0: Serial<UART0> = Serial::new(peripherals.UART0).unwrap();
//     writeln!(serial0, "tick!");
//     Timer::after(Duration::from_secs(1)).await;
// }
unsafe fn __make_static<T>(t: &mut T) -> &'static mut T {
    ::core::mem::transmute(t)
}
#[riscv_rt::entry]
    fn main() -> ! {
        let mut peripherals = init();

        let mut serial = Serial::new(peripherals.UART0).unwrap();
        writeln!(serial, "calling executor.run").ok();

        peripherals.UART0 = serial.free();


        let mut s0 = Serial::new(peripherals.UART0).unwrap();
        writeln!(s0, "borrow checking").ok();
        

        let mut executor = embassy::executor::Executor::new();
        writeln!(s0, "allocated executor!").ok();
        let executor = unsafe { __make_static(&mut executor) };
        


        peripherals.UART0 = s0.free();
        executor.run(|spawner| {
            spawner.must_spawn(__embassy_main(spawner, peripherals));
        })
    }

#[embassy::task]
async fn __embassy_main(spawner: Spawner, _p: Peripherals)
{

        
    let mut serial0 = Serial::new(_p.UART0).unwrap();
    loop {
        writeln!(serial0, "Hello world!").unwrap();
        Timer::after(Duration::from_micros(1000_000)).await;
    }
}

//     #[embassy::task]
// async fn __embassy_main(spawner: Spawner, _p: Peripherals){

// }
