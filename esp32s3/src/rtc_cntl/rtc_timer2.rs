#[doc = "Register `RTC_TIMER2` reader"]
pub struct R(crate::R<RTC_TIMER2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_TIMER2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_TIMER2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_TIMER2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_TIMER2` writer"]
pub struct W(crate::W<RTC_TIMER2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_TIMER2_SPEC>;
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
impl From<crate::W<RTC_TIMER2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_TIMER2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ULPCP_TOUCH_START_WAIT` reader - wait cycles in slow_clk_rtc before ULP-coprocessor / touch controller start to work"]
pub struct ULPCP_TOUCH_START_WAIT_R(crate::FieldReader<u16, u16>);
impl ULPCP_TOUCH_START_WAIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        ULPCP_TOUCH_START_WAIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ULPCP_TOUCH_START_WAIT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ULPCP_TOUCH_START_WAIT` writer - wait cycles in slow_clk_rtc before ULP-coprocessor / touch controller start to work"]
pub struct ULPCP_TOUCH_START_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> ULPCP_TOUCH_START_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 15)) | ((value as u32 & 0x01ff) << 15);
        self.w
    }
}
#[doc = "Field `MIN_TIME_CK8M_OFF` reader - minimal cycles in slow_clk_rtc for CK8M in power down state"]
pub struct MIN_TIME_CK8M_OFF_R(crate::FieldReader<u8, u8>);
impl MIN_TIME_CK8M_OFF_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MIN_TIME_CK8M_OFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MIN_TIME_CK8M_OFF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MIN_TIME_CK8M_OFF` writer - minimal cycles in slow_clk_rtc for CK8M in power down state"]
pub struct MIN_TIME_CK8M_OFF_W<'a> {
    w: &'a mut W,
}
impl<'a> MIN_TIME_CK8M_OFF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 15:23 - wait cycles in slow_clk_rtc before ULP-coprocessor / touch controller start to work"]
    #[inline(always)]
    pub fn ulpcp_touch_start_wait(&self) -> ULPCP_TOUCH_START_WAIT_R {
        ULPCP_TOUCH_START_WAIT_R::new(((self.bits >> 15) & 0x01ff) as u16)
    }
    #[doc = "Bits 24:31 - minimal cycles in slow_clk_rtc for CK8M in power down state"]
    #[inline(always)]
    pub fn min_time_ck8m_off(&self) -> MIN_TIME_CK8M_OFF_R {
        MIN_TIME_CK8M_OFF_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 15:23 - wait cycles in slow_clk_rtc before ULP-coprocessor / touch controller start to work"]
    #[inline(always)]
    pub fn ulpcp_touch_start_wait(&mut self) -> ULPCP_TOUCH_START_WAIT_W {
        ULPCP_TOUCH_START_WAIT_W { w: self }
    }
    #[doc = "Bits 24:31 - minimal cycles in slow_clk_rtc for CK8M in power down state"]
    #[inline(always)]
    pub fn min_time_ck8m_off(&mut self) -> MIN_TIME_CK8M_OFF_W {
        MIN_TIME_CK8M_OFF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rtc monitor state delay time\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_timer2]
(index.html) module"]
pub struct RTC_TIMER2_SPEC;
impl crate::RegisterSpec for RTC_TIMER2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_timer2::R]
(R) reader structure"]
impl crate::Readable for RTC_TIMER2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_timer2::W]
(W) writer structure"]
impl crate::Writable for RTC_TIMER2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_TIMER2 to value 0x0108_0000"]
impl crate::Resettable for RTC_TIMER2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0108_0000
    }
}