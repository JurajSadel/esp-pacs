#[doc = "Register `PCPU_INT1` reader"]
pub struct R(crate::R<PCPU_INT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCPU_INT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCPU_INT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCPU_INT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PROCPU_INT_H` reader - GPIO32~39 PRO CPU interrupt status"]
pub struct PROCPU_INT_H_R(crate::FieldReader<u8, u8>);
impl PROCPU_INT_H_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PROCPU_INT_H_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROCPU_INT_H_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - GPIO32~39 PRO CPU interrupt status"]
    #[inline(always)]
    pub fn procpu_int_h(&self) -> PROCPU_INT_H_R {
        PROCPU_INT_H_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "\n\nThis register you can [`read`]
(crate::generic::Reg::read). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcpu_int1]
(index.html) module"]
pub struct PCPU_INT1_SPEC;
impl crate::RegisterSpec for PCPU_INT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcpu_int1::R]
(R) reader structure"]
impl crate::Readable for PCPU_INT1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PCPU_INT1 to value 0"]
impl crate::Resettable for PCPU_INT1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}