use esp32c3::RTC_CNTL;
// use crate::pac::RTC_CNTL;
pub struct RtcCntl {
    rtc_cntl: RTC_CNTL,
}

impl RtcCntl {
    pub fn new(rtc_cntl: RTC_CNTL) -> Self {
        Self { rtc_cntl }
    }

    pub fn set_super_wdt_enable(&mut self, enable: bool) {
        self.set_swd_write_protection(false);

        self.rtc_cntl
            .swd_conf
            .write(|w| w.swd_auto_feed_en().bit(!enable));

        self.set_swd_write_protection(true);
    }

    fn set_swd_write_protection(&mut self, enable: bool) {
        let wkey = if enable { 0u32 } else { 0x8F1D_312A };

        self.rtc_cntl
            .swd_wprotect
            .write(|w| unsafe { w.swd_wkey().bits(wkey) });
    }

    pub fn set_wdt_enable(&mut self, enable: bool) {
        self.set_wdt_write_protection(false);

        if !enable {
            self.rtc_cntl.wdtconfig0.write(|w| unsafe { w.bits(0) });
        } else {
            self.rtc_cntl.wdtconfig0.write(|w| w.wdt_en().bit(enable));
        }

        self.set_wdt_write_protection(true);
    }

    fn set_wdt_write_protection(&mut self, enable: bool) {
        let wkey = if enable { 0u32 } else { 0x50D8_3AA1 };

        self.rtc_cntl
            .wdtwprotect
            .write(|w| unsafe { w.wdt_wkey().bits(wkey) });
    }
}
