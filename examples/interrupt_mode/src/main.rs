//! This example showcases how to create multiple Executor instances to run tasks at
//! different priority levels.
//!
//! Low priority executor runs in thread mode (not interrupt), and uses `sev` for signaling
//! there's work in the queue, and `wfe` for waiting for work.
//!
//! Medium and high priority executors run in two interrupts with different priorities.
//! Signaling work is done by pending the interrupt. No "waiting" needs to be done explicitly, since
//! when there's work the interrupt will trigger and run the executor.
//!
//! Sample output below. Note that high priority ticks can interrupt everything else, and
//! medium priority computations can interrupt low priority computations, making them to appear
//! to take significantly longer time.
//!
//! ```not_rust
//!     [med] Starting long computation
//!     [med] done in 992 ms
//!         [high] tick!
//! [low] Starting long computation
//!     [med] Starting long computation
//!         [high] tick!
//!         [high] tick!
//!     [med] done in 993 ms
//!     [med] Starting long computation
//!         [high] tick!
//!         [high] tick!
//!     [med] done in 993 ms
//! [low] done in 3972 ms
//!     [med] Starting long computation
//!         [high] tick!
//!         [high] tick!
//!     [med] done in 993 ms
//! ```
//!
//! For comparison, try changing the code so all 3 tasks get spawned on the low priority executor.
//! You will get an output like the following. Note that no computation is ever interrupted.
//!
//! ```not_rust
//!         [high] tick!
//!     [med] Starting long computation
//!     [med] done in 496 ms
//! [low] Starting long computation
//! [low] done in 992 ms
//!     [med] Starting long computation
//!     [med] done in 496 ms
//!         [high] tick!
//! [low] Starting long computation
//! [low] done in 992 ms
//!         [high] tick!
//!     [med] Starting long computation
//!     [med] done in 496 ms
//!         [high] tick!
//! ```
//!

#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

// use cortex_m_rt::entry;
// use defmt::{info, unwrap};
use core::fmt::Write;
use embassy::executor::{Executor, InterruptExecutor};
use embassy::time::{Duration, Timer};
use embassy::util::Forever;

// use esp32c3_hal::{pac::{Peripherals, LEDC, apb_ctrl::peri_backup_config}, prelude::*, RtcCntl, Serial, Timer as old_timer};
use embassy;

use panic_halt as _;
// use embassy_macros::{main, task};
use core::cell::RefCell;

use embassy::blocking_mutex::CriticalSectionMutex as Mutex;
use embassy_esp32c3::interrupt::SW_INT1;
use embassy_esp32c3::pac::UART0;
use embassy_esp32c3::Serial;

// use defmt_rtt as _; // global logger
// use panic_probe as _;

fn log_interrupt(msg: &str) {
    critical_section::with(|cs| unsafe {
        let mut serial = SERIAL.borrow(cs).borrow_mut();
        let serial = serial.as_mut().unwrap();

        writeln!(serial, "{}", msg).ok();
    })
}
#[no_mangle]
pub fn __log(msg: &str) {
    log_interrupt(msg);
}

static mut SERIAL: Mutex<RefCell<Option<Serial<UART0>>>> = Mutex::new(RefCell::new(None));

#[embassy::task]
async fn run_high() {
    loop {
        log_interrupt("high priority interrupt preempts computation!");
        Timer::after(Duration::from_secs(3)).await;
    }
}

#[embassy::task]
async fn run_low() {
    // let mut delay = McycleDelay::new(20_000_000);
    loop {
        log_interrupt("    [low] Starting long computation");

        // Spin-wait to simulate a long CPU computation
        let mut ctr = 0;
        let bignum = 0x2ffffff;
        while ctr < bignum {
            unsafe { riscv::asm::nop() }
            ctr += 1;
        }
        log_interrupt("    [low] finished long computation");

        Timer::after(Duration::from_millis(20)).await;
    }
}

static EXECUTOR_HIGH: Forever<InterruptExecutor<SW_INT1>> = Forever::new();

static EXECUTOR_LOW: Forever<Executor> = Forever::new();

#[riscv_rt::entry]
fn main() -> ! {
    let _p = embassy_esp32c3::init(Default::default());
    let mut serial = Serial::new(_p.UART0).unwrap();
    writeln!(
        serial,
        "if you flash in release mode, the compiler might remove the work simulation"
    )
    .ok();
    critical_section::with(move |_cs| unsafe {
        SERIAL.get_mut().replace(Some(serial));
    });

    // High-priority executor: SWI1_EGU1, priority level 6
    //highest priority
    log_interrupt("setting interrupt executor");
    let int_executor = SW_INT1::new(embassy_esp32c3::interrupt::Priority::Priority15);
    let executor = EXECUTOR_HIGH.put(InterruptExecutor::new(int_executor));
    log_interrupt("spawning interrupt executor task");
    executor.start(|spawner| match spawner.spawn(run_high()) {
        Ok(_) => log_interrupt("task successfully spawned!"),
        Err(_) => log_interrupt("something went wrong!"),
    });
    log_interrupt("spawning thread mode executor");

    // Low priority executor: runs in thread mode, using WFE/SEV
    let executor = EXECUTOR_LOW.put(Executor::new());
    executor.run(|spawner| {
        spawner.spawn(run_low()).ok();
    });
}
