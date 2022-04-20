#[doc = "Register `APB_PERIPHERAL_STATUS` reader"]
pub struct R(crate::R<APB_PERIPHERAL_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB_PERIPHERAL_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB_PERIPHERAL_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB_PERIPHERAL_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `APB_PERI_BYTE_ERROR_ADDR` reader - Record the illegitimate address of APB peripheral."]
pub struct APB_PERI_BYTE_ERROR_ADDR_R(crate::FieldReader<u32, u32>);
impl APB_PERI_BYTE_ERROR_ADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        APB_PERI_BYTE_ERROR_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB_PERI_BYTE_ERROR_ADDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Record the illegitimate address of APB peripheral."]
    #[inline(always)]
    pub fn apb_peri_byte_error_addr(&self) -> APB_PERI_BYTE_ERROR_ADDR_R {
        APB_PERI_BYTE_ERROR_ADDR_R::new(self.bits)
    }
}
#[doc = "PeribBus2 peripheral access status register.\n\nThis register you can [`read`]
(crate::generic::Reg::read). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_peripheral_status]
(index.html) module"]
pub struct APB_PERIPHERAL_STATUS_SPEC;
impl crate::RegisterSpec for APB_PERIPHERAL_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb_peripheral_status::R]
(R) reader structure"]
impl crate::Readable for APB_PERIPHERAL_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets APB_PERIPHERAL_STATUS to value 0"]
impl crate::Resettable for APB_PERIPHERAL_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}