#[doc = "Register `CACHE_CORE1_ACS_INT_MAP` reader"]
pub struct R(crate::R<CACHE_CORE1_ACS_INT_MAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CACHE_CORE1_ACS_INT_MAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CACHE_CORE1_ACS_INT_MAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CACHE_CORE1_ACS_INT_MAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CACHE_CORE1_ACS_INT_MAP` writer"]
pub struct W(crate::W<CACHE_CORE1_ACS_INT_MAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CACHE_CORE1_ACS_INT_MAP_SPEC>;
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
impl From<crate::W<CACHE_CORE1_ACS_INT_MAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CACHE_CORE1_ACS_INT_MAP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CACHE_CORE1_ACS_INT_MAP` reader - this register used to map cache_core1_acs interrupt to one of core1's external interrupt"]
pub struct CACHE_CORE1_ACS_INT_MAP_R(crate::FieldReader<u8, u8>);
impl CACHE_CORE1_ACS_INT_MAP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CACHE_CORE1_ACS_INT_MAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CACHE_CORE1_ACS_INT_MAP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CACHE_CORE1_ACS_INT_MAP` writer - this register used to map cache_core1_acs interrupt to one of core1's external interrupt"]
pub struct CACHE_CORE1_ACS_INT_MAP_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHE_CORE1_ACS_INT_MAP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - this register used to map cache_core1_acs interrupt to one of core1's external interrupt"]
    #[inline(always)]
    pub fn cache_core1_acs_int_map(&self) -> CACHE_CORE1_ACS_INT_MAP_R {
        CACHE_CORE1_ACS_INT_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - this register used to map cache_core1_acs interrupt to one of core1's external interrupt"]
    #[inline(always)]
    pub fn cache_core1_acs_int_map(&mut self) -> CACHE_CORE1_ACS_INT_MAP_W {
        CACHE_CORE1_ACS_INT_MAP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "cache_core1_acs interrupt configuration register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cache_core1_acs_int_map]
(index.html) module"]
pub struct CACHE_CORE1_ACS_INT_MAP_SPEC;
impl crate::RegisterSpec for CACHE_CORE1_ACS_INT_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cache_core1_acs_int_map::R]
(R) reader structure"]
impl crate::Readable for CACHE_CORE1_ACS_INT_MAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cache_core1_acs_int_map::W]
(W) writer structure"]
impl crate::Writable for CACHE_CORE1_ACS_INT_MAP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CACHE_CORE1_ACS_INT_MAP to value 0x10"]
impl crate::Resettable for CACHE_CORE1_ACS_INT_MAP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}