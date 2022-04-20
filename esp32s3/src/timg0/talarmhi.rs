#[doc = "Register `T%sALARMHI` reader"]
pub struct R(crate::R<TALARMHI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TALARMHI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TALARMHI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TALARMHI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `T%sALARMHI` writer"]
pub struct W(crate::W<TALARMHI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TALARMHI_SPEC>;
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
impl From<crate::W<TALARMHI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TALARMHI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T0_ALARM_HI` reader - Timer %s alarm trigger time-base counter value, high 22 bits."]
pub struct T0_ALARM_HI_R(crate::FieldReader<u32, u32>);
impl T0_ALARM_HI_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        T0_ALARM_HI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T0_ALARM_HI_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T0_ALARM_HI` writer - Timer %s alarm trigger time-base counter value, high 22 bits."]
pub struct T0_ALARM_HI_W<'a> {
    w: &'a mut W,
}
impl<'a> T0_ALARM_HI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x003f_ffff) | (value as u32 & 0x003f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:21 - Timer %s alarm trigger time-base counter value, high 22 bits."]
    #[inline(always)]
    pub fn t0_alarm_hi(&self) -> T0_ALARM_HI_R {
        T0_ALARM_HI_R::new((self.bits & 0x003f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:21 - Timer %s alarm trigger time-base counter value, high 22 bits."]
    #[inline(always)]
    pub fn t0_alarm_hi(&mut self) -> T0_ALARM_HI_W {
        T0_ALARM_HI_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer %s alarm value, high bits\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [talarmhi]
(index.html) module"]
pub struct TALARMHI_SPEC;
impl crate::RegisterSpec for TALARMHI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [talarmhi::R]
(R) reader structure"]
impl crate::Readable for TALARMHI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [talarmhi::W]
(W) writer structure"]
impl crate::Writable for TALARMHI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets T%sALARMHI to value 0"]
impl crate::Resettable for TALARMHI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}