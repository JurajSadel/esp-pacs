#[doc = "Register `STATE` reader"]
pub struct R(crate::R<STATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TX_IDLE` reader - "]
pub struct TX_IDLE_R(crate::FieldReader<bool, bool>);
impl TX_IDLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_IDLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_IDLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_FIFO_RESET_BACK` reader - "]
pub struct TX_FIFO_RESET_BACK_R(crate::FieldReader<bool, bool>);
impl TX_FIFO_RESET_BACK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_FIFO_RESET_BACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_FIFO_RESET_BACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_FIFO_RESET_BACK` reader - "]
pub struct RX_FIFO_RESET_BACK_R(crate::FieldReader<bool, bool>);
impl RX_FIFO_RESET_BACK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_FIFO_RESET_BACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_FIFO_RESET_BACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tx_idle(&self) -> TX_IDLE_R {
        TX_IDLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tx_fifo_reset_back(&self) -> TX_FIFO_RESET_BACK_R {
        TX_FIFO_RESET_BACK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rx_fifo_reset_back(&self) -> RX_FIFO_RESET_BACK_R {
        RX_FIFO_RESET_BACK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
#[doc = "\n\nThis register you can [`read`]
(crate::generic::Reg::read). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [state]
(index.html) module"]
pub struct STATE_SPEC;
impl crate::RegisterSpec for STATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [state::R]
(R) reader structure"]
impl crate::Readable for STATE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATE to value 0x07"]
impl crate::Resettable for STATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x07
    }
}