#[doc = "Register `RTC_GPIO_ENABLE` reader"]
pub struct R(crate::R<RTC_GPIO_ENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_GPIO_ENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_GPIO_ENABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_GPIO_ENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_GPIO_ENABLE` writer"]
pub struct W(crate::W<RTC_GPIO_ENABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_GPIO_ENABLE_SPEC>;
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
impl From<crate::W<RTC_GPIO_ENABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_GPIO_ENABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REG_RTCIO_REG_GPIO_ENABLE` reader - GPIO0 ~ 21 output enable. Bit10 corresponds to GPIO0, bit11 corresponds to GPIO1, etc. If the bit is set to 1, it means this GPIO pad is output."]
pub struct REG_RTCIO_REG_GPIO_ENABLE_R(crate::FieldReader<u32>);
impl REG_RTCIO_REG_GPIO_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        REG_RTCIO_REG_GPIO_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_RTCIO_REG_GPIO_ENABLE_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REG_RTCIO_REG_GPIO_ENABLE` writer - GPIO0 ~ 21 output enable. Bit10 corresponds to GPIO0, bit11 corresponds to GPIO1, etc. If the bit is set to 1, it means this GPIO pad is output."]
pub struct REG_RTCIO_REG_GPIO_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_RTCIO_REG_GPIO_ENABLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x003f_ffff << 10)) | ((value as u32 & 0x003f_ffff) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bits 10:31 - GPIO0 ~ 21 output enable. Bit10 corresponds to GPIO0, bit11 corresponds to GPIO1, etc. If the bit is set to 1, it means this GPIO pad is output."]
    #[inline(always)]
    pub fn reg_rtcio_reg_gpio_enable(&self) -> REG_RTCIO_REG_GPIO_ENABLE_R {
        REG_RTCIO_REG_GPIO_ENABLE_R::new(((self.bits >> 10) & 0x003f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 10:31 - GPIO0 ~ 21 output enable. Bit10 corresponds to GPIO0, bit11 corresponds to GPIO1, etc. If the bit is set to 1, it means this GPIO pad is output."]
    #[inline(always)]
    pub fn reg_rtcio_reg_gpio_enable(&mut self) -> REG_RTCIO_REG_GPIO_ENABLE_W {
        REG_RTCIO_REG_GPIO_ENABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC GPIO output enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_gpio_enable](index.html) module"]
pub struct RTC_GPIO_ENABLE_SPEC;
impl crate::RegisterSpec for RTC_GPIO_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_gpio_enable::R](R) reader structure"]
impl crate::Readable for RTC_GPIO_ENABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_gpio_enable::W](W) writer structure"]
impl crate::Writable for RTC_GPIO_ENABLE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_GPIO_ENABLE to value 0"]
impl crate::Resettable for RTC_GPIO_ENABLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
