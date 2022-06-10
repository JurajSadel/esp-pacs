#[doc = "Register `VALUE_LO` reader"]
pub struct R(crate::R<VALUE_LO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VALUE_LO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VALUE_LO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VALUE_LO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TIMER_VALUE_LO` reader - System timer value, low 32 bits."]
pub type TIMER_VALUE_LO_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - System timer value, low 32 bits."]
    #[inline(always)]
    pub fn timer_value_lo(&self) -> TIMER_VALUE_LO_R {
        TIMER_VALUE_LO_R::new(self.bits)
    }
}
#[doc = "System timer value, low 32 bits\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [value_lo](index.html) module"]
pub struct VALUE_LO_SPEC;
impl crate::RegisterSpec for VALUE_LO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [value_lo::R](R) reader structure"]
impl crate::Readable for VALUE_LO_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets VALUE_LO to value 0"]
impl crate::Resettable for VALUE_LO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
