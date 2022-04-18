use esp32c3_pac::{pac::Interrupt, Cpu};
extern "Rust" {
    static handler1: Handler;
    static handler2: Handler;
    static handler3: Handler;
    static handler4: Handler;
    static handler5: Handler;
    static handler6: Handler;
    static handler7: Handler;
    static handler8: Handler;
    static handler9: Handler;
    static handler10: Handler;
    static handler11: Handler;
    static handler12: Handler;
    static handler13: Handler;
    static handler14: Handler;
    static handler15: Handler;
    static handler16: Handler;
    static handler17: Handler;
    static handler18: Handler;
    static handler19: Handler;
    static handler20: Handler;
    static handler21: Handler;
    static handler22: Handler;
    static handler23: Handler;
    static handler24: Handler;
    static handler25: Handler;
    static handler26: Handler;
    static handler27: Handler;
    static handler28: Handler;
    static handler29: Handler;
    static handler30: Handler;
    static handler31: Handler;
}

/// Interrupt kind
pub enum InterruptKind {
    /// Level interrupt
    Level,
    /// Edge interrupt
    Edge,
}

/// Enumeration of available CPU interrupts.
/// It is possible to create a handler for each of the interrupts. (e.g.
/// `interrupt3`)
pub enum CpuInterrupt {
    Interrupt1 = 1,
    Interrupt2,
    Interrupt3,
    Interrupt4,
    Interrupt5,
    Interrupt6,
    Interrupt7,
    Interrupt8,
    Interrupt9,
    Interrupt10,
    Interrupt11,
    Interrupt12,
    Interrupt13,
    Interrupt14,
    Interrupt15,
    Interrupt16,
    Interrupt17,
    Interrupt18,
    Interrupt19,
    Interrupt20,
    Interrupt21,
    Interrupt22,
    Interrupt23,
    Interrupt24,
    Interrupt25,
    Interrupt26,
    Interrupt27,
    Interrupt28,
    Interrupt29,
    Interrupt30,
    Interrupt31,
}

/// Interrupt priority levels.
pub enum Priority {
    None,
    Priority1,
    Priority2,
    Priority3,
    Priority4,
    Priority5,
    Priority6,
    Priority7,
    Priority8,
    Priority9,
    Priority10,
    Priority11,
    Priority12,
    Priority13,
    Priority14,
    Priority15,
}

/// Enable and assign a peripheral interrupt to an CPU interrupt.
pub fn enable(_core: Cpu, interrupt: Interrupt, which: CpuInterrupt) {
    unsafe {
        let interrupt_number = interrupt as isize;
        let cpu_interrupt_number = which as isize;
        let intr = &*crate::pac::INTERRUPT_CORE0::ptr();
        let intr_map_base = intr.mac_intr_map.as_ptr();
        intr_map_base
            .offset(interrupt_number)
            .write_volatile(cpu_interrupt_number as u32);
        // enable interrupt
        intr.cpu_int_enable
            .modify(|r, w| w.bits((1 << cpu_interrupt_number) | r.bits()));
    }
}

/// Disable the given peripheral interrupt.
pub fn disable(_core: Cpu, interrupt: Interrupt) {
    unsafe {
        let interrupt_number = interrupt as isize;
        let intr = &*crate::pac::INTERRUPT_CORE0::ptr();
        let intr_map_base = intr.mac_intr_map.as_ptr();
        intr_map_base.offset(interrupt_number).write_volatile(0);
    }
}

/// Set the interrupt kind (i.e. level or edge) of an CPU interrupt
pub fn set_kind(_core: Cpu, which: CpuInterrupt, kind: InterruptKind) {
    unsafe {
        let intr = &*crate::pac::INTERRUPT_CORE0::ptr();
        let cpu_interrupt_number = which as isize;

        let interrupt_type = match kind {
            InterruptKind::Level => 0,
            InterruptKind::Edge => 1,
        };
        intr.cpu_int_type.modify(|r, w| {
            w.bits(
                r.bits() & !(1 << cpu_interrupt_number) | (interrupt_type << cpu_interrupt_number),
            )
        });
    }
}

/// Set the priority level of an CPU interrupt
pub fn set_priority(_core: Cpu, which: CpuInterrupt, priority: Priority) {
    unsafe {
        let intr = &*crate::pac::INTERRUPT_CORE0::ptr();
        let cpu_interrupt_number = which as isize;
        let intr_prio_base = intr.cpu_int_pri_0.as_ptr();

        intr_prio_base
            .offset(cpu_interrupt_number as isize)
            .write_volatile(priority as u32);
    }
}

/// Clear a CPU interrupt
pub fn clear(_core: Cpu, which: CpuInterrupt) {
    unsafe {
        let cpu_interrupt_number = which as isize;
        let intr = &*crate::pac::INTERRUPT_CORE0::ptr();
        intr.cpu_int_clear
            .write(|w| w.bits(1 << cpu_interrupt_number));
    }
}

