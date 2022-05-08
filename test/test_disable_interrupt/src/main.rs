#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use core::fmt::Write;

// use esp32c3_hal::{pac::{Peripherals, LEDC, apb_ctrl::peri_backup_config}, prelude::*, RtcCntl, Serial, Timer as old_timer};
use embassy;
use embassy::executor::Spawner;
use embassy::time::driver::{AlarmHandle, Driver};
use embassy::time::{Duration, Timer};
use futures::task::Spawn;
use nb::block;
use panic_halt as _;
use riscv_rt::entry;
// use embassy_macros::{main, task};
use critical_section::CriticalSection;
use embassy::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy::blocking_mutex::CriticalSectionMutex as Mutex;
use embassy_esp32c3::pac::{Peripherals, UART0};
use embassy_esp32c3::{init, rtc_cntl, systimer, timer, Serial};
use embedded_hal::prelude::_embedded_hal_watchdog_WatchdogDisable;

use core::cell::{Cell, RefCell};
use core::mem::transmute;
use core::option::Option::{self, None, Some};
use embassy_esp32c3::config;
use embassy_esp32c3::driver::{AlarmState, SysTimerDriver};
use embassy_esp32c3::interrupt::Priority;
// use embassy_esp32c3::{}
use riscv;

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
static mut SERIAL: Mutex<RefCell<Option<Serial<UART0>>>> = Mutex::new(RefCell::new(None));
static CTR: Mutex<Cell<usize>> = Mutex::new(Cell::new(0));

const ALARM_STATE_NONE: AlarmState = AlarmState::new();
const ALARM_COUNT: usize = 3;
fn increment_ctr(any: *mut ()) {
    let v: usize = any as usize;
    let res: &str = match (v) {
        1 => "interrupt 1!",
        2 => "interrupt 2!",
        3 => "interrupt 3!",
        _ => "unknown!",
    };

    critical_section::with(|cs| {
        let ctr = CTR.borrow(cs);
        let count = ctr.get();
        ctr.set(count + 1);
    });
    log_interrupt(res);
}
fn log_interrupt(msg: &str) {
    critical_section::with(|cs| unsafe {
        let mut serial = SERIAL.borrow(cs).borrow_mut();
        let mut serial = serial.as_mut().unwrap();

        writeln!(serial, "{}", msg).ok();
    })
}
fn compare_ctr(v: usize, on_fail: &str) {
    critical_section::with(|cs| unsafe {
        let count = CTR.borrow(cs).get();
        if count != v {
            let mut serial = SERIAL.borrow(cs).borrow_mut();
            let mut serial = serial.as_mut().unwrap();
            writeln!(serial, "{}", on_fail).ok();
        }
    })
}

extern "Rust" {
    fn _embassy_time_now() -> u64;
    fn _embassy_time_allocate_alarm() -> Option<AlarmHandle>;
    fn _embassy_time_set_alarm_callback(alarm: AlarmHandle, callback: fn(*mut ()), ctx: *mut ());
    fn _embassy_time_set_alarm(alarm: AlarmHandle, timestamp: u64);
}
#[riscv_rt::entry]
fn main() -> ! {
    let peripherals = init(config::Config::default());

    let mut serial = Serial::new(peripherals.UART0).unwrap();
    writeln!(serial, "initializing driver").ok();
    unsafe {
        writeln!(serial, "allocate alarm handles").ok();
        let alarm_1 = _embassy_time_allocate_alarm();
        if alarm_1.is_none() {
            writeln!(serial, "FAIL: alarm handle 1 not returned").ok();
        }
        let alarm_1 = alarm_1.unwrap();
        let alarm_2 = _embassy_time_allocate_alarm();
        if alarm_2.is_none() {
            writeln!(serial, "FAIL: alarm handle 2 not returned").ok();
        }
        let alarm_2 = alarm_2.unwrap();
        let alarm_3 = _embassy_time_allocate_alarm();
        if alarm_3.is_none() {
            writeln!(serial, "FAIL: alarm handle 3 not returned").ok();
        }
        let alarm_3 = alarm_3.unwrap();
        let alarm_fail = _embassy_time_allocate_alarm();
        if alarm_fail.is_some() {
            writeln!(serial, "FAIL: fourth alarm handle should not be allocated!").ok();
        }

        //now we place our serial inside a mutex since we want to log from interrupts
        critical_section::with(move |_cs| unsafe {
            SERIAL.get_mut().replace(Some(serial));
        });

        _embassy_time_set_alarm_callback(alarm_1, increment_ctr, 1 as usize as *mut ());
        let mut now = _embassy_time_now();

        let to_expire = now + 30_000_000u64;
        log_interrupt("disabling interrupts after setting an interrupt");
        _embassy_time_set_alarm(alarm_1, to_expire);

        unsafe { riscv::interrupt::disable() }
        riscv::asm::wfi();
        log_interrupt("an interrupt occurred, but it's not getting serviced yet!");
        riscv::interrupt::enable();

        log_interrupt(
            "testing what happens if an interrupt is received inside the critical section 
but not yet reaching the WFI section",
        );

        let mut now = _embassy_time_now();

        let to_expire = now + 15_000_000u64;
        log_interrupt("disabling interrupts after setting an interrupt");
        _embassy_time_set_alarm(alarm_1, to_expire);

        critical_section::with( |_| unsafe {
            let bignum = 0x2ffffff;
            let mut ctr = 0;
            log_interrupt("busy waiting so interrupt triggers before reaching WFI");
            while(ctr < bignum){
                unsafe{riscv::asm::nop()}
                ctr+=1;
            }
            log_interrupt("done doing work! now calling WFI");
            riscv::asm::wfi();
            log_interrupt("interrupt occurred, this should happen before servicing...");
        });
        log_interrupt("this should happen after servicing...");

        // _embassy_time_set_alarm_callback(, callback, ctx)
    }

    // peripherals.UART0 = serial.free();
    loop {}
}

//     #[embassy::task]
// async fn __embassy_main(spawner: Spawner, _p: Peripherals){

// }
