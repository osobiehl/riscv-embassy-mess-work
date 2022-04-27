use core::borrow::BorrowMut;
use core::cell::Cell;
use core::sync::atomic::{compiler_fence, Ordering};
use core::marker::Send;
use core::{mem, ptr};
use critical_section::CriticalSection;
use core::option::Option;
use core::panic;
use embassy::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy::blocking_mutex::CriticalSectionMutex as Mutex;
use embassy::interrupt::ESP32C3_Interrupts;
// use AtomicPtr;
// use embassy::interrupt::{Interrupt, InterruptExt};
use crate::interrupt;
use crate::systimer;
use embassy::time::driver::{AlarmHandle, Driver};
use esp32c3::SYSTIMER;
use riscv;

// fn noop() {}
// pub static NoOpHandler: embassy::interrupt::Handler = embassy::interrupt::Handler {
//     func: AtomicPtr::new(unsafe { mem::transmute(noop) }),
//     ctx: AtomicPtr::new(ptr::null_mut()),
// };
pub struct BaseInterrupt {
    external_interrupt_id: interrupt::interrupt_source,
    cpu_interrupt: interrupt::CpuInterrupt,
    priority: interrupt::Priority,
    // __handler: &'static embassy::interrupt::Handler,
}
//TODO: disable watchdog timers on super WDT, RTC WDT, and TIMG
pub fn generate_systimer_interrupt_structs(
    prio: interrupt::Priority,
) -> (BaseInterrupt, BaseInterrupt, BaseInterrupt) {
    (
        BaseInterrupt {
            external_interrupt_id: crate::interrupt::interrupt_source::SYSTIMER_TARGET0,
            cpu_interrupt: interrupt::CpuInterrupt::Interrupt1,
            priority: prio,
        },
        BaseInterrupt {
            external_interrupt_id: crate::interrupt::interrupt_source::SYSTIMER_TARGET1,
            cpu_interrupt: interrupt::CpuInterrupt::Interrupt2,
            priority: prio,
        },
        BaseInterrupt {
            external_interrupt_id: crate::interrupt::interrupt_source::SYSTIMER_TARGET2,
            cpu_interrupt: interrupt::CpuInterrupt::Interrupt3,
            priority: prio,
        },
    )
}

//number of independent alarms we can set for interrupts on systimer.
// as per the documentation:
// The system timer has three 52-bit comparators, shown as COMPx (x = 0, 1, or 2). The comparators can
// generate independent interrupts based on different alarm values (t) or alarm periods (δt). pg 252
const ALARM_COUNT: usize = 3;

pub struct AlarmState {
    pub timestamp: Cell<u64>,

    // This is really a Option<(fn(*mut ()), *mut ())>
    // but fn pointers aren't allowed in const yet
    pub callback: Cell<*const ()>,
    pub ctx: Cell<*mut ()>,
    pub allocated: Cell<bool>,
}

unsafe impl Send for AlarmState {}

impl AlarmState {
    pub const fn new() -> Self {
        Self {
            timestamp: Cell::new(u64::MAX),
            callback: Cell::new(ptr::null()),
            ctx: Cell::new(ptr::null_mut()),
            allocated: Cell::new(false),

        }
    }
}

fn disable_watchdogs() {}
// alarms are either set or unset:
// since we have no guarantee on which of the 3 alarms will trigger first
// we have to wrap them in an option for allocation, to see which handler is
// available, as well as to know where to allocate each fct.
pub struct SysTimerDriver {
    //we wrap everything in a cell because we try to get around
    //the non-mutable design of the Driver trait
    pub alarms: Mutex<[AlarmState; ALARM_COUNT]>,
    // esp32c3 has a 48-bit timer, that should be able to count for ~3 months (conservative estimate)
}

// enables the interrupts and sets interrupt type
fn enable(i: &mut BaseInterrupt) {
    interrupt::ESP32C3_Interrupts::enable(i.external_interrupt_id as isize, i.cpu_interrupt);
    interrupt::ESP32C3_Interrupts::set_kind(i.cpu_interrupt, interrupt::InterruptKind::Level)
}

// register driver with the rest of embassy

const ALARM_STATE_NONE: AlarmState = AlarmState::new();

embassy::time_driver_impl!(static DRIVER: SysTimerDriver = SysTimerDriver{
    alarms: Mutex::const_new(CriticalSectionRawMutex::new(), [ALARM_STATE_NONE; ALARM_COUNT]),
});


