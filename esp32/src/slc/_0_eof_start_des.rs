#[doc = "Register `_0_EOF_START_DES` reader"]
pub struct R(crate::R<_0_EOF_START_DES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<_0_EOF_START_DES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<_0_EOF_START_DES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<_0_EOF_START_DES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SLC0_EOF_START_DES_ADDR` reader - "]
pub struct SLC0_EOF_START_DES_ADDR_R(crate::FieldReader<u32, u32>);
impl SLC0_EOF_START_DES_ADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        SLC0_EOF_START_DES_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC0_EOF_START_DES_ADDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn slc0_eof_start_des_addr(&self) -> SLC0_EOF_START_DES_ADDR_R {
        SLC0_EOF_START_DES_ADDR_R::new(self.bits)
    }
}
#[doc = "\n\nThis register you can [`read`]
(crate::generic::Reg::read). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_0_eof_start_des]
(index.html) module"]
pub struct _0_EOF_START_DES_SPEC;
impl crate::RegisterSpec for _0_EOF_START_DES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [_0_eof_start_des::R]
(R) reader structure"]
impl crate::Readable for _0_EOF_START_DES_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets _0_EOF_START_DES to value 0"]
impl crate::Resettable for _0_EOF_START_DES_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}