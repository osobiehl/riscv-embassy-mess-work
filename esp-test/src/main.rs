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
use embassy_esp32c3::{Serial, init};
use embassy_esp32c3::pac::{UART0, Peripherals};

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
        let peripherals = init();

        let mut executor = embassy::executor::Executor::new();
        let executor = unsafe { __make_static(&mut executor) };

        executor.run(|spawner| {
            spawner.must_spawn(__embassy_main(spawner, peripherals));
        })
    }



fn __embassy_main(spawner: Spawner, _p: Peripherals) -> embassy::executor::SpawnToken<impl ::core::future::Future + 'static>
{
    use embassy::executor::raw::TaskStorage;
    async fn task(spawner: Spawner, _p: Peripherals){
    // Disable watchdog timers
        
        let mut serial0 = Serial::new(_p.UART0).unwrap();

        loop {
            writeln!(serial0, "Hello world!").unwrap();
            Timer::after(Duration::from_micros(1000)).await;
        }
    }
    type F = impl ::core::future::Future + 'static;
    #[allow(clippy::declare_interior_mutable_const)]
    const NEW_TASK: TaskStorage<F> = TaskStorage::new();
    static POOL: [TaskStorage<F>; 1] = [NEW_TASK; 1];
    unsafe { TaskStorage::spawn_pool(&POOL, move || task(spawner, _p)) }
}
//     #[embassy::task]
// async fn __embassy_main(spawner: Spawner, _p: Peripherals){

// }