impl SysTimerDriver {
    pub fn init(&'static self, irq_prio: crate::interrupt::Priority) {
        let (mut syst0, mut syst1,mut  syst2) = generate_systimer_interrupt_structs(irq_prio);
        enable(&mut syst0);
        interrupt::ESP32C3_Interrupts::set_priority(syst0.cpu_interrupt, irq_prio as u32);
        enable(&mut syst1);
        interrupt::ESP32C3_Interrupts::set_priority(syst1.cpu_interrupt, irq_prio as u32);
        enable(&mut syst2);
        interrupt::ESP32C3_Interrupts::set_priority(syst2.cpu_interrupt, irq_prio as u32);
    }
    fn get_time(&self) -> u64 {
        unsafe {
            let systimer = &*SYSTIMER::ptr();
            systimer
                .unit0_op
                .write(|w| w.timer_unit0_update().set_bit());

            while !systimer
                .unit0_op
                .read()
                .timer_unit0_value_valid()
                .bit_is_set()
            {}

            let value_lo = systimer.unit0_value_lo.read().bits();
            let value_hi = systimer.unit0_value_hi.read().bits();

            ((value_hi as u64) << 32) | value_lo as u64
        }
    }

    fn clear_target0(&self) {
        systimer::clear_target0_interrupt();

        ESP32C3_Interrupts::clear(interrupt::CpuInterrupt::Interrupt1);
    }
    fn clear_target1(&self) {
        systimer::clear_target1_interrupt();

        ESP32C3_Interrupts::clear(interrupt::CpuInterrupt::Interrupt2);
    }
    fn clear_target2(&self) {
                systimer::clear_target2_interrupt();

        ESP32C3_Interrupts::clear(interrupt::CpuInterrupt::Interrupt3);
    }
    fn trigger_alarm(&self, id: u8, cs: CriticalSection) {
        let alarm = self.get_alarm(cs, id);
        let f: fn(*mut ()) = unsafe { mem::transmute(alarm.callback.get()) };
        let ctx = alarm.ctx.get();
        f(ctx);
    }
    #[inline(always)]
    fn deallocate_alarm(&self, id: u8 ,_cs: CriticalSection,){
        unsafe{
            let alarm = self.alarms.borrow(_cs).get_unchecked(id as usize);
            alarm.allocated.set(false);
        }
    }

    fn on_interrupt(&self, id: u8) {
        match id {
            0 => self.clear_target0(),
            1 => self.clear_target1(),
            2 => self.clear_target2(),
            _ => panic!(),
        };
        critical_section::with( |cs| {
            self.deallocate_alarm(id, cs);
            self.trigger_alarm(id, cs);
        })
        
    }

    fn get_alarm<'a>(&'a self, _cs: CriticalSection<'a>, id: u8) -> &'a AlarmState {
        unsafe {
            self.alarms.borrow(_cs).get_unchecked(id as usize)
        }
    }
    fn set_target0_alarm(&self, timestamp: u64) {
        systimer::set_target0_alarm_from_timestamp(timestamp);
    }
    fn set_target1_alarm(&self, timestamp: u64) {
        systimer::set_target1_alarm_from_timestamp(timestamp)
    }
    fn set_target2_alarm(&self, timestamp: u64) {
        systimer::set_target2_alarm_from_timestamp(timestamp)
    }
    // first clear interrupt, then keep going
}

impl Driver for SysTimerDriver {
    fn now(&self) -> u64 {
        self.get_time()
    }
    unsafe fn allocate_alarm(& self) -> Option<AlarmHandle> {
        return critical_section::with(|_cs| unsafe  {
            let alarms = self.alarms.borrow(_cs);
            for i in 0..ALARM_COUNT {

                let c = alarms.get_unchecked(i);
                if ! c.allocated.get() {
                    // set alarm so it is not overwritten
                    c.allocated.set(true);
                    return Option::Some(AlarmHandle::new(i as u8));
                }
            }
            return Option::None;
        });
    }
    fn set_alarm_callback(&self, alarm: AlarmHandle, callback: fn(*mut ()), ctx: *mut ()) {
        critical_section::with(|cs| {
            let alarm = self.get_alarm(cs, alarm.id());

            alarm.callback.set(callback as *const ());
            alarm.ctx.set(ctx);
        })
    }
    fn set_alarm(&self, alarm: AlarmHandle, timestamp: u64) {
        critical_section::with(|cs| {
            let now = self.now();
            if timestamp < now {
                self.deallocate_alarm(alarm.id(), cs);
                self.trigger_alarm(alarm.id(), cs);
                return;
            }
            let alarm_state = self.get_alarm(cs, alarm.id());
            alarm_state.timestamp.set(timestamp);
            match alarm.id() {
                0 => self.set_target0_alarm(timestamp),
                1 => self.set_target1_alarm(timestamp),
                2 => self.set_target2_alarm(timestamp),
                _ => panic!(),
            }
        })
    }
}

#[no_mangle]
pub fn interrupt1() {
    DRIVER.on_interrupt(0);
}
#[no_mangle]
pub fn interrupt2() {
    DRIVER.on_interrupt(1);
}
#[no_mangle]
pub fn interrupt3() {
    DRIVER.on_interrupt(2);
}

pub(crate) fn init(irq_prio: crate::interrupt::Priority) {
    DRIVER.init(irq_prio)
}