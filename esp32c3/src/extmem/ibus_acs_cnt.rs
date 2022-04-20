#[doc = "Register `IBUS_ACS_CNT` reader"]
pub struct R(crate::R<IBUS_ACS_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IBUS_ACS_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IBUS_ACS_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IBUS_ACS_CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IBUS_ACS_CNT` reader - The bits are used to count the number of ibus access flash through icache."]
pub struct IBUS_ACS_CNT_R(crate::FieldReader<u32, u32>);
impl IBUS_ACS_CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        IBUS_ACS_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IBUS_ACS_CNT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - The bits are used to count the number of ibus access flash through icache."]
    #[inline(always)]
    pub fn ibus_acs_cnt(&self) -> IBUS_ACS_CNT_R {
        IBUS_ACS_CNT_R::new(self.bits)
    }
}
#[doc = "This description will be updated in the near future.\n\nThis register you can [`read`]
(crate::generic::Reg::read). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ibus_acs_cnt]
(index.html) module"]
pub struct IBUS_ACS_CNT_SPEC;
impl crate::RegisterSpec for IBUS_ACS_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ibus_acs_cnt::R]
(R) reader structure"]
impl crate::Readable for IBUS_ACS_CNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IBUS_ACS_CNT to value 0"]
impl crate::Resettable for IBUS_ACS_CNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}