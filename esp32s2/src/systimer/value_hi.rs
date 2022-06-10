#[doc = "Register `VALUE_HI` reader"]
pub struct R(crate::R<VALUE_HI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VALUE_HI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VALUE_HI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VALUE_HI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TIMER_VALUE_HI` reader - System timer value, high 32 bits."]
pub type TIMER_VALUE_HI_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - System timer value, high 32 bits."]
    #[inline(always)]
    pub fn timer_value_hi(&self) -> TIMER_VALUE_HI_R {
        TIMER_VALUE_HI_R::new(self.bits)
    }
}
#[doc = "System timer value, high 32 bits\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [value_hi](index.html) module"]
pub struct VALUE_HI_SPEC;
impl crate::RegisterSpec for VALUE_HI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [value_hi::R](R) reader structure"]
impl crate::Readable for VALUE_HI_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets VALUE_HI to value 0"]
impl crate::Resettable for VALUE_HI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
