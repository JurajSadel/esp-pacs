#[doc = "Register `CAP_CH0` reader"]
pub struct R(crate::R<CAP_CH0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAP_CH0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAP_CH0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAP_CH0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CAP0_VALUE` reader - "]
pub type CAP0_VALUE_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cap0_value(&self) -> CAP0_VALUE_R {
        CAP0_VALUE_R::new(self.bits)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cap_ch0](index.html) module"]
pub struct CAP_CH0_SPEC;
impl crate::RegisterSpec for CAP_CH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cap_ch0::R](R) reader structure"]
impl crate::Readable for CAP_CH0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CAP_CH0 to value 0"]
impl crate::Resettable for CAP_CH0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}