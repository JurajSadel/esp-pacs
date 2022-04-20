#[doc = "Register `TIMER_SYNCI_CFG` reader"]
pub struct R(crate::R<TIMER_SYNCI_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER_SYNCI_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER_SYNCI_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER_SYNCI_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER_SYNCI_CFG` writer"]
pub struct W(crate::W<TIMER_SYNCI_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER_SYNCI_CFG_SPEC>;
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
impl From<crate::W<TIMER_SYNCI_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER_SYNCI_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER0_SYNCISEL` reader - "]
pub struct TIMER0_SYNCISEL_R(crate::FieldReader<u8, u8>);
impl TIMER0_SYNCISEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TIMER0_SYNCISEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER0_SYNCISEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER0_SYNCISEL` writer - "]
pub struct TIMER0_SYNCISEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER0_SYNCISEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !7) | (value as u32 & 7);
        self.w
    }
}
#[doc = "Field `TIMER1_SYNCISEL` reader - "]
pub struct TIMER1_SYNCISEL_R(crate::FieldReader<u8, u8>);
impl TIMER1_SYNCISEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TIMER1_SYNCISEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER1_SYNCISEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER1_SYNCISEL` writer - "]
pub struct TIMER1_SYNCISEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER1_SYNCISEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 3)) | ((value as u32 & 7) << 3);
        self.w
    }
}
#[doc = "Field `TIMER2_SYNCISEL` reader - "]
pub struct TIMER2_SYNCISEL_R(crate::FieldReader<u8, u8>);
impl TIMER2_SYNCISEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TIMER2_SYNCISEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER2_SYNCISEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER2_SYNCISEL` writer - "]
pub struct TIMER2_SYNCISEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER2_SYNCISEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 6)) | ((value as u32 & 7) << 6);
        self.w
    }
}
#[doc = "Field `EXTERNAL_SYNCI0_INVERT` reader - "]
pub struct EXTERNAL_SYNCI0_INVERT_R(crate::FieldReader<bool, bool>);
impl EXTERNAL_SYNCI0_INVERT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXTERNAL_SYNCI0_INVERT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTERNAL_SYNCI0_INVERT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTERNAL_SYNCI0_INVERT` writer - "]
pub struct EXTERNAL_SYNCI0_INVERT_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTERNAL_SYNCI0_INVERT_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 9)) | ((value as u32 & 1) << 9);
        self.w
    }
}
#[doc = "Field `EXTERNAL_SYNCI1_INVERT` reader - "]
pub struct EXTERNAL_SYNCI1_INVERT_R(crate::FieldReader<bool, bool>);
impl EXTERNAL_SYNCI1_INVERT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXTERNAL_SYNCI1_INVERT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTERNAL_SYNCI1_INVERT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTERNAL_SYNCI1_INVERT` writer - "]
pub struct EXTERNAL_SYNCI1_INVERT_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTERNAL_SYNCI1_INVERT_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 10)) | ((value as u32 & 1) << 10);
        self.w
    }
}
#[doc = "Field `EXTERNAL_SYNCI2_INVERT` reader - "]
pub struct EXTERNAL_SYNCI2_INVERT_R(crate::FieldReader<bool, bool>);
impl EXTERNAL_SYNCI2_INVERT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXTERNAL_SYNCI2_INVERT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTERNAL_SYNCI2_INVERT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTERNAL_SYNCI2_INVERT` writer - "]
pub struct EXTERNAL_SYNCI2_INVERT_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTERNAL_SYNCI2_INVERT_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 11)) | ((value as u32 & 1) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn timer0_syncisel(&self) -> TIMER0_SYNCISEL_R {
        TIMER0_SYNCISEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn timer1_syncisel(&self) -> TIMER1_SYNCISEL_R {
        TIMER1_SYNCISEL_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8"]
    #[inline(always)]
    pub fn timer2_syncisel(&self) -> TIMER2_SYNCISEL_R {
        TIMER2_SYNCISEL_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn external_synci0_invert(&self) -> EXTERNAL_SYNCI0_INVERT_R {
        EXTERNAL_SYNCI0_INVERT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn external_synci1_invert(&self) -> EXTERNAL_SYNCI1_INVERT_R {
        EXTERNAL_SYNCI1_INVERT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn external_synci2_invert(&self) -> EXTERNAL_SYNCI2_INVERT_R {
        EXTERNAL_SYNCI2_INVERT_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn timer0_syncisel(&mut self) -> TIMER0_SYNCISEL_W {
        TIMER0_SYNCISEL_W { w: self }
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn timer1_syncisel(&mut self) -> TIMER1_SYNCISEL_W {
        TIMER1_SYNCISEL_W { w: self }
    }
    #[doc = "Bits 6:8"]
    #[inline(always)]
    pub fn timer2_syncisel(&mut self) -> TIMER2_SYNCISEL_W {
        TIMER2_SYNCISEL_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn external_synci0_invert(&mut self) -> EXTERNAL_SYNCI0_INVERT_W {
        EXTERNAL_SYNCI0_INVERT_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn external_synci1_invert(&mut self) -> EXTERNAL_SYNCI1_INVERT_W {
        EXTERNAL_SYNCI1_INVERT_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn external_synci2_invert(&mut self) -> EXTERNAL_SYNCI2_INVERT_W {
        EXTERNAL_SYNCI2_INVERT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer_synci_cfg]
(index.html) module"]
pub struct TIMER_SYNCI_CFG_SPEC;
impl crate::RegisterSpec for TIMER_SYNCI_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer_synci_cfg::R]
(R) reader structure"]
impl crate::Readable for TIMER_SYNCI_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer_synci_cfg::W]
(W) writer structure"]
impl crate::Writable for TIMER_SYNCI_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMER_SYNCI_CFG to value 0"]
impl crate::Resettable for TIMER_SYNCI_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}