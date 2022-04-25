#![no_std]
use esp32c3::Peripherals;
// use esp32;
pub mod serial;
pub use serial::Serial;
pub mod timer;
pub mod gpio;
pub mod driver;
pub mod rtc_cntl;
pub mod systimer;
pub mod interrupt {
    pub use esp32c3::Interrupt as interrupt_source;

    pub use embassy::interrupt::{
        declare, take, Interrupt, CpuInterrupt, Priority, ESP32C3_Interrupts, InterruptKind};
    // pub use embassy_hal_common::interrupt::Priority3 as Priority;
}
pub mod pac{
    pub use esp32c3::*;
}
use embedded_hal::prelude::_embedded_hal_watchdog_WatchdogDisable;
pub fn init()->Peripherals{
    //steal peripherals to 
    unsafe{    
        let peripherals = Peripherals::steal();
        let mut rtc_cntl = rtc_cntl::RtcCntl::new(peripherals.RTC_CNTL);
        let mut timer0 = timer::Timer::new(peripherals.TIMG0);
        let mut timer1 = timer::Timer::new(peripherals.TIMG1);
        timer0.disable();
        timer1.disable();
        rtc_cntl.set_super_wdt_enable(false);
        rtc_cntl.set_wdt_enable(false);
        timer1.free();
        timer0.free();
    }
    Peripherals::take().unwrap()

}