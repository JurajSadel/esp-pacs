#[doc = "Register `RTC_DEBUG_SEL` reader"]
pub struct R(crate::R<RTC_DEBUG_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_DEBUG_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_DEBUG_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_DEBUG_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_DEBUG_SEL` writer"]
pub struct W(crate::W<RTC_DEBUG_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_DEBUG_SEL_SPEC>;
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
impl From<crate::W<RTC_DEBUG_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_DEBUG_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC_DEBUG_SEL0` reader - configure rtc debug"]
pub struct RTC_DEBUG_SEL0_R(crate::FieldReader<u8, u8>);
impl RTC_DEBUG_SEL0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RTC_DEBUG_SEL0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_DEBUG_SEL0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_DEBUG_SEL0` writer - configure rtc debug"]
pub struct RTC_DEBUG_SEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_DEBUG_SEL0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
#[doc = "Field `RTC_DEBUG_SEL1` reader - configure rtc debug"]
pub struct RTC_DEBUG_SEL1_R(crate::FieldReader<u8, u8>);
impl RTC_DEBUG_SEL1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RTC_DEBUG_SEL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_DEBUG_SEL1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_DEBUG_SEL1` writer - configure rtc debug"]
pub struct RTC_DEBUG_SEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_DEBUG_SEL1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 5)) | ((value as u32 & 0x1f) << 5);
        self.w
    }
}
#[doc = "Field `RTC_DEBUG_SEL2` reader - configure rtc debug"]
pub struct RTC_DEBUG_SEL2_R(crate::FieldReader<u8, u8>);
impl RTC_DEBUG_SEL2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RTC_DEBUG_SEL2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_DEBUG_SEL2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_DEBUG_SEL2` writer - configure rtc debug"]
pub struct RTC_DEBUG_SEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_DEBUG_SEL2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 10)) | ((value as u32 & 0x1f) << 10);
        self.w
    }
}
#[doc = "Field `RTC_DEBUG_SEL3` reader - configure rtc debug"]
pub struct RTC_DEBUG_SEL3_R(crate::FieldReader<u8, u8>);
impl RTC_DEBUG_SEL3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RTC_DEBUG_SEL3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_DEBUG_SEL3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_DEBUG_SEL3` writer - configure rtc debug"]
pub struct RTC_DEBUG_SEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_DEBUG_SEL3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 15)) | ((value as u32 & 0x1f) << 15);
        self.w
    }
}
#[doc = "Field `RTC_DEBUG_SEL4` reader - configure rtc debug"]
pub struct RTC_DEBUG_SEL4_R(crate::FieldReader<u8, u8>);
impl RTC_DEBUG_SEL4_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RTC_DEBUG_SEL4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_DEBUG_SEL4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_DEBUG_SEL4` writer - configure rtc debug"]
pub struct RTC_DEBUG_SEL4_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_DEBUG_SEL4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 20)) | ((value as u32 & 0x1f) << 20);
        self.w
    }
}
#[doc = "Field `RTC_DEBUG_12M_NO_GATING` reader - configure rtc debug"]
pub struct RTC_DEBUG_12M_NO_GATING_R(crate::FieldReader<bool, bool>);
impl RTC_DEBUG_12M_NO_GATING_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_DEBUG_12M_NO_GATING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_DEBUG_12M_NO_GATING_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_DEBUG_12M_NO_GATING` writer - configure rtc debug"]
pub struct RTC_DEBUG_12M_NO_GATING_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_DEBUG_12M_NO_GATING_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 25)) | ((value as u32 & 1) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - configure rtc debug"]
    #[inline(always)]
    pub fn rtc_debug_sel0(&self) -> RTC_DEBUG_SEL0_R {
        RTC_DEBUG_SEL0_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - configure rtc debug"]
    #[inline(always)]
    pub fn rtc_debug_sel1(&self) -> RTC_DEBUG_SEL1_R {
        RTC_DEBUG_SEL1_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - configure rtc debug"]
    #[inline(always)]
    pub fn rtc_debug_sel2(&self) -> RTC_DEBUG_SEL2_R {
        RTC_DEBUG_SEL2_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - configure rtc debug"]
    #[inline(always)]
    pub fn rtc_debug_sel3(&self) -> RTC_DEBUG_SEL3_R {
        RTC_DEBUG_SEL3_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - configure rtc debug"]
    #[inline(always)]
    pub fn rtc_debug_sel4(&self) -> RTC_DEBUG_SEL4_R {
        RTC_DEBUG_SEL4_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bit 25 - configure rtc debug"]
    #[inline(always)]
    pub fn rtc_debug_12m_no_gating(&self) -> RTC_DEBUG_12M_NO_GATING_R {
        RTC_DEBUG_12M_NO_GATING_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - configure rtc debug"]
    #[inline(always)]
    pub fn rtc_debug_sel0(&mut self) -> RTC_DEBUG_SEL0_W {
        RTC_DEBUG_SEL0_W { w: self }
    }
    #[doc = "Bits 5:9 - configure rtc debug"]
    #[inline(always)]
    pub fn rtc_debug_sel1(&mut self) -> RTC_DEBUG_SEL1_W {
        RTC_DEBUG_SEL1_W { w: self }
    }
    #[doc = "Bits 10:14 - configure rtc debug"]
    #[inline(always)]
    pub fn rtc_debug_sel2(&mut self) -> RTC_DEBUG_SEL2_W {
        RTC_DEBUG_SEL2_W { w: self }
    }
    #[doc = "Bits 15:19 - configure rtc debug"]
    #[inline(always)]
    pub fn rtc_debug_sel3(&mut self) -> RTC_DEBUG_SEL3_W {
        RTC_DEBUG_SEL3_W { w: self }
    }
    #[doc = "Bits 20:24 - configure rtc debug"]
    #[inline(always)]
    pub fn rtc_debug_sel4(&mut self) -> RTC_DEBUG_SEL4_W {
        RTC_DEBUG_SEL4_W { w: self }
    }
    #[doc = "Bit 25 - configure rtc debug"]
    #[inline(always)]
    pub fn rtc_debug_12m_no_gating(&mut self) -> RTC_DEBUG_12M_NO_GATING_W {
        RTC_DEBUG_12M_NO_GATING_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configure rtc debug\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_debug_sel]
(index.html) module"]
pub struct RTC_DEBUG_SEL_SPEC;
impl crate::RegisterSpec for RTC_DEBUG_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_debug_sel::R]
(R) reader structure"]
impl crate::Readable for RTC_DEBUG_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_debug_sel::W]
(W) writer structure"]
impl crate::Writable for RTC_DEBUG_SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_DEBUG_SEL to value 0"]
impl crate::Resettable for RTC_DEBUG_SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}