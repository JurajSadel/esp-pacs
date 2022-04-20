#[doc = "Register `TARGET2_CONF` reader"]
pub struct R(crate::R<TARGET2_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TARGET2_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TARGET2_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TARGET2_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TARGET2_CONF` writer"]
pub struct W(crate::W<TARGET2_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TARGET2_CONF_SPEC>;
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
impl From<crate::W<TARGET2_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TARGET2_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TARGET2_PERIOD` reader - Set alarm period for system timer target 2, only valid in periodic alarms mode."]
pub struct TARGET2_PERIOD_R(crate::FieldReader<u32, u32>);
impl TARGET2_PERIOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        TARGET2_PERIOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TARGET2_PERIOD_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TARGET2_PERIOD` writer - Set alarm period for system timer target 2, only valid in periodic alarms mode."]
pub struct TARGET2_PERIOD_W<'a> {
    w: &'a mut W,
}
impl<'a> TARGET2_PERIOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff_ffff) | (value as u32 & 0x3fff_ffff);
        self.w
    }
}
#[doc = "Field `TARGET2_PERIOD_MODE` reader - Set work mode for system timer target 2. 0: work in a timedelay alarm mode; 1: work in periodic alarms mode."]
pub struct TARGET2_PERIOD_MODE_R(crate::FieldReader<bool, bool>);
impl TARGET2_PERIOD_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TARGET2_PERIOD_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TARGET2_PERIOD_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TARGET2_PERIOD_MODE` writer - Set work mode for system timer target 2. 0: work in a timedelay alarm mode; 1: work in periodic alarms mode."]
pub struct TARGET2_PERIOD_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> TARGET2_PERIOD_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 30)) | ((value as u32 & 1) << 30);
        self.w
    }
}
#[doc = "Field `TARGET2_WORK_EN` reader - System timer target 2 work enable."]
pub struct TARGET2_WORK_EN_R(crate::FieldReader<bool, bool>);
impl TARGET2_WORK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TARGET2_WORK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TARGET2_WORK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TARGET2_WORK_EN` writer - System timer target 2 work enable."]
pub struct TARGET2_WORK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TARGET2_WORK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 31)) | ((value as u32 & 1) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:29 - Set alarm period for system timer target 2, only valid in periodic alarms mode."]
    #[inline(always)]
    pub fn target2_period(&self) -> TARGET2_PERIOD_R {
        TARGET2_PERIOD_R::new((self.bits & 0x3fff_ffff) as u32)
    }
    #[doc = "Bit 30 - Set work mode for system timer target 2. 0: work in a timedelay alarm mode; 1: work in periodic alarms mode."]
    #[inline(always)]
    pub fn target2_period_mode(&self) -> TARGET2_PERIOD_MODE_R {
        TARGET2_PERIOD_MODE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - System timer target 2 work enable."]
    #[inline(always)]
    pub fn target2_work_en(&self) -> TARGET2_WORK_EN_R {
        TARGET2_WORK_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:29 - Set alarm period for system timer target 2, only valid in periodic alarms mode."]
    #[inline(always)]
    pub fn target2_period(&mut self) -> TARGET2_PERIOD_W {
        TARGET2_PERIOD_W { w: self }
    }
    #[doc = "Bit 30 - Set work mode for system timer target 2. 0: work in a timedelay alarm mode; 1: work in periodic alarms mode."]
    #[inline(always)]
    pub fn target2_period_mode(&mut self) -> TARGET2_PERIOD_MODE_W {
        TARGET2_PERIOD_MODE_W { w: self }
    }
    #[doc = "Bit 31 - System timer target 2 work enable."]
    #[inline(always)]
    pub fn target2_work_en(&mut self) -> TARGET2_WORK_EN_W {
        TARGET2_WORK_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configure work mode for system timer target 2\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [target2_conf]
(index.html) module"]
pub struct TARGET2_CONF_SPEC;
impl crate::RegisterSpec for TARGET2_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [target2_conf::R]
(R) reader structure"]
impl crate::Readable for TARGET2_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [target2_conf::W]
(W) writer structure"]
impl crate::Writable for TARGET2_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TARGET2_CONF to value 0"]
impl crate::Resettable for TARGET2_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}