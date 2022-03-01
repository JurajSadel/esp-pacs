#[doc = "Register `CORE_1_RCD_RECORDING` reader"]
pub struct R(crate::R<CORE_1_RCD_RECORDING_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_1_RCD_RECORDING_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_1_RCD_RECORDING_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_1_RCD_RECORDING_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORE_1_RCD_RECORDING` writer"]
pub struct W(crate::W<CORE_1_RCD_RECORDING_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_1_RCD_RECORDING_SPEC>;
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
impl From<crate::W<CORE_1_RCD_RECORDING_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_1_RCD_RECORDING_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_1_RCD_RECORDING` reader - Pdebug record enable,set 1 to record Core1 pdebug interface signal"]
pub struct CORE_1_RCD_RECORDING_R(crate::FieldReader<bool, bool>);
impl CORE_1_RCD_RECORDING_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CORE_1_RCD_RECORDING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_1_RCD_RECORDING_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_1_RCD_RECORDING` writer - Pdebug record enable,set 1 to record Core1 pdebug interface signal"]
pub struct CORE_1_RCD_RECORDING_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_1_RCD_RECORDING_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Pdebug record enable,set 1 to record Core1 pdebug interface signal"]
    #[inline(always)]
    pub fn core_1_rcd_recording(&self) -> CORE_1_RCD_RECORDING_R {
        CORE_1_RCD_RECORDING_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pdebug record enable,set 1 to record Core1 pdebug interface signal"]
    #[inline(always)]
    pub fn core_1_rcd_recording(&mut self) -> CORE_1_RCD_RECORDING_W {
        CORE_1_RCD_RECORDING_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Core1 pdebug status register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_1_rcd_recording]
(index.html) module"]
pub struct CORE_1_RCD_RECORDING_SPEC;
impl crate::RegisterSpec for CORE_1_RCD_RECORDING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_1_rcd_recording::R]
(R) reader structure"]
impl crate::Readable for CORE_1_RCD_RECORDING_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_1_rcd_recording::W]
(W) writer structure"]
impl crate::Writable for CORE_1_RCD_RECORDING_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CORE_1_RCD_RECORDING to value 0"]
impl crate::Resettable for CORE_1_RCD_RECORDING_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}