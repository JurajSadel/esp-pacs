#[doc = "Register `INTERNAL_SRAM_USAGE_1` reader"]
pub struct R(crate::R<INTERNAL_SRAM_USAGE_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTERNAL_SRAM_USAGE_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTERNAL_SRAM_USAGE_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTERNAL_SRAM_USAGE_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTERNAL_SRAM_USAGE_1` writer"]
pub struct W(crate::W<INTERNAL_SRAM_USAGE_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTERNAL_SRAM_USAGE_1_SPEC>;
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
impl From<crate::W<INTERNAL_SRAM_USAGE_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTERNAL_SRAM_USAGE_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTERNAL_SRAM_ICACHE_USAGE` reader - Set 1 to someone bit means corresponding internal SRAM level can be accessed by icache."]
pub struct INTERNAL_SRAM_ICACHE_USAGE_R(crate::FieldReader<u8, u8>);
impl INTERNAL_SRAM_ICACHE_USAGE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        INTERNAL_SRAM_ICACHE_USAGE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTERNAL_SRAM_ICACHE_USAGE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTERNAL_SRAM_ICACHE_USAGE` writer - Set 1 to someone bit means corresponding internal SRAM level can be accessed by icache."]
pub struct INTERNAL_SRAM_ICACHE_USAGE_W<'a> {
    w: &'a mut W,
}
impl<'a> INTERNAL_SRAM_ICACHE_USAGE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !3) | (value as u32 & 3);
        self.w
    }
}
#[doc = "Field `INTERNAL_SRAM_DCACHE_USAGE` reader - Set 1 to someone bit means corresponding internal SRAM level can be accessed by dcache."]
pub struct INTERNAL_SRAM_DCACHE_USAGE_R(crate::FieldReader<u8, u8>);
impl INTERNAL_SRAM_DCACHE_USAGE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        INTERNAL_SRAM_DCACHE_USAGE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTERNAL_SRAM_DCACHE_USAGE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTERNAL_SRAM_DCACHE_USAGE` writer - Set 1 to someone bit means corresponding internal SRAM level can be accessed by dcache."]
pub struct INTERNAL_SRAM_DCACHE_USAGE_W<'a> {
    w: &'a mut W,
}
impl<'a> INTERNAL_SRAM_DCACHE_USAGE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 2)) | ((value as u32 & 3) << 2);
        self.w
    }
}
#[doc = "Field `INTERNAL_SRAM_CPU_USAGE` reader - Set 1 to someone bit means corresponding internal SRAM level can be accessed by cpu."]
pub struct INTERNAL_SRAM_CPU_USAGE_R(crate::FieldReader<u8, u8>);
impl INTERNAL_SRAM_CPU_USAGE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        INTERNAL_SRAM_CPU_USAGE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTERNAL_SRAM_CPU_USAGE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTERNAL_SRAM_CPU_USAGE` writer - Set 1 to someone bit means corresponding internal SRAM level can be accessed by cpu."]
pub struct INTERNAL_SRAM_CPU_USAGE_W<'a> {
    w: &'a mut W,
}
impl<'a> INTERNAL_SRAM_CPU_USAGE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 4)) | ((value as u32 & 0x7f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Set 1 to someone bit means corresponding internal SRAM level can be accessed by icache."]
    #[inline(always)]
    pub fn internal_sram_icache_usage(&self) -> INTERNAL_SRAM_ICACHE_USAGE_R {
        INTERNAL_SRAM_ICACHE_USAGE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Set 1 to someone bit means corresponding internal SRAM level can be accessed by dcache."]
    #[inline(always)]
    pub fn internal_sram_dcache_usage(&self) -> INTERNAL_SRAM_DCACHE_USAGE_R {
        INTERNAL_SRAM_DCACHE_USAGE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:10 - Set 1 to someone bit means corresponding internal SRAM level can be accessed by cpu."]
    #[inline(always)]
    pub fn internal_sram_cpu_usage(&self) -> INTERNAL_SRAM_CPU_USAGE_R {
        INTERNAL_SRAM_CPU_USAGE_R::new(((self.bits >> 4) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Set 1 to someone bit means corresponding internal SRAM level can be accessed by icache."]
    #[inline(always)]
    pub fn internal_sram_icache_usage(&mut self) -> INTERNAL_SRAM_ICACHE_USAGE_W {
        INTERNAL_SRAM_ICACHE_USAGE_W { w: self }
    }
    #[doc = "Bits 2:3 - Set 1 to someone bit means corresponding internal SRAM level can be accessed by dcache."]
    #[inline(always)]
    pub fn internal_sram_dcache_usage(&mut self) -> INTERNAL_SRAM_DCACHE_USAGE_W {
        INTERNAL_SRAM_DCACHE_USAGE_W { w: self }
    }
    #[doc = "Bits 4:10 - Set 1 to someone bit means corresponding internal SRAM level can be accessed by cpu."]
    #[inline(always)]
    pub fn internal_sram_cpu_usage(&mut self) -> INTERNAL_SRAM_CPU_USAGE_W {
        INTERNAL_SRAM_CPU_USAGE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal SRAM configuration register 1.\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [internal_sram_usage_1]
(index.html) module"]
pub struct INTERNAL_SRAM_USAGE_1_SPEC;
impl crate::RegisterSpec for INTERNAL_SRAM_USAGE_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [internal_sram_usage_1::R]
(R) reader structure"]
impl crate::Readable for INTERNAL_SRAM_USAGE_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [internal_sram_usage_1::W]
(W) writer structure"]
impl crate::Writable for INTERNAL_SRAM_USAGE_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTERNAL_SRAM_USAGE_1 to value 0x07ff"]
impl crate::Resettable for INTERNAL_SRAM_USAGE_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x07ff
    }
}