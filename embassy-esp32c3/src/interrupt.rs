use atomic_polyfill::{compiler_fence, Ordering};
use core::ptr;
use embassy::interrupt::{Interrupt, InterruptExt};
pub use esp32c3::interrupt::Interrupt as interrupt_source;
use esp32c3::{ INTERRUPT_CORE0, SYSTEM};
use riscv::register::mcause;
use riscv_atomic_emulation_trap as _;
//TODO SETUP BOOT LINKING
// User code shouldn't usually take the mutable TrapFrame or the TrapFrame in
// general. However this makes things like preemtive multitasking easier in
// future
extern "C" {
    fn interrupt1(frame: &mut TrapFrame);
    fn interrupt2(frame: &mut TrapFrame);
    fn interrupt3(frame: &mut TrapFrame);
    // fn interrupt4(frame: &mut TrapFrame);
    fn interrupt5(frame: &mut TrapFrame);
    fn interrupt6(frame: &mut TrapFrame);
    fn interrupt7(frame: &mut TrapFrame);
    fn interrupt8(frame: &mut TrapFrame);
    fn interrupt9(frame: &mut TrapFrame);
    fn interrupt10(frame: &mut TrapFrame);
    fn interrupt11(frame: &mut TrapFrame);
    fn interrupt12(frame: &mut TrapFrame);
    fn interrupt13(frame: &mut TrapFrame);
    fn interrupt14(frame: &mut TrapFrame);
    fn interrupt15(frame: &mut TrapFrame);
    fn interrupt16(frame: &mut TrapFrame);
    fn interrupt17(frame: &mut TrapFrame);
    fn interrupt18(frame: &mut TrapFrame);
    fn interrupt19(frame: &mut TrapFrame);
    fn interrupt20(frame: &mut TrapFrame);
    fn interrupt21(frame: &mut TrapFrame);
    fn interrupt22(frame: &mut TrapFrame);
    fn interrupt23(frame: &mut TrapFrame);
    fn interrupt24(frame: &mut TrapFrame);
    fn interrupt25(frame: &mut TrapFrame);
    fn interrupt26(frame: &mut TrapFrame);
    fn interrupt27(frame: &mut TrapFrame);
    fn interrupt28(frame: &mut TrapFrame);
    fn interrupt29(frame: &mut TrapFrame);
    fn interrupt30(frame: &mut TrapFrame);
    fn interrupt31(frame: &mut TrapFrame);
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
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy)]
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
impl Into<u8> for Priority {
    fn into(self) -> u8 {
        return self as u8;
    }
}
impl From<u8> for Priority {
    fn from(v: u8) -> Self {
        match v {
            1 => Priority::Priority1,
            2 => Priority::Priority2,
            3 => Priority::Priority3,
            4 => Priority::Priority4,
            5 => Priority::Priority5,
            6 => Priority::Priority6,
            7 => Priority::Priority7,
            8 => Priority::Priority8,
            9 => Priority::Priority9,
            10 => Priority::Priority10,
            11 => Priority::Priority11,
            12 => Priority::Priority12,
            13 => Priority::Priority13,
            14 => Priority::Priority14,
            15 => Priority::Priority15,
            _ => Priority::None,
        }
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
    // #[no_mangle]

    let cause = mcause::read();
    if cause.is_exception() {
        handle_exception(trap_frame);
    } else {
        let code = riscv::register::mcause::read().code();
        match code {
            1 => interrupt1(trap_frame.as_mut().unwrap()),
            2 => interrupt2(trap_frame.as_mut().unwrap()),
            3 => interrupt3(trap_frame.as_mut().unwrap()),
            4 => interrupt4(trap_frame.as_mut().unwrap()),
            5 => interrupt5(trap_frame.as_mut().unwrap()),
            6 => interrupt6(trap_frame.as_mut().unwrap()),
            7 => interrupt7(trap_frame.as_mut().unwrap()),
            8 => interrupt8(trap_frame.as_mut().unwrap()),
            9 => interrupt9(trap_frame.as_mut().unwrap()),
            10 => interrupt10(trap_frame.as_mut().unwrap()),
            11 => interrupt11(trap_frame.as_mut().unwrap()),
            12 => interrupt12(trap_frame.as_mut().unwrap()),
            13 => interrupt13(trap_frame.as_mut().unwrap()),
            14 => interrupt14(trap_frame.as_mut().unwrap()),
            16 => interrupt16(trap_frame.as_mut().unwrap()),
            15 => interrupt15(trap_frame.as_mut().unwrap()),
            17 => interrupt17(trap_frame.as_mut().unwrap()),
            18 => interrupt18(trap_frame.as_mut().unwrap()),
            19 => interrupt19(trap_frame.as_mut().unwrap()),
            20 => interrupt20(trap_frame.as_mut().unwrap()),
            21 => interrupt21(trap_frame.as_mut().unwrap()),
            22 => interrupt22(trap_frame.as_mut().unwrap()),
            23 => interrupt23(trap_frame.as_mut().unwrap()),
            24 => interrupt24(trap_frame.as_mut().unwrap()),
            25 => interrupt25(trap_frame.as_mut().unwrap()),
            26 => interrupt26(trap_frame.as_mut().unwrap()),
            27 => interrupt27(trap_frame.as_mut().unwrap()),
            28 => interrupt28(trap_frame.as_mut().unwrap()),
            29 => interrupt29(trap_frame.as_mut().unwrap()),
            30 => interrupt30(trap_frame.as_mut().unwrap()),
            31 => interrupt31(trap_frame.as_mut().unwrap()),
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
pub unsafe fn handle_exception(trap_frame: *mut TrapFrame) {
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

#[allow(non_snake_case)]
pub mod ESP32C3_Interrupts {
    pub use super::*;
    pub fn enable(interrupt_number: isize, cpu_interrupt_number: CpuInterrupt) {
        unsafe {
            let cpu_interrupt_number = cpu_interrupt_number as isize;
            let intr = &*INTERRUPT_CORE0::ptr();
            let intr_map_base = intr.mac_intr_map.as_ptr();
            intr_map_base
                .offset(interrupt_number)
                .write_volatile(cpu_interrupt_number as u32);

            // enable interrupt
            intr.cpu_int_enable
                .modify(|r, w| w.bits((1 << cpu_interrupt_number) | r.bits()));
        }
    }
    pub fn disable(interrupt: isize) {
        unsafe {
            let interrupt_number = interrupt as isize;
            let intr = &*INTERRUPT_CORE0::ptr();
            let intr_map_base = intr.mac_intr_map.as_ptr();
            intr_map_base.offset(interrupt_number).write_volatile(0);
        }
    }
    pub fn set_kind(which: CpuInterrupt, kind: InterruptKind) {
        unsafe {
            let intr = &*INTERRUPT_CORE0::ptr();
            let cpu_interrupt_number = which as isize;

            let interrupt_type = match kind {
                InterruptKind::Level => 0,
                InterruptKind::Edge => 1,
            };
            intr.cpu_int_type.modify(|r, w| {
                w.bits(
                    r.bits() & !(1 << cpu_interrupt_number)
                        | (interrupt_type << cpu_interrupt_number),
                )
            });
        }
    }
    pub fn set_priority(which: CpuInterrupt, priority: u32) {
        unsafe {
            let intr = &*INTERRUPT_CORE0::ptr();
            let cpu_interrupt_number = which as isize;
            let intr_prio_base = intr.cpu_int_pri_0.as_ptr();

            intr_prio_base
                .offset(cpu_interrupt_number as isize)
                .write_volatile(priority as u32);
        }
    }
    pub fn clear(which: CpuInterrupt) {
        unsafe {
            let cpu_interrupt_number = which as isize;
            let intr = &*INTERRUPT_CORE0::ptr();
            intr.cpu_int_clear
                .write(|w| w.bits(1 << cpu_interrupt_number));
        }
    }
    //TODO implement testing for these functions
    pub fn is_enabled(cpu_interrupt_number: CpuInterrupt) -> bool {
        unsafe {
            let intr = &*INTERRUPT_CORE0::ptr();
            let b = intr.cpu_int_enable.read().bits();
            let ans = b & (1 << cpu_interrupt_number as isize);
            ans != 0
        }
    }
    pub fn is_pending(cpu_interrupt_number: CpuInterrupt) -> bool {
        unsafe {
            let intr = &*INTERRUPT_CORE0::ptr();
            let b = intr.cpu_int_eip_status.read().bits();
            let ans = b & (1 << cpu_interrupt_number as isize);
            ans != 0
        }
    }
    pub fn get_priority(which: CpuInterrupt) -> u32 {
        unsafe {
            let intr = &*INTERRUPT_CORE0::ptr();
            let cpu_interrupt_number = which as isize;
            let intr_prio_base = intr.cpu_int_pri_0.as_ptr();

            let x = intr_prio_base
                .offset(cpu_interrupt_number as isize)
                .read_volatile();
            return x;
        }
    }
}

static SW_INT1_HANDLER: embassy::interrupt::Handler = embassy::interrupt::Handler::new();
/// Wrapper for software interrupt 1, can be used for interrupt mode executor
#[allow(non_camel_case_types)]
pub struct SW_INT1 {}
impl SW_INT1 {
   pub fn new(prio: Priority) -> Self {
        let s = Self {};
        s.set_priority(prio);
        s
    }
}
unsafe impl Interrupt for SW_INT1 {
    type Priority = Priority;
    fn number(&self) -> isize {
        4
    }
    unsafe fn steal() -> Self {
        Self {}
    }
    unsafe fn __handler(&self) -> &'static embassy::interrupt::Handler {
        //TODO
        return &SW_INT1_HANDLER;
    }
}


impl InterruptExt for SW_INT1 {
    fn enable(&self) {
        unsafe {
            let intr = &*INTERRUPT_CORE0::ptr();
            intr.cpu_intr_from_cpu_0_map
                .modify(|_, w| w.cpu_intr_from_cpu_0_map().bits(4));
            intr.cpu_int_enable
                .modify(|r, w| w.bits((1 << 4) | r.bits()));
            ESP32C3_Interrupts::set_priority(CpuInterrupt::Interrupt4, Priority::Priority1 as u32);
            ESP32C3_Interrupts::set_kind(CpuInterrupt::Interrupt4, InterruptKind::Level);
        }
    }
    fn disable(&self) {
        unsafe {
        ESP32C3_Interrupts::disable(CpuInterrupt::Interrupt4 as isize);
        let intr = &*INTERRUPT_CORE0::ptr();
        intr.cpu_intr_from_cpu_0_map
            .modify(|_, w| w.cpu_intr_from_cpu_0_map().bits(0));
        intr.cpu_int_enable
            .modify(|r, w| w.bits( !(1 << 4) & r.bits()));
        }

    }
    fn get_priority(&self) -> Self::Priority {
        let p = ESP32C3_Interrupts::get_priority(CpuInterrupt::Interrupt4);
        //TODO derive from for priority
        Priority::from(p as u8)
    }
    fn is_active(&self) -> bool {
        // TODO read from MIE
        let code = riscv::register::mcause::read().code();
        code == 4
    }
    fn is_pending(&self) -> bool {
        ESP32C3_Interrupts::is_pending(CpuInterrupt::Interrupt4)
    }
    fn is_enabled(&self) -> bool {
        ESP32C3_Interrupts::is_enabled(CpuInterrupt::Interrupt4)
    }
    fn pend(&self) {
        unsafe {
            let system = &*SYSTEM::ptr();
            system
                .cpu_intr_from_cpu_0
                .modify(|_, w| w.cpu_intr_from_cpu_0().set_bit());
        }
    }
    fn unpend(&self) {
        unsafe {
            let system = &*SYSTEM::ptr();
            system
                .cpu_intr_from_cpu_0
                .modify(|_, w| w.cpu_intr_from_cpu_0().clear_bit());
            ESP32C3_Interrupts::clear(CpuInterrupt::Interrupt4);
        }
    }
    fn set_priority(&self, prio: Self::Priority) {
        ESP32C3_Interrupts::set_priority(CpuInterrupt::Interrupt4, prio as u32);
    }
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
}
unsafe impl ::embassy::util::Unborrow for SW_INT1 {
    type Target = SW_INT1;
    unsafe fn unborrow(self) -> SW_INT1 {
        self
    }
}
//interrupt4 is defined here
#[no_mangle]
pub fn interrupt4(_: *mut TrapFrame) {
    unsafe {
        let func = SW_INT1_HANDLER
            .func
            .load(embassy::export::atomic::Ordering::Relaxed);
        let ctx = SW_INT1_HANDLER
            .ctx
            .load(embassy::export::atomic::Ordering::Relaxed);
        let func: fn(*mut ()) = core::mem::transmute(func);
        func(ctx)
    }
}
