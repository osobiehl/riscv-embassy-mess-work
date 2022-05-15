#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use core::fmt::Write;

// use esp32c3_hal::{pac::{Peripherals, LEDC, apb_ctrl::peri_backup_config}, prelude::*, RtcCntl, Serial, Timer as old_timer};
use embassy;
use panic_halt as _;
// use embassy_macros::{main, task};
use core::cell::{Cell, RefCell};
use core::option::Option::{self, None, Some};
use embassy::blocking_mutex::CriticalSectionMutex as Mutex;
use embassy::interrupt::InterruptExt;
use embassy_esp32c3::config;
use embassy_esp32c3::interrupt::{Priority, SW_INT1};
use embassy_esp32c3::pac::UART0;
use embassy_esp32c3::{init, Serial};
// use embassy_esp32c3::{}

// #[task]
// async fn run(){
//     static  peripherals: Peripherals = Peripher::take().unwrap();
//     static serial0: Serial<UART0> = Serial::new(peripherals.UART0).unwrap();
//     writeln!(serial0, "tick!");
//     Timer::after(Duration::from_secs(1)).await;
// }
unsafe fn __make_static<T>(t: &mut T) -> &'static mut T {
    ::core::mem::transmute(t)
}
static mut SERIAL: Mutex<RefCell<Option<Serial<UART0>>>> = Mutex::new(RefCell::new(None));
static CTR: Mutex<Cell<usize>> = Mutex::new(Cell::new(0));

fn increment_ctr(any: *mut ()) {
    let irq = unsafe { &*(any as *mut SW_INT1) };
    log_interrupt("unpending!");
    irq.unpend();
    critical_section::with(|cs| {
        let ctr = CTR.borrow(cs);
        let count = ctr.get();
        ctr.set(count + 1);
    });
    log_interrupt("software interrupt!");
}
fn log_interrupt(msg: &str) {
    critical_section::with(|cs| unsafe {
        let mut serial = SERIAL.borrow(cs).borrow_mut();
        let serial = serial.as_mut().unwrap();

        writeln!(serial, "{}", msg).ok();
    })
}

#[riscv_rt::entry]
fn main() -> ! {
    let peripherals = init(config::Config::default());

    let mut serial = Serial::new(peripherals.UART0).unwrap();
    writeln!(serial, "initializing enabling interrupt").ok();

    let mut software_interrupt = SW_INT1::new(Priority::Priority4);
    software_interrupt.set_handler(increment_ctr);
    let intr_ptr: *mut SW_INT1 = &mut software_interrupt;
    software_interrupt.set_handler_context(intr_ptr as *mut ());
    software_interrupt.enable();
    //now we place our serial inside a mutex since we want to log from interrupts
    critical_section::with(move |_cs| unsafe {
        SERIAL.get_mut().replace(Some(serial));
    });
    log_interrupt("pending software interrupt, if no output is given, this fails");
    software_interrupt.pend();
    log_interrupt("DONE!");
    software_interrupt.disable();

    log_interrupt("disabling software interrupt, program should not start an interrupt");
    software_interrupt.pend();
    log_interrupt("Done!");

    // _embassy_time_set_alarm_callback(, callback, ctx)

    // peripherals.UART0 = serial.free();
    loop {}
}

//     #[embassy::task]
// async fn __embassy_main(spawner: Spawner, _p: Peripherals){

// }
