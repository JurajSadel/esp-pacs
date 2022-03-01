#[doc = "Register `QUERY_BUSY` reader"]
pub struct R(crate::R<QUERY_BUSY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QUERY_BUSY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QUERY_BUSY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QUERY_BUSY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BUSY_STATE` reader - The state of Hmac. 1'b0: idle. 1'b1: busy."]
pub struct BUSY_STATE_R(crate::FieldReader<bool, bool>);
impl BUSY_STATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BUSY_STATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUSY_STATE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - The state of Hmac. 1'b0: idle. 1'b1: busy."]
    #[inline(always)]
    pub fn busy_state(&self) -> BUSY_STATE_R {
        BUSY_STATE_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "The busy state of HMAC module\n\nThis register you can [`read`]
(crate::generic::Reg::read). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [query_busy]
(index.html) module"]
pub struct QUERY_BUSY_SPEC;
impl crate::RegisterSpec for QUERY_BUSY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [query_busy::R]
(R) reader structure"]
impl crate::Readable for QUERY_BUSY_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets QUERY_BUSY to value 0"]
impl crate::Resettable for QUERY_BUSY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}