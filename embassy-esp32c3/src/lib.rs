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