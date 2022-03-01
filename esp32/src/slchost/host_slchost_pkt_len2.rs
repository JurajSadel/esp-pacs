#[doc = "Register `HOST_SLCHOST_PKT_LEN2` reader"]
pub struct R(crate::R<HOST_SLCHOST_PKT_LEN2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOST_SLCHOST_PKT_LEN2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOST_SLCHOST_PKT_LEN2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOST_SLCHOST_PKT_LEN2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HOST_HOSTSLC0_LEN2` reader - "]
pub struct HOST_HOSTSLC0_LEN2_R(crate::FieldReader<u32, u32>);
impl HOST_HOSTSLC0_LEN2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        HOST_HOSTSLC0_LEN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOST_HOSTSLC0_LEN2_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn host_hostslc0_len2(&self) -> HOST_HOSTSLC0_LEN2_R {
        HOST_HOSTSLC0_LEN2_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
#[doc = "\n\nThis register you can [`read`]
(crate::generic::Reg::read). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_slchost_pkt_len2]
(index.html) module"]
pub struct HOST_SLCHOST_PKT_LEN2_SPEC;
impl crate::RegisterSpec for HOST_SLCHOST_PKT_LEN2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [host_slchost_pkt_len2::R]
(R) reader structure"]
impl crate::Readable for HOST_SLCHOST_PKT_LEN2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HOST_SLCHOST_PKT_LEN2 to value 0"]
impl crate::Resettable for HOST_SLCHOST_PKT_LEN2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}