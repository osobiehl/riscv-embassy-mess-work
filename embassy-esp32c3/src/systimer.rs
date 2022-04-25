use crate::interrupt::{CpuInterrupt, ESP32C3_Interrupts, InterruptKind, Priority};
pub use esp32c3::{Interrupt, INTERRUPT_CORE0, SYSTIMER};
pub fn get_time() -> u64 {
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
// TODO optimize into init() and set() portions
pub fn set_target0_alarm_from_timestamp(timestamp: u64) {
    unsafe {
       
        let systimer = &*SYSTIMER::ptr();
        systimer.target0_conf.write(|w| {
            // /1. Set SYSTIMER_TARGETx_TIMER_UNIT_SEL to select the counter (UNIT0 or UNIT1) used for COMPx.
            // w.target0_timer_unit_sel().set_bit().
            let w1 = w.target0_timer_unit_sel().clear_bit();
            //3. Clear SYSTIMER_TARGETx_PERIOD_MODE to enable target mode. 
            let w2 = w1.target0_period_mode().clear_bit();
            w2
        });
        //2. Read current count value, see Section 10.5.1. This value will be used to calculate the alarm value (t) in Step
        
        let target_time = timestamp;
        //4. Set an alarm value (t), and fill its lower 32 bits to SYSTIMER_TIMER_TARGETx_LO, and the higher 20 bits to hi
        systimer
            .target0_hi
            .write(|w| w.timer_target0_hi().bits((target_time >> 32) as u32));
        systimer.target0_lo.write(|w| {
            w.timer_target0_lo()
                .bits((target_time & 0xFFFF_FFFF) as u32)
        });
        //5. Set SYSTIMER_TIMER_COMPx_LOAD to synchronize the alarm value (t) to COMPx, i.e. load the alarm
        systimer
            .comp0_load
            .write(|w| w.timer_comp0_load().set_bit());
        //6. Set SYSTIMER_TARGETx_WORK_EN to enable the selected COMPx. COMPx starts comparing the count
        systimer.conf.write(|w| {
            w.target0_work_en()
                .set_bit()
                .timer_unit0_core0_stall_en()
                .clear_bit()
        });
        // 7. Set SYSTIMER_TARGETx_INT_ENA to enable timer interrupt. When Unitn counts to the alarm value (t), a
        // SYSTIMER_TARGETx_INT interrupt is triggered
        systimer.int_ena.write(|w| w.target0_int_ena().set_bit());
    }
}

pub fn set_target1_alarm_from_timestamp(timestamp: u64) {
    unsafe {
        let micros_to_delay = delay_micros; //(delay_micros * CLK_FREQ_HZ) / 1_000_000;
        let systimer = &*SYSTIMER::ptr();
        systimer.target1_conf.write(|w| {
            // /1. Set SYSTIMER_TARGETx_TIMER_UNIT_SEL to select the counter (UNIT0 or UNIT1) used for COMPx.
            // w.target1_timer_unit_sel().set_bit().
            let w1 = w.target1_timer_unit_sel().clear_bit();
            //3. Clear SYSTIMER_TARGETx_PERIOD_MODE to enable target mode.
            let w2 = w1.target1_period_mode().clear_bit();
            w2
        });
        //2. Read current count value, see Section 10.5.1. This value will be used to calculate the alarm value (t) in Step
        let target_time = timestamp;
        //4. Set an alarm value (t), and fill its lower 32 bits to SYSTIMER_TIMER_TARGETx_LO, and the higher 20 bits to hi
        systimer
            .target1_hi
            .write(|w| w.timer_target1_hi().bits((target_time >> 32) as u32));
        systimer.target1_lo.write(|w| {
            w.timer_target1_lo()
                .bits((target_time & 0xFFFF_FFFF) as u32)
        });
        //5. Set SYSTIMER_TIMER_COMPx_LOAD to synchronize the alarm value (t) to COMPx, i.e. load the alarm
        systimer
            .comp0_load
            .write(|w| w.timer_comp0_load().set_bit());
        //6. Set SYSTIMER_TARGETx_WORK_EN to enable the selected COMPx. COMPx starts comparing the count
        systimer.conf.write(|w| {
            w.target1_work_en()
                .set_bit()
                .timer_unit0_core0_stall_en()
                .clear_bit()
        });
        // 7. Set SYSTIMER_TARGETx_INT_ENA to enable timer interrupt. When Unitn counts to the alarm value (t), a
        // SYSTIMER_TARGETx_INT interrupt is triggered
        systimer.int_ena.write(|w| w.target1_int_ena().set_bit());
    }
}

pub fn set_target2_alarm_from_timestamp(timestamp: u64) {
    unsafe {
        let micros_to_delay = delay_micros; //(delay_micros * CLK_FREQ_HZ) / 1_000_000;
        let systimer = &*SYSTIMER::ptr();
        systimer.target2_conf.write(|w| {
            // /1. Set SYSTIMER_TARGETx_TIMER_UNIT_SEL to select the counter (UNIT0 or UNIT1) used for COMPx.
            // w.target2_timer_unit_sel().set_bit().
            let w1 = w.target2_timer_unit_sel().clear_bit();
            //3. Clear SYSTIMER_TARGETx_PERIOD_MODE to enable target mode.
            let w2 = w1.target2_period_mode().clear_bit();
            w2
        });
        //2. Read current count value, see Section 10.5.1. This value will be used to calculate the alarm value (t) in Step

        let target_time = timestamp;
        //4. Set an alarm value (t), and fill its lower 32 bits to SYSTIMER_TIMER_TARGETx_LO, and the higher 20 bits to hi
        systimer
            .target2_hi
            .write(|w| w.timer_target2_hi().bits((target_time >> 32) as u32));
        systimer.target2_lo.write(|w| {
            w.timer_target2_lo()
                .bits((target_time & 0xFFFF_FFFF) as u32)
        });
        //5. Set SYSTIMER_TIMER_COMPx_LOAD to synchronize the alarm value (t) to COMPx, i.e. load the alarm
        systimer
            .comp0_load
            .write(|w| w.timer_comp0_load().set_bit());
        //6. Set SYSTIMER_TARGETx_WORK_EN to enable the selected COMPx. COMPx starts comparing the count
        systimer.conf.write(|w| {
            w.target2_work_en()
                .set_bit()
                .timer_unit0_core0_stall_en()
                .clear_bit()
        });
        // 7. Set SYSTIMER_TARGETx_INT_ENA to enable timer interrupt. When Unitn counts to the alarm value (t), a
        // SYSTIMER_TARGETx_INT interrupt is triggered
        systimer.int_ena.write(|w| w.target2_int_ena().set_bit());
    }
}

pub fn clear_target0_interrupt(){
    unsafe{
        let systimer = &*SYSTIMER::ptr();
        //stop comparing values to interrupt so no new interrupt is triggerred
        systimer.int_clr.write(|w| w.target0_int_clr().set_bit());
    }
}
pub fn clear_target1_interrupt(){
    unsafe{
        let systimer = &*SYSTIMER::ptr();
        //stop comparing values to interrupt so no new interrupt is triggerred
        systimer.int_clr.write(|w| w.target1_int_clr().set_bit());
    }
}
pub fn clear_target2_interrupt(){
    unsafe{
        let systimer = &*SYSTIMER::ptr();
        //stop comparing values to interrupt so no new interrupt is triggerred
        systimer.int_clr.write(|w| w.target2_int_clr().set_bit());
    }
}