

use atomic_polyfill::{compiler_fence, AtomicPtr, Ordering};
use core::mem;
use core::ptr;
// use cortex_m::peripheral::NVIC;

pub use embassy_macros::interrupt_declare as declare;
pub use embassy_macros::interrupt_take as take;

/// Implementation detail, do not use outside embassy crates.
#[doc(hidden)]
pub struct Handler {
    pub func: AtomicPtr<()>,
    pub ctx: AtomicPtr<()>,
}

impl Handler {
    pub const fn new() -> Self {
        Self {
            func: AtomicPtr::new(ptr::null_mut()),
            ctx: AtomicPtr::new(ptr::null_mut()),
        }
    }
}

pub unsafe trait Interrupt: crate::util::Unborrow<Target = Self> {
    type Priority: From<u8> + Into<u8> + Copy;
    fn number(&self) -> isize;
    unsafe fn steal() -> Self;

    /// Implementation detail, do not use outside embassy crates.
    #[doc(hidden)]
    unsafe fn __handler(&self) -> &'static Handler;
}

pub trait InterruptExt: Interrupt {
    fn set_handler(&self, func: unsafe fn(*mut ()));
    fn remove_handler(&self);
    fn set_handler_context(&self, ctx: *mut ());
    fn enable(&self);
    fn disable(&self);
    #[cfg(not(armv6m))]
    fn is_active(&self) -> bool;
    fn is_enabled(&self) -> bool;
    fn is_pending(&self) -> bool;
    fn pend(&self);
    fn unpend(&self);
    fn get_priority(&self) -> Self::Priority;
    fn set_priority(&self, prio: Self::Priority);
}