/// Get status of peripheral interrupts
pub fn get_status(_core: Cpu) -> u128 {
    unsafe {
        ((*crate::pac::INTERRUPT_CORE0::ptr())
            .intr_status_reg_0
            .read()
            .bits() as u128)
            | ((*crate::pac::INTERRUPT_CORE0::ptr())
                .intr_status_reg_1
                .read()
                .bits() as u128)
                << 32
    }
}

/// Registers saved in trap handler
#[doc(hidden)]
#[allow(missing_docs)]
#[derive(Debug, Default, Clone, Copy)]
#[repr(C)]
pub struct TrapFrame {
    pub ra: usize,
    pub t0: usize,
    pub t1: usize,
    pub t2: usize,
    pub t3: usize,
    pub t4: usize,
    pub t5: usize,
    pub t6: usize,
    pub a0: usize,
    pub a1: usize,
    pub a2: usize,
    pub a3: usize,
    pub a4: usize,
    pub a5: usize,
    pub a6: usize,
    pub a7: usize,
    pub s0: usize,
    pub s1: usize,
    pub s2: usize,
    pub s3: usize,
    pub s4: usize,
    pub s5: usize,
    pub s6: usize,
    pub s7: usize,
    pub s8: usize,
    pub s9: usize,
    pub s10: usize,
    pub s11: usize,
    pub gp: usize,
    pub tp: usize,
    pub sp: usize,
}

/// # Safety
///
/// This function is called from an assembly trap handler.
#[doc(hidden)]
#[link_section = ".trap.rust"]
#[export_name = "_start_trap_rust_hal"]
pub unsafe extern "C" fn start_trap_rust_hal(trap_frame: *mut TrapFrame) {
    extern "C" {
        pub fn DefaultHandler();
    }

    let cause = mcause::read();
    if cause.is_exception() {
        handle_exception(trap_frame);
    } else {
        let code = riscv::register::mcause::read().code();
        match code {
            1 => {
                let func = handler1.func as unsafe fn(*mut ());
                let arg: *mut () = handler1.ctx as *mut ();
                func(arg);
            }
            2 => {
                let func = handler2.func as unsafe fn(*mut ());
                let arg: *mut () = handler2.ctx as *mut ();
                func(arg);
            }
            3 => {
                let func = handler3.func as unsafe fn(*mut ());
                let arg: *mut () = handler3.ctx as *mut ();
                func(arg);
            }
            4 => {
                let func = handler4.func as unsafe fn(*mut ());
                let arg: *mut () = handler4.ctx as *mut ();
                func(arg);
            }
            5 => {
                let func = handler5.func as unsafe fn(*mut ());
                let arg: *mut () = handler5.ctx as *mut ();
                func(arg);
            }
            6 => {
                let func = handler6.func as unsafe fn(*mut ());
                let arg: *mut () = handler6.ctx as *mut ();
                func(arg);
            }
            7 => {
                let func = handler7.func as unsafe fn(*mut ());
                let arg: *mut () = handler7.ctx as *mut ();
                func(arg);
            }
            8 => {
                let func = handler8.func as unsafe fn(*mut ());
                let arg: *mut () = handler8.ctx as *mut ();
                func(arg);
            }
            9 => {
                let func = handler9.func as unsafe fn(*mut ());
                let arg: *mut () = handler9.ctx as *mut ();
                func(arg);
            }
            10 => {
                let func = handler10.func as unsafe fn(*mut ());
                let arg: *mut () = handler10.ctx as *mut ();
                func(arg);
            }
            11 => {
                let func = handler11.func as unsafe fn(*mut ());
                let arg: *mut () = handler11.ctx as *mut ();
                func(arg);
            }
            12 => {
                let func = handler12.func as unsafe fn(*mut ());
                let arg: *mut () = handler12.ctx as *mut ();
                func(arg);
            }
            13 => {
                let func = handler13.func as unsafe fn(*mut ());
                let arg: *mut () = handler13.ctx as *mut ();
                func(arg);
            }
            14 => {
                let func = handler14.func as unsafe fn(*mut ());
                let arg: *mut () = handler14.ctx as *mut ();
                func(arg);
            }
            16 => {
                let func = handler16.func as unsafe fn(*mut ());
                let arg: *mut () = handler16.ctx as *mut ();
                func(arg);
            }
            15 => {
                let func = handler15.func as unsafe fn(*mut ());
                let arg: *mut () = handler15.ctx as *mut ();
                func(arg);
            }
            17 => {
                let func = handler17.func as unsafe fn(*mut ());
                let arg: *mut () = handler17.ctx as *mut ();
                func(arg);
            }
            18 => {
                let func = handler18.func as unsafe fn(*mut ());
                let arg: *mut () = handler18.ctx as *mut ();
                func(arg);
            }
            19 => {
                let func = handler19.func as unsafe fn(*mut ());
                let arg: *mut () = handler19.ctx as *mut ();
                func(arg);
            }
            20 => {
                let func = handler20.func as unsafe fn(*mut ());
                let arg: *mut () = handler20.ctx as *mut ();
                func(arg);
            }
            21 => {
                let func = handler21.func as unsafe fn(*mut ());
                let arg: *mut () = handler21.ctx as *mut ();
                func(arg);
            }
            22 => {
                let func = handler22.func as unsafe fn(*mut ());
                let arg: *mut () = handler22.ctx as *mut ();
                func(arg);
            }
            23 => {
                let func = handler23.func as unsafe fn(*mut ());
                let arg: *mut () = handler23.ctx as *mut ();
                func(arg);
            }
            24 => {
                let func = handler24.func as unsafe fn(*mut ());
                let arg: *mut () = handler24.ctx as *mut ();
                func(arg);
            }
            25 => {
                let func = handler25.func as unsafe fn(*mut ());
                let arg: *mut () = handler25.ctx as *mut ();
                func(arg);
            }
            26 => {
                let func = handler26.func as unsafe fn(*mut ());
                let arg: *mut () = handler26.ctx as *mut ();
                func(arg);
            }
            27 => {
                let func = handler27.func as unsafe fn(*mut ());
                let arg: *mut () = handler27.ctx as *mut ();
                func(arg);
            }
            28 => {
                let func = handler28.func as unsafe fn(*mut ());
                let arg: *mut () = handler28.ctx as *mut ();
                func(arg);
            }
            29 => {
                let func = handler29.func as unsafe fn(*mut ());
                let arg: *mut () = handler29.ctx as *mut ();
                func(arg);
            }
            30 => {
                let func = handler30.func as unsafe fn(*mut ());
                let arg: *mut () = handler30.ctx as *mut ();
                func(arg);
            }
            31 => {
                let func = handler31.func as unsafe fn(*mut ());
                let arg: *mut () = handler31.ctx as *mut ();
                func(arg);
            }
            _ => DefaultHandler(),
        };
    }
}

