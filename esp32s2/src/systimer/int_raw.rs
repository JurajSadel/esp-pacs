#[doc = "Register `INT_RAW` reader"]
pub struct R(crate::R<INT_RAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_RAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_RAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_RAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INT0_RAW` reader - Interrupt raw bit of system timer target 0."]
pub struct INT0_RAW_R(crate::FieldReader<bool, bool>);
impl INT0_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INT0_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT0_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT1_RAW` reader - Interrupt raw bit of system timer target 1."]
pub struct INT1_RAW_R(crate::FieldReader<bool, bool>);
impl INT1_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INT1_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT1_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT2_RAW` reader - Interrupt raw bit of system timer target 2."]
pub struct INT2_RAW_R(crate::FieldReader<bool, bool>);
impl INT2_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INT2_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT2_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt raw bit of system timer target 0."]
    #[inline(always)]
    pub fn int0_raw(&self) -> INT0_RAW_R {
        INT0_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt raw bit of system timer target 1."]
    #[inline(always)]
    pub fn int1_raw(&self) -> INT1_RAW_R {
        INT1_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt raw bit of system timer target 2."]
    #[inline(always)]
    pub fn int2_raw(&self) -> INT2_RAW_R {
        INT2_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "System timer interrupt raw\n\nThis register you can [`read`]
(crate::generic::Reg::read). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_raw]
(index.html) module"]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_raw::R]
(R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}