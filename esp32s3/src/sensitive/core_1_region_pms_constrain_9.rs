#[doc = "Register `CORE_1_REGION_PMS_CONSTRAIN_9` reader"]
pub struct R(crate::R<CORE_1_REGION_PMS_CONSTRAIN_9_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_1_REGION_PMS_CONSTRAIN_9_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_1_REGION_PMS_CONSTRAIN_9_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_1_REGION_PMS_CONSTRAIN_9_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORE_1_REGION_PMS_CONSTRAIN_9` writer"]
pub struct W(crate::W<CORE_1_REGION_PMS_CONSTRAIN_9_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_1_REGION_PMS_CONSTRAIN_9_SPEC>;
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
impl From<crate::W<CORE_1_REGION_PMS_CONSTRAIN_9_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_1_REGION_PMS_CONSTRAIN_9_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_1_REGION_PMS_CONSTRAIN_ADDR_6` reader - Region 5 end address and Region 6 start address for core1."]
pub struct CORE_1_REGION_PMS_CONSTRAIN_ADDR_6_R(crate::FieldReader<u32, u32>);
impl CORE_1_REGION_PMS_CONSTRAIN_ADDR_6_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CORE_1_REGION_PMS_CONSTRAIN_ADDR_6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_1_REGION_PMS_CONSTRAIN_ADDR_6_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_1_REGION_PMS_CONSTRAIN_ADDR_6` writer - Region 5 end address and Region 6 start address for core1."]
pub struct CORE_1_REGION_PMS_CONSTRAIN_ADDR_6_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_1_REGION_PMS_CONSTRAIN_ADDR_6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff_ffff) | (value as u32 & 0x3fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:29 - Region 5 end address and Region 6 start address for core1."]
    #[inline(always)]
    pub fn core_1_region_pms_constrain_addr_6(&self) -> CORE_1_REGION_PMS_CONSTRAIN_ADDR_6_R {
        CORE_1_REGION_PMS_CONSTRAIN_ADDR_6_R::new((self.bits & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:29 - Region 5 end address and Region 6 start address for core1."]
    #[inline(always)]
    pub fn core_1_region_pms_constrain_addr_6(&mut self) -> CORE_1_REGION_PMS_CONSTRAIN_ADDR_6_W {
        CORE_1_REGION_PMS_CONSTRAIN_ADDR_6_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "core1 region permission register 9.\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_1_region_pms_constrain_9]
(index.html) module"]
pub struct CORE_1_REGION_PMS_CONSTRAIN_9_SPEC;
impl crate::RegisterSpec for CORE_1_REGION_PMS_CONSTRAIN_9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_1_region_pms_constrain_9::R]
(R) reader structure"]
impl crate::Readable for CORE_1_REGION_PMS_CONSTRAIN_9_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_1_region_pms_constrain_9::W]
(W) writer structure"]
impl crate::Writable for CORE_1_REGION_PMS_CONSTRAIN_9_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CORE_1_REGION_PMS_CONSTRAIN_9 to value 0"]
impl crate::Resettable for CORE_1_REGION_PMS_CONSTRAIN_9_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}