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
#[doc = "Field `RX_TAKE_DATA_INT_ST` reader - "]
pub struct RX_TAKE_DATA_INT_ST_R(crate::FieldReader<bool, bool>);
impl RX_TAKE_DATA_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_TAKE_DATA_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_TAKE_DATA_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_PUT_DATA_INT_ST` reader - "]
pub struct TX_PUT_DATA_INT_ST_R(crate::FieldReader<bool, bool>);
impl TX_PUT_DATA_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_PUT_DATA_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_PUT_DATA_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_WFULL_INT_ST` reader - "]
pub struct RX_WFULL_INT_ST_R(crate::FieldReader<bool, bool>);
impl RX_WFULL_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_WFULL_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_WFULL_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_REMPTY_INT_ST` reader - "]
pub struct RX_REMPTY_INT_ST_R(crate::FieldReader<bool, bool>);
impl RX_REMPTY_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_REMPTY_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_REMPTY_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_WFULL_INT_ST` reader - "]
pub struct TX_WFULL_INT_ST_R(crate::FieldReader<bool, bool>);
impl TX_WFULL_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_WFULL_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_WFULL_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_REMPTY_INT_ST` reader - "]
pub struct TX_REMPTY_INT_ST_R(crate::FieldReader<bool, bool>);
impl TX_REMPTY_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_REMPTY_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_REMPTY_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_HUNG_INT_ST` reader - "]
pub struct RX_HUNG_INT_ST_R(crate::FieldReader<bool, bool>);
impl RX_HUNG_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_HUNG_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_HUNG_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_HUNG_INT_ST` reader - "]
pub struct TX_HUNG_INT_ST_R(crate::FieldReader<bool, bool>);
impl TX_HUNG_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_HUNG_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_HUNG_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN_DONE_INT_ST` reader - "]
pub struct IN_DONE_INT_ST_R(crate::FieldReader<bool, bool>);
impl IN_DONE_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IN_DONE_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN_DONE_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN_SUC_EOF_INT_ST` reader - "]
pub struct IN_SUC_EOF_INT_ST_R(crate::FieldReader<bool, bool>);
impl IN_SUC_EOF_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IN_SUC_EOF_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN_SUC_EOF_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN_ERR_EOF_INT_ST` reader - "]
pub struct IN_ERR_EOF_INT_ST_R(crate::FieldReader<bool, bool>);
impl IN_ERR_EOF_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IN_ERR_EOF_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN_ERR_EOF_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT_DONE_INT_ST` reader - "]
pub struct OUT_DONE_INT_ST_R(crate::FieldReader<bool, bool>);
impl OUT_DONE_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUT_DONE_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_DONE_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT_EOF_INT_ST` reader - "]
pub struct OUT_EOF_INT_ST_R(crate::FieldReader<bool, bool>);
impl OUT_EOF_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUT_EOF_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_EOF_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN_DSCR_ERR_INT_ST` reader - "]
pub struct IN_DSCR_ERR_INT_ST_R(crate::FieldReader<bool, bool>);
impl IN_DSCR_ERR_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IN_DSCR_ERR_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN_DSCR_ERR_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT_DSCR_ERR_INT_ST` reader - "]
pub struct OUT_DSCR_ERR_INT_ST_R(crate::FieldReader<bool, bool>);
impl OUT_DSCR_ERR_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUT_DSCR_ERR_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_DSCR_ERR_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN_DSCR_EMPTY_INT_ST` reader - "]
pub struct IN_DSCR_EMPTY_INT_ST_R(crate::FieldReader<bool, bool>);
impl IN_DSCR_EMPTY_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IN_DSCR_EMPTY_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN_DSCR_EMPTY_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT_TOTAL_EOF_INT_ST` reader - "]
pub struct OUT_TOTAL_EOF_INT_ST_R(crate::FieldReader<bool, bool>);
impl OUT_TOTAL_EOF_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUT_TOTAL_EOF_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_TOTAL_EOF_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rx_take_data_int_st(&self) -> RX_TAKE_DATA_INT_ST_R {
        RX_TAKE_DATA_INT_ST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tx_put_data_int_st(&self) -> TX_PUT_DATA_INT_ST_R {
        TX_PUT_DATA_INT_ST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rx_wfull_int_st(&self) -> RX_WFULL_INT_ST_R {
        RX_WFULL_INT_ST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rx_rempty_int_st(&self) -> RX_REMPTY_INT_ST_R {
        RX_REMPTY_INT_ST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn tx_wfull_int_st(&self) -> TX_WFULL_INT_ST_R {
        TX_WFULL_INT_ST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn tx_rempty_int_st(&self) -> TX_REMPTY_INT_ST_R {
        TX_REMPTY_INT_ST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rx_hung_int_st(&self) -> RX_HUNG_INT_ST_R {
        RX_HUNG_INT_ST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn tx_hung_int_st(&self) -> TX_HUNG_INT_ST_R {
        TX_HUNG_INT_ST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn in_done_int_st(&self) -> IN_DONE_INT_ST_R {
        IN_DONE_INT_ST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn in_suc_eof_int_st(&self) -> IN_SUC_EOF_INT_ST_R {
        IN_SUC_EOF_INT_ST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn in_err_eof_int_st(&self) -> IN_ERR_EOF_INT_ST_R {
        IN_ERR_EOF_INT_ST_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn out_done_int_st(&self) -> OUT_DONE_INT_ST_R {
        OUT_DONE_INT_ST_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn out_eof_int_st(&self) -> OUT_EOF_INT_ST_R {
        OUT_EOF_INT_ST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn in_dscr_err_int_st(&self) -> IN_DSCR_ERR_INT_ST_R {
        IN_DSCR_ERR_INT_ST_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn out_dscr_err_int_st(&self) -> OUT_DSCR_ERR_INT_ST_R {
        OUT_DSCR_ERR_INT_ST_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn in_dscr_empty_int_st(&self) -> IN_DSCR_EMPTY_INT_ST_R {
        IN_DSCR_EMPTY_INT_ST_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn out_total_eof_int_st(&self) -> OUT_TOTAL_EOF_INT_ST_R {
        OUT_TOTAL_EOF_INT_ST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
#[doc = "\n\nThis register you can [`read`]
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