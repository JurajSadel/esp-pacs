#[doc = "Register `INT_RAW_RTC` reader"]
pub struct R(crate::R<INT_RAW_RTC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_RAW_RTC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_RAW_RTC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_RAW_RTC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SLP_WAKEUP_INT_RAW` reader - sleep wakeup interrupt raw"]
pub type SLP_WAKEUP_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `SLP_REJECT_INT_RAW` reader - sleep reject interrupt raw"]
pub type SLP_REJECT_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `WDT_INT_RAW` reader - RTC WDT interrupt raw"]
pub type WDT_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `BROWN_OUT_INT_RAW` reader - brown out interrupt raw"]
pub type BROWN_OUT_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `MAIN_TIMER_INT_RAW` reader - RTC main timer interrupt raw"]
pub type MAIN_TIMER_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `SWD_INT_RAW` reader - super watch dog interrupt raw"]
pub type SWD_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `XTAL32K_DEAD_INT_RAW` reader - xtal32k dead detection interrupt raw"]
pub type XTAL32K_DEAD_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `GLITCH_DET_INT_RAW` reader - glitch_det_interrupt_raw"]
pub type GLITCH_DET_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `BBPLL_CAL_INT_RAW` reader - bbpll cal end interrupt state"]
pub type BBPLL_CAL_INT_RAW_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - sleep wakeup interrupt raw"]
    #[inline(always)]
    pub fn slp_wakeup_int_raw(&self) -> SLP_WAKEUP_INT_RAW_R {
        SLP_WAKEUP_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - sleep reject interrupt raw"]
    #[inline(always)]
    pub fn slp_reject_int_raw(&self) -> SLP_REJECT_INT_RAW_R {
        SLP_REJECT_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - RTC WDT interrupt raw"]
    #[inline(always)]
    pub fn wdt_int_raw(&self) -> WDT_INT_RAW_R {
        WDT_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 9 - brown out interrupt raw"]
    #[inline(always)]
    pub fn brown_out_int_raw(&self) -> BROWN_OUT_INT_RAW_R {
        BROWN_OUT_INT_RAW_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RTC main timer interrupt raw"]
    #[inline(always)]
    pub fn main_timer_int_raw(&self) -> MAIN_TIMER_INT_RAW_R {
        MAIN_TIMER_INT_RAW_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - super watch dog interrupt raw"]
    #[inline(always)]
    pub fn swd_int_raw(&self) -> SWD_INT_RAW_R {
        SWD_INT_RAW_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - xtal32k dead detection interrupt raw"]
    #[inline(always)]
    pub fn xtal32k_dead_int_raw(&self) -> XTAL32K_DEAD_INT_RAW_R {
        XTAL32K_DEAD_INT_RAW_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 19 - glitch_det_interrupt_raw"]
    #[inline(always)]
    pub fn glitch_det_int_raw(&self) -> GLITCH_DET_INT_RAW_R {
        GLITCH_DET_INT_RAW_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - bbpll cal end interrupt state"]
    #[inline(always)]
    pub fn bbpll_cal_int_raw(&self) -> BBPLL_CAL_INT_RAW_R {
        BBPLL_CAL_INT_RAW_R::new(((self.bits >> 20) & 1) != 0)
    }
}
#[doc = "rtc configure register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_raw_rtc](index.html) module"]
pub struct INT_RAW_RTC_SPEC;
impl crate::RegisterSpec for INT_RAW_RTC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_raw_rtc::R](R) reader structure"]
impl crate::Readable for INT_RAW_RTC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INT_RAW_RTC to value 0"]
impl crate::Resettable for INT_RAW_RTC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}