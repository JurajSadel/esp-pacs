#[doc = "Register `INTERNAL_SRAM_USAGE_2` reader"]
pub struct R(crate::R<INTERNAL_SRAM_USAGE_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTERNAL_SRAM_USAGE_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTERNAL_SRAM_USAGE_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTERNAL_SRAM_USAGE_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTERNAL_SRAM_USAGE_2` writer"]
pub struct W(crate::W<INTERNAL_SRAM_USAGE_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTERNAL_SRAM_USAGE_2_SPEC>;
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
impl From<crate::W<INTERNAL_SRAM_USAGE_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTERNAL_SRAM_USAGE_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTERNAL_SRAM_CORE0_TRACE_USAGE` reader - Set 1 to someone bit means corresponding internal SRAM level can be accessed by core0 trace bus."]
pub struct INTERNAL_SRAM_CORE0_TRACE_USAGE_R(crate::FieldReader<u8, u8>);
impl INTERNAL_SRAM_CORE0_TRACE_USAGE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        INTERNAL_SRAM_CORE0_TRACE_USAGE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTERNAL_SRAM_CORE0_TRACE_USAGE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTERNAL_SRAM_CORE0_TRACE_USAGE` writer - Set 1 to someone bit means corresponding internal SRAM level can be accessed by core0 trace bus."]
pub struct INTERNAL_SRAM_CORE0_TRACE_USAGE_W<'a> {
    w: &'a mut W,
}
impl<'a> INTERNAL_SRAM_CORE0_TRACE_USAGE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
#[doc = "Field `INTERNAL_SRAM_CORE1_TRACE_USAGE` reader - Set 1 to someone bit means corresponding internal SRAM level can be accessed by core1 trace bus."]
pub struct INTERNAL_SRAM_CORE1_TRACE_USAGE_R(crate::FieldReader<u8, u8>);
impl INTERNAL_SRAM_CORE1_TRACE_USAGE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        INTERNAL_SRAM_CORE1_TRACE_USAGE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTERNAL_SRAM_CORE1_TRACE_USAGE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTERNAL_SRAM_CORE1_TRACE_USAGE` writer - Set 1 to someone bit means corresponding internal SRAM level can be accessed by core1 trace bus."]
pub struct INTERNAL_SRAM_CORE1_TRACE_USAGE_W<'a> {
    w: &'a mut W,
}
impl<'a> INTERNAL_SRAM_CORE1_TRACE_USAGE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 7)) | ((value as u32 & 0x7f) << 7);
        self.w
    }
}
#[doc = "Field `INTERNAL_SRAM_CORE0_TRACE_ALLOC` reader - Which internal SRAM bank (16KB) of 64KB can be accessed by core0 trace bus."]
pub struct INTERNAL_SRAM_CORE0_TRACE_ALLOC_R(crate::FieldReader<u8, u8>);
impl INTERNAL_SRAM_CORE0_TRACE_ALLOC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        INTERNAL_SRAM_CORE0_TRACE_ALLOC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTERNAL_SRAM_CORE0_TRACE_ALLOC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTERNAL_SRAM_CORE0_TRACE_ALLOC` writer - Which internal SRAM bank (16KB) of 64KB can be accessed by core0 trace bus."]
pub struct INTERNAL_SRAM_CORE0_TRACE_ALLOC_W<'a> {
    w: &'a mut W,
}
impl<'a> INTERNAL_SRAM_CORE0_TRACE_ALLOC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 14)) | ((value as u32 & 3) << 14);
        self.w
    }
}
#[doc = "Field `INTERNAL_SRAM_CORE1_TRACE_ALLOC` reader - Which internal SRAM bank (16KB) of 64KB can be accessed by core1 trace bus."]
pub struct INTERNAL_SRAM_CORE1_TRACE_ALLOC_R(crate::FieldReader<u8, u8>);
impl INTERNAL_SRAM_CORE1_TRACE_ALLOC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        INTERNAL_SRAM_CORE1_TRACE_ALLOC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTERNAL_SRAM_CORE1_TRACE_ALLOC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTERNAL_SRAM_CORE1_TRACE_ALLOC` writer - Which internal SRAM bank (16KB) of 64KB can be accessed by core1 trace bus."]
pub struct INTERNAL_SRAM_CORE1_TRACE_ALLOC_W<'a> {
    w: &'a mut W,
}
impl<'a> INTERNAL_SRAM_CORE1_TRACE_ALLOC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 16)) | ((value as u32 & 3) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Set 1 to someone bit means corresponding internal SRAM level can be accessed by core0 trace bus."]
    #[inline(always)]
    pub fn internal_sram_core0_trace_usage(&self) -> INTERNAL_SRAM_CORE0_TRACE_USAGE_R {
        INTERNAL_SRAM_CORE0_TRACE_USAGE_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:13 - Set 1 to someone bit means corresponding internal SRAM level can be accessed by core1 trace bus."]
    #[inline(always)]
    pub fn internal_sram_core1_trace_usage(&self) -> INTERNAL_SRAM_CORE1_TRACE_USAGE_R {
        INTERNAL_SRAM_CORE1_TRACE_USAGE_R::new(((self.bits >> 7) & 0x7f) as u8)
    }
    #[doc = "Bits 14:15 - Which internal SRAM bank (16KB) of 64KB can be accessed by core0 trace bus."]
    #[inline(always)]
    pub fn internal_sram_core0_trace_alloc(&self) -> INTERNAL_SRAM_CORE0_TRACE_ALLOC_R {
        INTERNAL_SRAM_CORE0_TRACE_ALLOC_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Which internal SRAM bank (16KB) of 64KB can be accessed by core1 trace bus."]
    #[inline(always)]
    pub fn internal_sram_core1_trace_alloc(&self) -> INTERNAL_SRAM_CORE1_TRACE_ALLOC_R {
        INTERNAL_SRAM_CORE1_TRACE_ALLOC_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Set 1 to someone bit means corresponding internal SRAM level can be accessed by core0 trace bus."]
    #[inline(always)]
    pub fn internal_sram_core0_trace_usage(&mut self) -> INTERNAL_SRAM_CORE0_TRACE_USAGE_W {
        INTERNAL_SRAM_CORE0_TRACE_USAGE_W { w: self }
    }
    #[doc = "Bits 7:13 - Set 1 to someone bit means corresponding internal SRAM level can be accessed by core1 trace bus."]
    #[inline(always)]
    pub fn internal_sram_core1_trace_usage(&mut self) -> INTERNAL_SRAM_CORE1_TRACE_USAGE_W {
        INTERNAL_SRAM_CORE1_TRACE_USAGE_W { w: self }
    }
    #[doc = "Bits 14:15 - Which internal SRAM bank (16KB) of 64KB can be accessed by core0 trace bus."]
    #[inline(always)]
    pub fn internal_sram_core0_trace_alloc(&mut self) -> INTERNAL_SRAM_CORE0_TRACE_ALLOC_W {
        INTERNAL_SRAM_CORE0_TRACE_ALLOC_W { w: self }
    }
    #[doc = "Bits 16:17 - Which internal SRAM bank (16KB) of 64KB can be accessed by core1 trace bus."]
    #[inline(always)]
    pub fn internal_sram_core1_trace_alloc(&mut self) -> INTERNAL_SRAM_CORE1_TRACE_ALLOC_W {
        INTERNAL_SRAM_CORE1_TRACE_ALLOC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal SRAM configuration register 2.\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [internal_sram_usage_2]
(index.html) module"]
pub struct INTERNAL_SRAM_USAGE_2_SPEC;
impl crate::RegisterSpec for INTERNAL_SRAM_USAGE_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [internal_sram_usage_2::R]
(R) reader structure"]
impl crate::Readable for INTERNAL_SRAM_USAGE_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [internal_sram_usage_2::W]
(W) writer structure"]
impl crate::Writable for INTERNAL_SRAM_USAGE_2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTERNAL_SRAM_USAGE_2 to value 0"]
impl crate::Resettable for INTERNAL_SRAM_USAGE_2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}