/// Apply atomic emulation if needed. Call the default exception handler
/// otherwise.
///
/// # Safety
///
/// This function is called from an trap handler.
#[doc(hidden)]
unsafe fn handle_exception(trap_frame: *mut TrapFrame) {
    extern "C" {
        pub fn _start_trap_atomic_rust(trap_frame: *mut riscv_atomic_emulation_trap::TrapFrame);
    }

    let mut atomic_emulation_trap_frame = riscv_atomic_emulation_trap::TrapFrame {
        pc: riscv::register::mepc::read(),
        ra: (*trap_frame).ra,
        sp: (*trap_frame).sp,
        gp: (*trap_frame).gp,
        tp: (*trap_frame).tp,
        t0: (*trap_frame).t0,
        t1: (*trap_frame).t1,
        t2: (*trap_frame).t2,
        fp: (*trap_frame).s0,
        s1: (*trap_frame).s1,
        a0: (*trap_frame).a0,
        a1: (*trap_frame).a1,
        a2: (*trap_frame).a2,
        a3: (*trap_frame).a3,
        a4: (*trap_frame).a4,
        a5: (*trap_frame).a5,
        a6: (*trap_frame).a6,
        a7: (*trap_frame).a7,
        s2: (*trap_frame).s2,
        s3: (*trap_frame).s3,
        s4: (*trap_frame).s4,
        s5: (*trap_frame).s5,
        s6: (*trap_frame).s6,
        s7: (*trap_frame).s7,
        s8: (*trap_frame).s8,
        s9: (*trap_frame).s9,
        s10: (*trap_frame).s10,
        s11: (*trap_frame).s11,
        t3: (*trap_frame).t3,
        t4: (*trap_frame).t4,
        t5: (*trap_frame).t5,
        t6: (*trap_frame).t6,
    };

    _start_trap_atomic_rust(&mut atomic_emulation_trap_frame);

    riscv::register::mepc::write(atomic_emulation_trap_frame.pc);
    (*trap_frame).ra = atomic_emulation_trap_frame.ra;
    (*trap_frame).sp = atomic_emulation_trap_frame.sp;
    (*trap_frame).gp = atomic_emulation_trap_frame.gp;
    (*trap_frame).tp = atomic_emulation_trap_frame.tp;
    (*trap_frame).t0 = atomic_emulation_trap_frame.t0;
    (*trap_frame).t1 = atomic_emulation_trap_frame.t1;
    (*trap_frame).t2 = atomic_emulation_trap_frame.t2;
    (*trap_frame).s0 = atomic_emulation_trap_frame.fp;
    (*trap_frame).s1 = atomic_emulation_trap_frame.s1;
    (*trap_frame).a0 = atomic_emulation_trap_frame.a0;
    (*trap_frame).a1 = atomic_emulation_trap_frame.a1;
    (*trap_frame).a2 = atomic_emulation_trap_frame.a2;
    (*trap_frame).a3 = atomic_emulation_trap_frame.a3;
    (*trap_frame).a4 = atomic_emulation_trap_frame.a4;
    (*trap_frame).a5 = atomic_emulation_trap_frame.a5;
    (*trap_frame).a6 = atomic_emulation_trap_frame.a6;
    (*trap_frame).a7 = atomic_emulation_trap_frame.a7;
    (*trap_frame).s2 = atomic_emulation_trap_frame.s2;
    (*trap_frame).s3 = atomic_emulation_trap_frame.s3;
    (*trap_frame).s4 = atomic_emulation_trap_frame.s4;
    (*trap_frame).s5 = atomic_emulation_trap_frame.s5;
    (*trap_frame).s6 = atomic_emulation_trap_frame.s6;
    (*trap_frame).s7 = atomic_emulation_trap_frame.s7;
    (*trap_frame).s8 = atomic_emulation_trap_frame.s8;
    (*trap_frame).s9 = atomic_emulation_trap_frame.s9;
    (*trap_frame).s10 = atomic_emulation_trap_frame.s10;
    (*trap_frame).s11 = atomic_emulation_trap_frame.s11;
    (*trap_frame).t3 = atomic_emulation_trap_frame.t3;
    (*trap_frame).t4 = atomic_emulation_trap_frame.t4;
    (*trap_frame).t5 = atomic_emulation_trap_frame.t5;
    (*trap_frame).t6 = atomic_emulation_trap_frame.t6;
}

