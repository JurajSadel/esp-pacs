#[doc = "Register `RD_USR_DATA4` reader"]
pub struct R(crate::R<RD_USR_DATA4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_USR_DATA4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_USR_DATA4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_USR_DATA4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `USR_DATA4` reader - Stores the fourth 32 bits of BLOCK3 (user)."]
pub type USR_DATA4_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Stores the fourth 32 bits of BLOCK3 (user)."]
    #[inline(always)]
    pub fn usr_data4(&self) -> USR_DATA4_R {
        USR_DATA4_R::new(self.bits)
    }
}
#[doc = "Register 4 of BLOCK3 (user).\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_usr_data4](index.html) module"]
pub struct RD_USR_DATA4_SPEC;
impl crate::RegisterSpec for RD_USR_DATA4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_usr_data4::R](R) reader structure"]
impl crate::Readable for RD_USR_DATA4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RD_USR_DATA4 to value 0"]
impl crate::Resettable for RD_USR_DATA4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}