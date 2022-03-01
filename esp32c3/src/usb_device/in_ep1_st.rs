#[doc = "Register `IN_EP1_ST` reader"]
pub struct R(crate::R<IN_EP1_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IN_EP1_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IN_EP1_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IN_EP1_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IN_EP1_STATE` reader - State of IN Endpoint 1."]
pub struct IN_EP1_STATE_R(crate::FieldReader<u8, u8>);
impl IN_EP1_STATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IN_EP1_STATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN_EP1_STATE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN_EP1_WR_ADDR` reader - Write data address of IN endpoint 1."]
pub struct IN_EP1_WR_ADDR_R(crate::FieldReader<u8, u8>);
impl IN_EP1_WR_ADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IN_EP1_WR_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN_EP1_WR_ADDR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN_EP1_RD_ADDR` reader - Read data address of IN endpoint 1."]
pub struct IN_EP1_RD_ADDR_R(crate::FieldReader<u8, u8>);
impl IN_EP1_RD_ADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IN_EP1_RD_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN_EP1_RD_ADDR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:1 - State of IN Endpoint 1."]
    #[inline(always)]
    pub fn in_ep1_state(&self) -> IN_EP1_STATE_R {
        IN_EP1_STATE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:8 - Write data address of IN endpoint 1."]
    #[inline(always)]
    pub fn in_ep1_wr_addr(&self) -> IN_EP1_WR_ADDR_R {
        IN_EP1_WR_ADDR_R::new(((self.bits >> 2) & 0x7f) as u8)
    }
    #[doc = "Bits 9:15 - Read data address of IN endpoint 1."]
    #[inline(always)]
    pub fn in_ep1_rd_addr(&self) -> IN_EP1_RD_ADDR_R {
        IN_EP1_RD_ADDR_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
}
#[doc = "USB_DEVICE_IN_EP1_ST_REG.\n\nThis register you can [`read`]
(crate::generic::Reg::read). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [in_ep1_st]
(index.html) module"]
pub struct IN_EP1_ST_SPEC;
impl crate::RegisterSpec for IN_EP1_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [in_ep1_st::R]
(R) reader structure"]
impl crate::Readable for IN_EP1_ST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IN_EP1_ST to value 0x01"]
impl crate::Resettable for IN_EP1_ST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}