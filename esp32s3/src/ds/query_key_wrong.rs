#[doc = "Register `QUERY_KEY_WRONG` reader"]
pub struct R(crate::R<QUERY_KEY_WRONG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QUERY_KEY_WRONG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QUERY_KEY_WRONG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QUERY_KEY_WRONG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `QUERY_KEY_WRONG` reader - 1-15: HMAC was activated, but the DS peripheral did not successfully receive the DS_KEY from the HMAC peripheral. (The biggest value is 15). 0: HMAC is not activated."]
pub struct QUERY_KEY_WRONG_R(crate::FieldReader<u8, u8>);
impl QUERY_KEY_WRONG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        QUERY_KEY_WRONG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for QUERY_KEY_WRONG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - 1-15: HMAC was activated, but the DS peripheral did not successfully receive the DS_KEY from the HMAC peripheral. (The biggest value is 15). 0: HMAC is not activated."]
    #[inline(always)]
    pub fn query_key_wrong(&self) -> QUERY_KEY_WRONG_R {
        QUERY_KEY_WRONG_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "Checks the reason why DS_KEY is not ready\n\nThis register you can [`read`]
(crate::generic::Reg::read). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [query_key_wrong]
(index.html) module"]
pub struct QUERY_KEY_WRONG_SPEC;
impl crate::RegisterSpec for QUERY_KEY_WRONG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [query_key_wrong::R]
(R) reader structure"]
impl crate::Readable for QUERY_KEY_WRONG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets QUERY_KEY_WRONG to value 0"]
impl crate::Resettable for QUERY_KEY_WRONG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}