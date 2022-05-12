#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
use core::fmt::Write;
use panic_halt as _;
use embassy::{executor::Spawner, time::{Duration, Timer}};
use embassy_esp32c3::{pac::Peripherals, Serial};
#[embassy::main]
async fn main(_spawner: Spawner, _p: Peripherals){
    let mut serial0 = Serial::new(_p.UART0).unwrap();
    loop {
        writeln!(serial0, "Hello world!").unwrap();
        Timer::after(Duration::from_millis(1000)).await;
    }
}

//     #[embassy::task]
// async fn __embassy_main(spawner: Spawner, _p: Peripherals){

// }
