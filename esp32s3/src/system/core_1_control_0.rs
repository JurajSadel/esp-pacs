#[doc = "Register `CORE_1_CONTROL_0` reader"]
pub struct R(crate::R<CORE_1_CONTROL_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_1_CONTROL_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_1_CONTROL_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_1_CONTROL_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORE_1_CONTROL_0` writer"]
pub struct W(crate::W<CORE_1_CONTROL_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_1_CONTROL_0_SPEC>;
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
impl From<crate::W<CORE_1_CONTROL_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_1_CONTROL_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CONTROL_CORE_1_RUNSTALL` reader - Set 1 to stall core1"]
pub struct CONTROL_CORE_1_RUNSTALL_R(crate::FieldReader<bool, bool>);
impl CONTROL_CORE_1_RUNSTALL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CONTROL_CORE_1_RUNSTALL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CONTROL_CORE_1_RUNSTALL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CONTROL_CORE_1_RUNSTALL` writer - Set 1 to stall core1"]
pub struct CONTROL_CORE_1_RUNSTALL_W<'a> {
    w: &'a mut W,
}
impl<'a> CONTROL_CORE_1_RUNSTALL_W<'a> {
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
        self.w.bits = (self.w.bits & !1) | (value as u32 & 1);
        self.w
    }
}
#[doc = "Field `CONTROL_CORE_1_CLKGATE_EN` reader - Set 1 to open core1 clock"]
pub struct CONTROL_CORE_1_CLKGATE_EN_R(crate::FieldReader<bool, bool>);
impl CONTROL_CORE_1_CLKGATE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CONTROL_CORE_1_CLKGATE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CONTROL_CORE_1_CLKGATE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CONTROL_CORE_1_CLKGATE_EN` writer - Set 1 to open core1 clock"]
pub struct CONTROL_CORE_1_CLKGATE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CONTROL_CORE_1_CLKGATE_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u32 & 1) << 1);
        self.w
    }
}
#[doc = "Field `CONTROL_CORE_1_RESETING` reader - Set 1 to let core1 reset"]
pub struct CONTROL_CORE_1_RESETING_R(crate::FieldReader<bool, bool>);
impl CONTROL_CORE_1_RESETING_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CONTROL_CORE_1_RESETING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CONTROL_CORE_1_RESETING_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CONTROL_CORE_1_RESETING` writer - Set 1 to let core1 reset"]
pub struct CONTROL_CORE_1_RESETING_W<'a> {
    w: &'a mut W,
}
impl<'a> CONTROL_CORE_1_RESETING_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 2)) | ((value as u32 & 1) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Set 1 to stall core1"]
    #[inline(always)]
    pub fn control_core_1_runstall(&self) -> CONTROL_CORE_1_RUNSTALL_R {
        CONTROL_CORE_1_RUNSTALL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 1 to open core1 clock"]
    #[inline(always)]
    pub fn control_core_1_clkgate_en(&self) -> CONTROL_CORE_1_CLKGATE_EN_R {
        CONTROL_CORE_1_CLKGATE_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set 1 to let core1 reset"]
    #[inline(always)]
    pub fn control_core_1_reseting(&self) -> CONTROL_CORE_1_RESETING_R {
        CONTROL_CORE_1_RESETING_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to stall core1"]
    #[inline(always)]
    pub fn control_core_1_runstall(&mut self) -> CONTROL_CORE_1_RUNSTALL_W {
        CONTROL_CORE_1_RUNSTALL_W { w: self }
    }
    #[doc = "Bit 1 - Set 1 to open core1 clock"]
    #[inline(always)]
    pub fn control_core_1_clkgate_en(&mut self) -> CONTROL_CORE_1_CLKGATE_EN_W {
        CONTROL_CORE_1_CLKGATE_EN_W { w: self }
    }
    #[doc = "Bit 2 - Set 1 to let core1 reset"]
    #[inline(always)]
    pub fn control_core_1_reseting(&mut self) -> CONTROL_CORE_1_RESETING_W {
        CONTROL_CORE_1_RESETING_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Core0 control regiter 0\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_1_control_0]
(index.html) module"]
pub struct CORE_1_CONTROL_0_SPEC;
impl crate::RegisterSpec for CORE_1_CONTROL_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_1_control_0::R]
(R) reader structure"]
impl crate::Readable for CORE_1_CONTROL_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_1_control_0::W]
(W) writer structure"]
impl crate::Writable for CORE_1_CONTROL_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CORE_1_CONTROL_0 to value 0x04"]
impl crate::Resettable for CORE_1_CONTROL_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x04
    }
}