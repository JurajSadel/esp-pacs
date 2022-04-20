#[doc = "Register `RTC_GPIO_STATUS_W1TS` writer"]
pub struct W(crate::W<RTC_GPIO_STATUS_W1TS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_GPIO_STATUS_W1TS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<RTC_GPIO_STATUS_W1TS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_GPIO_STATUS_W1TS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO_STATUS_INT_W1TS` writer - GPIO0 ~ 21 interrupt set register. If the value 1 is written to a bit here, the corresponding bit in RTCIO_GPIO_STATUS_INT will be set to 1. Recommended operation: use this register to set RTCIO_GPIO_STATUS_INT."]
pub struct GPIO_STATUS_INT_W1TS_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_STATUS_INT_W1TS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x003f_ffff << 10)) | ((value as u32 & 0x003f_ffff) << 10);
        self.w
    }
}
impl W {
    #[doc = "Bits 10:31 - GPIO0 ~ 21 interrupt set register. If the value 1 is written to a bit here, the corresponding bit in RTCIO_GPIO_STATUS_INT will be set to 1. Recommended operation: use this register to set RTCIO_GPIO_STATUS_INT."]
    #[inline(always)]
    pub fn gpio_status_int_w1ts(&mut self) -> GPIO_STATUS_INT_W1TS_W {
        GPIO_STATUS_INT_W1TS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC GPIO interrupt status bit set register\n\nThis register you can [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_gpio_status_w1ts]
(index.html) module"]
pub struct RTC_GPIO_STATUS_W1TS_SPEC;
impl crate::RegisterSpec for RTC_GPIO_STATUS_W1TS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [rtc_gpio_status_w1ts::W]
(W) writer structure"]
impl crate::Writable for RTC_GPIO_STATUS_W1TS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_GPIO_STATUS_W1TS to value 0"]
impl crate::Resettable for RTC_GPIO_STATUS_W1TS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}