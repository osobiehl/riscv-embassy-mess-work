use core::cell::Cell;
use core::sync::atomic::{compiler_fence, AtomicU32, AtomicU8, Ordering};
use core::{mem, ptr};
use critical_section::CriticalSection;
use critical_section::CriticalSection;
use embassy::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy::blocking_mutex::CriticalSectionMutex as Mutex;
use embassy::interrupt::ESP32C3_Interrupts;
use std::borrow::BorrowMut;
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
struct BaseInterrupt {
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
struct AlarmState {
    timestamp: Cell<u64>,

    // This is really a Option<(fn(*mut ()), *mut ())>
    // but fn pointers aren't allowed in const yet
    callback: Cell<*const ()>,
    ctx: Cell<*mut ()>,
}

unsafe impl Send for AlarmState {}

impl AlarmState {
    const fn new() -> Self {
        Self {
            timestamp: Cell::new(u64::MAX),
            callback: Cell::new(ptr::null()),
            ctx: Cell::new(ptr::null_mut()),
        }
    }
}

fn disable_watchdogs() {}
// alarms are either set or unset:
// since we have no guarantee on which of the 3 alarms will trigger first
// we have to wrap them in an option for allocation, to see which handler is
// available, as well as to know where to allocate each fct.
struct SysTimerDriver {
    alarms: Mutex<[Option<AlarmState>; ALARM_COUNT]>,
    // esp32c3 has a 48-bit timer, that should be able to count for ~3 months (conservative estimate)
}

// enables the interrupts and sets interrupt type
fn enable(i: &mut BaseInterrupt) {
    interrupt::ESP32C3_Interrupts::enable(i.external_interrupt_id as isize, i.cpu_interrupt);
    interrupt::ESP32C3_Interrupts::set_kind(i.cpu_interrupt, interrupt::InterruptKind::Level)
}

// register driver with the rest of embassy
embassy::time_driver_impl!(static DRIVER: SysTimerDriver = SysTimerDriver{
    alarms: Mutex::const_new(CriticalSectionRawMutex::new(), [None; ALARM_COUNT]);
});


impl SysTimerDriver {
    fn init(&'static self, irq_prio: crate::interrupt::Priority) {
        let (syst0, syst1, syst2) = generate_systimer_interrupt_structs(irq_prio);
        enable(&mut syst0);
        interrupt::ESP32C3_Interrupts::set_priority(syst0, irq_prio);
        enable(&mut syst1);
        interrupt::ESP32C3_Interrupts::set_priority(syst1, irq_prio);
        enable(&mut syst2);
        interrupt::ESP32C3_Interrupts::set_priority(syst2, irq_prio);
    }
    fn get_time() -> u64 {
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
        ESP32C3_Interrupts::clear(Interrupt::CpuInterrupt::Interrupt1);
        systimer::clear_target0_interrupt();
    }
    fn clear_target1(&self) {
        ESP32C3_Interrupts::clear(Interrupt::CpuInterrupt::Interrupt2);
        systimer::clear_target1_interrupt();
    }
    fn clear_target2(&self) {
        ESP32C3_Interrupts::clear(Interrupt::CpuInterrupt::Interrupt3);
        systimer.clear_target2_interrupt();
    }
    fn take_alarm(&self, id: u8, cs: CriticalSection)->AlarmState {
        let alarms = self.alarms.borrow(cs);
        let idx = match id {
            0 => 0,
            1 => 1,
            2 => 2,
            _ => panic!(),
        };
        let alarm = (*alarms)[idx].take().unwrap();
        return alarm;

    }

    fn trigger_alarm(&self, id: u8, cs: CriticalSection) {
        let alarm = self.take_alarm(id, cs);
        let f: fn(*mut ()) = unsafe { mem::transmute(alarm.callback.get()) };
        let ctx = alarm.ctx.get();
        f(ctx);
    }

    fn on_interrupt(&self, id: u32) {
        match id {
            0 => self.clear_target0(),
            1 => self.clear_target1(),
            2 => self.clear_target2(),
            _ => panic!(),
        };
        self.trigger_alarm(id, cs);
    }

    fn get_alarm<'a>(&'a mut self, _cs: CriticalSection, id: u8) -> &'a mut AlarmState {
        unsafe {
            let alarms = self.alarms.borrow(cs);
            return alarms.get_unchecked_mut(handle.id());
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
    pub fn on_interrupt_target0(&self) {}
}

impl Driver for SysTimerDriver {
    fn now(&self) -> u64 {
        self.get_time()
    }
    unsafe fn allocate_alarm(&self) -> Option<AlarmHandle> {
        critical_section::with(|cs| {
            let alarms = self.alarms.borrow(cs);
            for i in 0..ALARM_COUNT {
                if let None = alarms[i] {
                    // set alarm so it is not overwritten
                    alarms[i] = Some(AlarmState::new());
                    return Some(AlarmHandle::new(i as u8));
                }
            }
            return None;
        });
        return None;
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
            let alarm_state = self.get_alarm(cs, alarm);
            alarm_state.timestamp = timestamp;
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
