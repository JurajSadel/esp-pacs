#[doc = "Register `INT_ST` reader"]
pub struct R(crate::R<INT_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CH0_TX_END_INT_ST` reader - The masked interrupt status bit for CH0_TX_END_INT."]
pub struct CH0_TX_END_INT_ST_R(crate::FieldReader<bool, bool>);
impl CH0_TX_END_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH0_TX_END_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH0_TX_END_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH0_RX_END_INT_ST` reader - The masked interrupt status bit for CH0_RX_END_INT."]
pub struct CH0_RX_END_INT_ST_R(crate::FieldReader<bool, bool>);
impl CH0_RX_END_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH0_RX_END_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH0_RX_END_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH0_ERR_INT_ST` reader - The masked interrupt status bit for CH0_ERR_INT."]
pub struct CH0_ERR_INT_ST_R(crate::FieldReader<bool, bool>);
impl CH0_ERR_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH0_ERR_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH0_ERR_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH1_TX_END_INT_ST` reader - The masked interrupt status bit for CH1_TX_END_INT."]
pub struct CH1_TX_END_INT_ST_R(crate::FieldReader<bool, bool>);
impl CH1_TX_END_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH1_TX_END_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH1_TX_END_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH1_RX_END_INT_ST` reader - The masked interrupt status bit for CH1_RX_END_INT."]
pub struct CH1_RX_END_INT_ST_R(crate::FieldReader<bool, bool>);
impl CH1_RX_END_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH1_RX_END_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH1_RX_END_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH1_ERR_INT_ST` reader - The masked interrupt status bit for CH1_ERR_INT."]
pub struct CH1_ERR_INT_ST_R(crate::FieldReader<bool, bool>);
impl CH1_ERR_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH1_ERR_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH1_ERR_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH2_TX_END_INT_ST` reader - The masked interrupt status bit for CH2_TX_END_INT."]
pub struct CH2_TX_END_INT_ST_R(crate::FieldReader<bool, bool>);
impl CH2_TX_END_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH2_TX_END_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH2_TX_END_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH2_RX_END_INT_ST` reader - The masked interrupt status bit for CH2_RX_END_INT."]
pub struct CH2_RX_END_INT_ST_R(crate::FieldReader<bool, bool>);
impl CH2_RX_END_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH2_RX_END_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH2_RX_END_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH2_ERR_INT_ST` reader - The masked interrupt status bit for CH2_ERR_INT."]
pub struct CH2_ERR_INT_ST_R(crate::FieldReader<bool, bool>);
impl CH2_ERR_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH2_ERR_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH2_ERR_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH3_TX_END_INT_ST` reader - The masked interrupt status bit for CH3_TX_END_INT."]
pub struct CH3_TX_END_INT_ST_R(crate::FieldReader<bool, bool>);
impl CH3_TX_END_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH3_TX_END_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH3_TX_END_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH3_RX_END_INT_ST` reader - The masked interrupt status bit for CH3_RX_END_INT."]
pub struct CH3_RX_END_INT_ST_R(crate::FieldReader<bool, bool>);
impl CH3_RX_END_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH3_RX_END_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH3_RX_END_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH3_ERR_INT_ST` reader - The masked interrupt status bit for CH3_ERR_INT."]
pub struct CH3_ERR_INT_ST_R(crate::FieldReader<bool, bool>);
impl CH3_ERR_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH3_ERR_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH3_ERR_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH0_TX_THR_EVENT_INT_ST` reader - The masked interrupt status bit for CH0_TX_THR_EVENT_INT."]
pub struct CH0_TX_THR_EVENT_INT_ST_R(crate::FieldReader<bool, bool>);
impl CH0_TX_THR_EVENT_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH0_TX_THR_EVENT_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH0_TX_THR_EVENT_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH1_TX_THR_EVENT_INT_ST` reader - The masked interrupt status bit for CH1_TX_THR_EVENT_INT."]
pub struct CH1_TX_THR_EVENT_INT_ST_R(crate::FieldReader<bool, bool>);
impl CH1_TX_THR_EVENT_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH1_TX_THR_EVENT_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH1_TX_THR_EVENT_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH2_TX_THR_EVENT_INT_ST` reader - The masked interrupt status bit for CH2_TX_THR_EVENT_INT."]
pub struct CH2_TX_THR_EVENT_INT_ST_R(crate::FieldReader<bool, bool>);
impl CH2_TX_THR_EVENT_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH2_TX_THR_EVENT_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH2_TX_THR_EVENT_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH3_TX_THR_EVENT_INT_ST` reader - The masked interrupt status bit for CH3_TX_THR_EVENT_INT."]
pub struct CH3_TX_THR_EVENT_INT_ST_R(crate::FieldReader<bool, bool>);
impl CH3_TX_THR_EVENT_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH3_TX_THR_EVENT_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH3_TX_THR_EVENT_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH0_TX_LOOP_INT_ST` reader - The masked interrupt status bit for CH0_TX_LOOP_INT."]
pub struct CH0_TX_LOOP_INT_ST_R(crate::FieldReader<bool, bool>);
impl CH0_TX_LOOP_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH0_TX_LOOP_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH0_TX_LOOP_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH1_TX_LOOP_INT_ST` reader - The masked interrupt status bit for CH1_TX_LOOP_INT."]
pub struct CH1_TX_LOOP_INT_ST_R(crate::FieldReader<bool, bool>);
impl CH1_TX_LOOP_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH1_TX_LOOP_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH1_TX_LOOP_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH2_TX_LOOP_INT_ST` reader - The masked interrupt status bit for CH2_TX_LOOP_INT."]
pub struct CH2_TX_LOOP_INT_ST_R(crate::FieldReader<bool, bool>);
impl CH2_TX_LOOP_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH2_TX_LOOP_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH2_TX_LOOP_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH3_TX_LOOP_INT_ST` reader - The masked interrupt status bit for CH3_TX_LOOP_INT."]
pub struct CH3_TX_LOOP_INT_ST_R(crate::FieldReader<bool, bool>);
impl CH3_TX_LOOP_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH3_TX_LOOP_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH3_TX_LOOP_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - The masked interrupt status bit for CH0_TX_END_INT."]
    #[inline(always)]
    pub fn ch0_tx_end_int_st(&self) -> CH0_TX_END_INT_ST_R {
        CH0_TX_END_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The masked interrupt status bit for CH0_RX_END_INT."]
    #[inline(always)]
    pub fn ch0_rx_end_int_st(&self) -> CH0_RX_END_INT_ST_R {
        CH0_RX_END_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The masked interrupt status bit for CH0_ERR_INT."]
    #[inline(always)]
    pub fn ch0_err_int_st(&self) -> CH0_ERR_INT_ST_R {
        CH0_ERR_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The masked interrupt status bit for CH1_TX_END_INT."]
    #[inline(always)]
    pub fn ch1_tx_end_int_st(&self) -> CH1_TX_END_INT_ST_R {
        CH1_TX_END_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The masked interrupt status bit for CH1_RX_END_INT."]
    #[inline(always)]
    pub fn ch1_rx_end_int_st(&self) -> CH1_RX_END_INT_ST_R {
        CH1_RX_END_INT_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The masked interrupt status bit for CH1_ERR_INT."]
    #[inline(always)]
    pub fn ch1_err_int_st(&self) -> CH1_ERR_INT_ST_R {
        CH1_ERR_INT_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The masked interrupt status bit for CH2_TX_END_INT."]
    #[inline(always)]
    pub fn ch2_tx_end_int_st(&self) -> CH2_TX_END_INT_ST_R {
        CH2_TX_END_INT_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The masked interrupt status bit for CH2_RX_END_INT."]
    #[inline(always)]
    pub fn ch2_rx_end_int_st(&self) -> CH2_RX_END_INT_ST_R {
        CH2_RX_END_INT_ST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The masked interrupt status bit for CH2_ERR_INT."]
    #[inline(always)]
    pub fn ch2_err_int_st(&self) -> CH2_ERR_INT_ST_R {
        CH2_ERR_INT_ST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The masked interrupt status bit for CH3_TX_END_INT."]
    #[inline(always)]
    pub fn ch3_tx_end_int_st(&self) -> CH3_TX_END_INT_ST_R {
        CH3_TX_END_INT_ST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The masked interrupt status bit for CH3_RX_END_INT."]
    #[inline(always)]
    pub fn ch3_rx_end_int_st(&self) -> CH3_RX_END_INT_ST_R {
        CH3_RX_END_INT_ST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The masked interrupt status bit for CH3_ERR_INT."]
    #[inline(always)]
    pub fn ch3_err_int_st(&self) -> CH3_ERR_INT_ST_R {
        CH3_ERR_INT_ST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The masked interrupt status bit for CH0_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch0_tx_thr_event_int_st(&self) -> CH0_TX_THR_EVENT_INT_ST_R {
        CH0_TX_THR_EVENT_INT_ST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The masked interrupt status bit for CH1_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch1_tx_thr_event_int_st(&self) -> CH1_TX_THR_EVENT_INT_ST_R {
        CH1_TX_THR_EVENT_INT_ST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The masked interrupt status bit for CH2_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch2_tx_thr_event_int_st(&self) -> CH2_TX_THR_EVENT_INT_ST_R {
        CH2_TX_THR_EVENT_INT_ST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The masked interrupt status bit for CH3_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch3_tx_thr_event_int_st(&self) -> CH3_TX_THR_EVENT_INT_ST_R {
        CH3_TX_THR_EVENT_INT_ST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - The masked interrupt status bit for CH0_TX_LOOP_INT."]
    #[inline(always)]
    pub fn ch0_tx_loop_int_st(&self) -> CH0_TX_LOOP_INT_ST_R {
        CH0_TX_LOOP_INT_ST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - The masked interrupt status bit for CH1_TX_LOOP_INT."]
    #[inline(always)]
    pub fn ch1_tx_loop_int_st(&self) -> CH1_TX_LOOP_INT_ST_R {
        CH1_TX_LOOP_INT_ST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - The masked interrupt status bit for CH2_TX_LOOP_INT."]
    #[inline(always)]
    pub fn ch2_tx_loop_int_st(&self) -> CH2_TX_LOOP_INT_ST_R {
        CH2_TX_LOOP_INT_ST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - The masked interrupt status bit for CH3_TX_LOOP_INT."]
    #[inline(always)]
    pub fn ch3_tx_loop_int_st(&self) -> CH3_TX_LOOP_INT_ST_R {
        CH3_TX_LOOP_INT_ST_R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[doc = "Masked interrupt status\n\nThis register you can [`read`]
(crate::generic::Reg::read). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_st]
(index.html) module"]
pub struct INT_ST_SPEC;
impl crate::RegisterSpec for INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_st::R]
(R) reader structure"]
impl crate::Readable for INT_ST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for INT_ST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}