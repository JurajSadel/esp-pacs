#[doc = "Register `DMA_INT_ST` reader"]
pub struct R(crate::R<DMA_INT_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_INT_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_INT_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_INT_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_INT_ST` writer"]
pub struct W(crate::W<DMA_INT_ST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_INT_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<DMA_INT_ST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_INT_ST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INLINK_DSCR_EMPTY_INT_ST` reader - The status bit for lack of enough inlink descriptors."]
pub struct INLINK_DSCR_EMPTY_INT_ST_R(crate::FieldReader<bool, bool>);
impl INLINK_DSCR_EMPTY_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INLINK_DSCR_EMPTY_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INLINK_DSCR_EMPTY_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUTLINK_DSCR_ERROR_INT_ST` reader - The status bit for outlink descriptor error."]
pub struct OUTLINK_DSCR_ERROR_INT_ST_R(crate::FieldReader<bool, bool>);
impl OUTLINK_DSCR_ERROR_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUTLINK_DSCR_ERROR_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTLINK_DSCR_ERROR_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INLINK_DSCR_ERROR_INT_ST` reader - The status bit for inlink descriptor error."]
pub struct INLINK_DSCR_ERROR_INT_ST_R(crate::FieldReader<bool, bool>);
impl INLINK_DSCR_ERROR_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INLINK_DSCR_ERROR_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INLINK_DSCR_ERROR_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN_DONE_INT_ST` reader - The status bit for completing usage of a inlink descriptor."]
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
#[doc = "Field `IN_ERR_EOF_INT_ST` reader - The status bit for receiving error."]
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
#[doc = "Field `IN_SUC_EOF_INT_ST` reader - The status bit for completing receiving all the packets from host."]
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
#[doc = "Field `OUT_DONE_INT_ST` reader - The status bit for completing usage of a outlink descriptor."]
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
#[doc = "Field `OUT_EOF_INT_ST` reader - The status bit for sending a packet to host done."]
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
#[doc = "Field `OUT_TOTAL_EOF_INT_ST` reader - The status bit for sending all the packets to host done."]
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
#[doc = "Field `INFIFO_FULL_ERR_INT_ST` reader - The status bit for infifo full error."]
pub struct INFIFO_FULL_ERR_INT_ST_R(crate::FieldReader<bool, bool>);
impl INFIFO_FULL_ERR_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INFIFO_FULL_ERR_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INFIFO_FULL_ERR_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUTFIFO_EMPTY_ERR_INT_ST` reader - The status bit for outfifo empty error."]
pub struct OUTFIFO_EMPTY_ERR_INT_ST_R(crate::FieldReader<bool, bool>);
impl OUTFIFO_EMPTY_ERR_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUTFIFO_EMPTY_ERR_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTFIFO_EMPTY_ERR_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_CMD6_INT_ST` reader - The status bit for SPI slave CMD6 interrupt."]
pub struct SLV_CMD6_INT_ST_R(crate::FieldReader<bool, bool>);
impl SLV_CMD6_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLV_CMD6_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_CMD6_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_CMD6_INT_ST` writer - The status bit for SPI slave CMD6 interrupt."]
pub struct SLV_CMD6_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_CMD6_INT_ST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `SLV_CMD7_INT_ST` reader - The status bit for SPI slave CMD7 interrupt."]
pub struct SLV_CMD7_INT_ST_R(crate::FieldReader<bool, bool>);
impl SLV_CMD7_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLV_CMD7_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_CMD7_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_CMD7_INT_ST` writer - The status bit for SPI slave CMD7 interrupt."]
pub struct SLV_CMD7_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_CMD7_INT_ST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `SLV_CMD8_INT_ST` reader - The status bit for SPI slave CMD8 interrupt."]
pub struct SLV_CMD8_INT_ST_R(crate::FieldReader<bool, bool>);
impl SLV_CMD8_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLV_CMD8_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_CMD8_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_CMD8_INT_ST` writer - The status bit for SPI slave CMD8 interrupt."]
pub struct SLV_CMD8_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_CMD8_INT_ST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `SLV_CMD9_INT_ST` reader - The status bit for SPI slave CMD9 interrupt."]
pub struct SLV_CMD9_INT_ST_R(crate::FieldReader<bool, bool>);
impl SLV_CMD9_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLV_CMD9_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_CMD9_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_CMD9_INT_ST` writer - The status bit for SPI slave CMD9 interrupt."]
pub struct SLV_CMD9_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_CMD9_INT_ST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `SLV_CMDA_INT_ST` reader - The status bit for SPI slave CMDA interrupt."]
pub struct SLV_CMDA_INT_ST_R(crate::FieldReader<bool, bool>);
impl SLV_CMDA_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLV_CMDA_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_CMDA_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_CMDA_INT_ST` writer - The status bit for SPI slave CMDA interrupt."]
pub struct SLV_CMDA_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_CMDA_INT_ST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - The status bit for lack of enough inlink descriptors."]
    #[inline(always)]
    pub fn inlink_dscr_empty_int_st(&self) -> INLINK_DSCR_EMPTY_INT_ST_R {
        INLINK_DSCR_EMPTY_INT_ST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - The status bit for outlink descriptor error."]
    #[inline(always)]
    pub fn outlink_dscr_error_int_st(&self) -> OUTLINK_DSCR_ERROR_INT_ST_R {
        OUTLINK_DSCR_ERROR_INT_ST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - The status bit for inlink descriptor error."]
    #[inline(always)]
    pub fn inlink_dscr_error_int_st(&self) -> INLINK_DSCR_ERROR_INT_ST_R {
        INLINK_DSCR_ERROR_INT_ST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - The status bit for completing usage of a inlink descriptor."]
    #[inline(always)]
    pub fn in_done_int_st(&self) -> IN_DONE_INT_ST_R {
        IN_DONE_INT_ST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - The status bit for receiving error."]
    #[inline(always)]
    pub fn in_err_eof_int_st(&self) -> IN_ERR_EOF_INT_ST_R {
        IN_ERR_EOF_INT_ST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - The status bit for completing receiving all the packets from host."]
    #[inline(always)]
    pub fn in_suc_eof_int_st(&self) -> IN_SUC_EOF_INT_ST_R {
        IN_SUC_EOF_INT_ST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - The status bit for completing usage of a outlink descriptor."]
    #[inline(always)]
    pub fn out_done_int_st(&self) -> OUT_DONE_INT_ST_R {
        OUT_DONE_INT_ST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - The status bit for sending a packet to host done."]
    #[inline(always)]
    pub fn out_eof_int_st(&self) -> OUT_EOF_INT_ST_R {
        OUT_EOF_INT_ST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - The status bit for sending all the packets to host done."]
    #[inline(always)]
    pub fn out_total_eof_int_st(&self) -> OUT_TOTAL_EOF_INT_ST_R {
        OUT_TOTAL_EOF_INT_ST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - The status bit for infifo full error."]
    #[inline(always)]
    pub fn infifo_full_err_int_st(&self) -> INFIFO_FULL_ERR_INT_ST_R {
        INFIFO_FULL_ERR_INT_ST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - The status bit for outfifo empty error."]
    #[inline(always)]
    pub fn outfifo_empty_err_int_st(&self) -> OUTFIFO_EMPTY_ERR_INT_ST_R {
        OUTFIFO_EMPTY_ERR_INT_ST_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - The status bit for SPI slave CMD6 interrupt."]
    #[inline(always)]
    pub fn slv_cmd6_int_st(&self) -> SLV_CMD6_INT_ST_R {
        SLV_CMD6_INT_ST_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - The status bit for SPI slave CMD7 interrupt."]
    #[inline(always)]
    pub fn slv_cmd7_int_st(&self) -> SLV_CMD7_INT_ST_R {
        SLV_CMD7_INT_ST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - The status bit for SPI slave CMD8 interrupt."]
    #[inline(always)]
    pub fn slv_cmd8_int_st(&self) -> SLV_CMD8_INT_ST_R {
        SLV_CMD8_INT_ST_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - The status bit for SPI slave CMD9 interrupt."]
    #[inline(always)]
    pub fn slv_cmd9_int_st(&self) -> SLV_CMD9_INT_ST_R {
        SLV_CMD9_INT_ST_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - The status bit for SPI slave CMDA interrupt."]
    #[inline(always)]
    pub fn slv_cmda_int_st(&self) -> SLV_CMDA_INT_ST_R {
        SLV_CMDA_INT_ST_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - The status bit for SPI slave CMD6 interrupt."]
    #[inline(always)]
    pub fn slv_cmd6_int_st(&mut self) -> SLV_CMD6_INT_ST_W {
        SLV_CMD6_INT_ST_W { w: self }
    }
    #[doc = "Bit 12 - The status bit for SPI slave CMD7 interrupt."]
    #[inline(always)]
    pub fn slv_cmd7_int_st(&mut self) -> SLV_CMD7_INT_ST_W {
        SLV_CMD7_INT_ST_W { w: self }
    }
    #[doc = "Bit 13 - The status bit for SPI slave CMD8 interrupt."]
    #[inline(always)]
    pub fn slv_cmd8_int_st(&mut self) -> SLV_CMD8_INT_ST_W {
        SLV_CMD8_INT_ST_W { w: self }
    }
    #[doc = "Bit 14 - The status bit for SPI slave CMD9 interrupt."]
    #[inline(always)]
    pub fn slv_cmd9_int_st(&mut self) -> SLV_CMD9_INT_ST_W {
        SLV_CMD9_INT_ST_W { w: self }
    }
    #[doc = "Bit 15 - The status bit for SPI slave CMDA interrupt."]
    #[inline(always)]
    pub fn slv_cmda_int_st(&mut self) -> SLV_CMDA_INT_ST_W {
        SLV_CMDA_INT_ST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI DMA interrupt status register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_int_st]
(index.html) module"]
pub struct DMA_INT_ST_SPEC;
impl crate::RegisterSpec for DMA_INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_int_st::R]
(R) reader structure"]
impl crate::Readable for DMA_INT_ST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_int_st::W]
(W) writer structure"]
impl crate::Writable for DMA_INT_ST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_INT_ST to value 0"]
impl crate::Resettable for DMA_INT_ST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}