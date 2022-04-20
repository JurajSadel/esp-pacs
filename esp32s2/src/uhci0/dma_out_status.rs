#[doc = "Register `DMA_OUT_STATUS` reader"]
pub struct R(crate::R<DMA_OUT_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_OUT_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_OUT_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_OUT_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OUT_FULL` reader - 1: DMA TX FIFO is full."]
pub struct OUT_FULL_R(crate::FieldReader<bool, bool>);
impl OUT_FULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUT_FULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_FULL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT_EMPTY` reader - 1: DMA TX FIFO is empty."]
pub struct OUT_EMPTY_R(crate::FieldReader<bool, bool>);
impl OUT_EMPTY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUT_EMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_EMPTY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - 1: DMA TX FIFO is full."]
    #[inline(always)]
    pub fn out_full(&self) -> OUT_FULL_R {
        OUT_FULL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: DMA TX FIFO is empty."]
    #[inline(always)]
    pub fn out_empty(&self) -> OUT_EMPTY_R {
        OUT_EMPTY_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "DMA data-output status register\n\nThis register you can [`read`]
(crate::generic::Reg::read). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_out_status]
(index.html) module"]
pub struct DMA_OUT_STATUS_SPEC;
impl crate::RegisterSpec for DMA_OUT_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_out_status::R]
(R) reader structure"]
impl crate::Readable for DMA_OUT_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMA_OUT_STATUS to value 0x02"]
impl crate::Resettable for DMA_OUT_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}