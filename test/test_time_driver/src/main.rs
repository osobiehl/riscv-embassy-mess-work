#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use core::fmt::Write;

// use esp32c3_hal::{pac::{Peripherals, LEDC, apb_ctrl::peri_backup_config}, prelude::*, RtcCntl, Serial, Timer as old_timer};
use embassy;
use embassy::time::driver::AlarmHandle;

use panic_halt as _;

// use embassy_macros::{main, task};

use embassy::blocking_mutex::CriticalSectionMutex as Mutex;
use embassy_esp32c3::pac::UART0;
use embassy_esp32c3::{init, Serial};

use core::cell::{Cell, RefCell};
use core::option::Option::{self, None, Some};
use embassy_esp32c3::config;
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
static mut SERIAL: Mutex<RefCell<Option<Serial<UART0>>>> = Mutex::new(RefCell::new(None));
static CTR: Mutex<Cell<usize>> = Mutex::new(Cell::new(0));
fn increment_ctr(any: *mut ()) {
    let v: usize = any as usize;
    let res: &str = match v {
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
        let serial = serial.as_mut().unwrap();

        writeln!(serial, "{}", msg).ok();
    })
}
fn compare_ctr(v: usize, on_fail: &str) {
    critical_section::with(|cs| unsafe {
        let count = CTR.borrow(cs).get();
        if count != v {
            let mut serial = SERIAL.borrow(cs).borrow_mut();
            let serial = serial.as_mut().unwrap();
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
        writeln!(serial, "single alarm callback test").ok();

        //now we place our serial inside a mutex since we want to log from interrupts
        critical_section::with(move |_cs| {
            SERIAL.get_mut().replace(Some(serial));
        });

        _embassy_time_set_alarm_callback(alarm_1, increment_ctr, 1 as usize as *mut ());
        log_interrupt("getting current time, failure if hangs\n");
        let now = _embassy_time_now();

        let to_expire = now + 30_000_000u64;
        log_interrupt("setting alarm timer in 30_000_000 counts, \nif this hangs, tests have failed\nif this loops, tests have also failed\n");
        _embassy_time_set_alarm(alarm_1, to_expire);
        riscv::asm::wfi();
        compare_ctr(1, "FAIL: interrupt did not properly execute");
        log_interrupt("alarm deallocation leaves frees alarm slots\n");

        log_interrupt("can re-allocate a used timer\n");
        _embassy_time_set_alarm_callback(alarm_1, increment_ctr, 1 as usize as *mut ());

        let now = _embassy_time_now();
        let to_expire = now + 30_000_000u64;

        log_interrupt("setting alarm timer in 30_000_000 counts, \nif this hangs, tests have failed\nif this loops, tests have also failed\n");
        _embassy_time_set_alarm(alarm_1, to_expire);
        riscv::asm::wfi();
        compare_ctr(2, "FAIL: interrupt did not properly execute");

        log_interrupt("interrupt for passed time alarms triggers instantly\n");

        _embassy_time_set_alarm_callback(alarm_1, increment_ctr, 1 as usize as *mut ());
        _embassy_time_set_alarm(alarm_1, 0);
        //riscv::asm::wfi();
        compare_ctr(3, "FAIL: interrupt did not execute immediately");

        log_interrupt("can trigger alarms sequentially, fails if hangs\n");
        let mut alarms = [&alarm_1, &alarm_2, &alarm_3];
        for a in alarms {
            _embassy_time_set_alarm_callback(*a, increment_ctr, (a.id() + 1) as usize as *mut ())
        }

        for a in alarms {
            let now = _embassy_time_now();
            _embassy_time_set_alarm(*a, now + 30_000_000u64);
            riscv::asm::wfi();
        }
        compare_ctr(6, "FAIL: ctr value is incorrect!");
        log_interrupt("can set multiple alarms concurrently\n");

        alarms.swap(0, 2);
        for a in alarms.iter() {
            _embassy_time_set_alarm_callback(**a, increment_ctr, (a.id() + 1) as usize as *mut ());
        }
        let now = _embassy_time_now();

        for a in alarms.iter() {
            _embassy_time_set_alarm(**a, now + ((a.id() + 1) as u64 * 30_000_000u64));
        }

        riscv::asm::wfi();
        riscv::asm::wfi();
        riscv::asm::wfi();
        compare_ctr(9, "FAIL: ctr value is incorrect!");

        log_interrupt("can set multiple alarms for very close time\n");

        for a in alarms.iter() {
            _embassy_time_set_alarm_callback(**a, increment_ctr, (a.id() + 1) as usize as *mut ());
        }
        let now = _embassy_time_now();

        for a in alarms.iter() {
            _embassy_time_set_alarm(**a, now + 60_000_000u64);
        }
        //interrupts should immediately trigger after the other, so only 1 WFI call should be done
        riscv::asm::wfi();
        compare_ctr(12, "FAIL: ctr value is incorrect!");

        log_interrupt("DONE!")

        // _embassy_time_set_alarm_callback(, callback, ctx)
    }

    // peripherals.UART0 = serial.free();
    loop {}
}

//     #[embassy::task]
// async fn __embassy_main(spawner: Spawner, _p: Peripherals){

// }