#[doc(hidden)]
#[no_mangle]
pub fn _setup_interrupts() {
    extern "C" {
        static _vector_table_hal: *const u32;
    }

    unsafe {
        let vec_table = &_vector_table_hal as *const _ as usize;
        riscv::register::mtvec::write(vec_table, riscv::register::mtvec::TrapMode::Vectored);
    };
}




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

#[derive(Clone, Copy)]
pub(crate) struct NrWrap(pub(crate) u16);
unsafe impl cortex_m::interrupt::InterruptNumber for NrWrap {
    fn number(self) -> u16 {
        self.0
    }
}

pub unsafe trait Interrupt: crate::util::Unborrow<Target = Self> {
    type Priority: From<u8> + Into<u8> + Copy;
    fn number(&self) -> u16;
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

impl<T: Interrupt + ?Sized> InterruptExt for T {
    fn set_handler(&self, func: unsafe fn(*mut ())) {
        compiler_fence(Ordering::SeqCst);
        let handler = unsafe { self.__handler() };
        handler.func.store(func as *mut (), Ordering::Relaxed);
        compiler_fence(Ordering::SeqCst);
    }

    fn remove_handler(&self) {
        compiler_fence(Ordering::SeqCst);
        let handler = unsafe { self.__handler() };
        handler.func.store(ptr::null_mut(), Ordering::Relaxed);
        compiler_fence(Ordering::SeqCst);
    }

    fn set_handler_context(&self, ctx: *mut ()) {
        let handler = unsafe { self.__handler() };
        handler.ctx.store(ctx, Ordering::Relaxed);
    }

    #[inline]
    fn enable(&self) {
        compiler_fence(Ordering::SeqCst);
        unsafe {
            NVIC::unmask(NrWrap(self.number()));
        }
    }

    #[inline]
    fn disable(&self) {
        NVIC::mask(NrWrap(self.number()));
        compiler_fence(Ordering::SeqCst);
    }

    #[inline]
    #[cfg(not(armv6m))]
    fn is_active(&self) -> bool {
        NVIC::is_active(NrWrap(self.number()))
    }

    #[inline]
    fn is_enabled(&self) -> bool {
        NVIC::is_enabled(NrWrap(self.number()))
    }

    #[inline]
    fn is_pending(&self) -> bool {
        NVIC::is_pending(NrWrap(self.number()))
    }

    #[inline]
    fn pend(&self) {
        NVIC::pend(NrWrap(self.number()))
    }

    #[inline]
    fn unpend(&self) {
        NVIC::unpend(NrWrap(self.number()))
    }

    #[inline]
    fn get_priority(&self) -> Self::Priority {
        Self::Priority::from(NVIC::get_priority(NrWrap(self.number())))
    }

    #[inline]
    fn set_priority(&self, prio: Self::Priority) {
        unsafe {
            let mut nvic: cortex_m::peripheral::NVIC = mem::transmute(());
            nvic.set_priority(NrWrap(self.number()), prio.into())
        }
    }
}
