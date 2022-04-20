#[doc = "Register `LC_CONF` reader"]
pub struct R(crate::R<LC_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LC_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LC_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LC_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LC_CONF` writer"]
pub struct W(crate::W<LC_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LC_CONF_SPEC>;
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
impl From<crate::W<LC_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LC_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IN_RST` reader - Set this bit to reset in-DMA FSM. Set this bit before the DMA configuration."]
pub struct IN_RST_R(crate::FieldReader<bool, bool>);
impl IN_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IN_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN_RST` writer - Set this bit to reset in-DMA FSM. Set this bit before the DMA configuration."]
pub struct IN_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_RST_W<'a> {
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
        self.w.bits = (self.w.bits & !1) | (value as u32 & 1);
        self.w
    }
}
#[doc = "Field `OUT_RST` reader - Set this bit to reset out-DMA FSM. Set this bit before the DMA configuration."]
pub struct OUT_RST_R(crate::FieldReader<bool, bool>);
impl OUT_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUT_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT_RST` writer - Set this bit to reset out-DMA FSM. Set this bit before the DMA configuration."]
pub struct OUT_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u32 & 1) << 1);
        self.w
    }
}
#[doc = "Field `AHBM_FIFO_RST` reader - Set this bit to reset AHB interface cmdFIFO of DMA. Set this bit before the DMA configuration."]
pub struct AHBM_FIFO_RST_R(crate::FieldReader<bool, bool>);
impl AHBM_FIFO_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AHBM_FIFO_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AHBM_FIFO_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AHBM_FIFO_RST` writer - Set this bit to reset AHB interface cmdFIFO of DMA. Set this bit before the DMA configuration."]
pub struct AHBM_FIFO_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> AHBM_FIFO_RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 2)) | ((value as u32 & 1) << 2);
        self.w
    }
}
#[doc = "Field `AHBM_RST` reader - Set this bit to reset AHB interface of DMA. Set this bit before the DMA configuration."]
pub struct AHBM_RST_R(crate::FieldReader<bool, bool>);
impl AHBM_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AHBM_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AHBM_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AHBM_RST` writer - Set this bit to reset AHB interface of DMA. Set this bit before the DMA configuration."]
pub struct AHBM_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> AHBM_RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 3)) | ((value as u32 & 1) << 3);
        self.w
    }
}
#[doc = "Field `OUT_LOOP_TEST` reader - Set this bit to loop test inlink."]
pub struct OUT_LOOP_TEST_R(crate::FieldReader<bool, bool>);
impl OUT_LOOP_TEST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUT_LOOP_TEST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_LOOP_TEST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT_LOOP_TEST` writer - Set this bit to loop test inlink."]
pub struct OUT_LOOP_TEST_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_LOOP_TEST_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u32 & 1) << 4);
        self.w
    }
}
#[doc = "Field `IN_LOOP_TEST` reader - Set this bit to loop test outlink."]
pub struct IN_LOOP_TEST_R(crate::FieldReader<bool, bool>);
impl IN_LOOP_TEST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IN_LOOP_TEST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN_LOOP_TEST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN_LOOP_TEST` writer - Set this bit to loop test outlink."]
pub struct IN_LOOP_TEST_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_LOOP_TEST_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 5)) | ((value as u32 & 1) << 5);
        self.w
    }
}
#[doc = "Field `OUT_AUTO_WRBACK` reader - Set this bit to enable outlink-written-back automatically when out buffer is transmitted done."]
pub struct OUT_AUTO_WRBACK_R(crate::FieldReader<bool, bool>);
impl OUT_AUTO_WRBACK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUT_AUTO_WRBACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_AUTO_WRBACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT_AUTO_WRBACK` writer - Set this bit to enable outlink-written-back automatically when out buffer is transmitted done."]
pub struct OUT_AUTO_WRBACK_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_AUTO_WRBACK_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 6)) | ((value as u32 & 1) << 6);
        self.w
    }
}
#[doc = "Field `OUT_NO_RESTART_CLR` reader - Reserved."]
pub struct OUT_NO_RESTART_CLR_R(crate::FieldReader<bool, bool>);
impl OUT_NO_RESTART_CLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUT_NO_RESTART_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_NO_RESTART_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT_NO_RESTART_CLR` writer - Reserved."]
pub struct OUT_NO_RESTART_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_NO_RESTART_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 7)) | ((value as u32 & 1) << 7);
        self.w
    }
}
#[doc = "Field `OUT_EOF_MODE` reader - DMA out EOF flag generation mode. 1: When DMA has popped all data from the FIFO. 0: When AHB has pushed all data to the FIFO."]
pub struct OUT_EOF_MODE_R(crate::FieldReader<bool, bool>);
impl OUT_EOF_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUT_EOF_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_EOF_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT_EOF_MODE` writer - DMA out EOF flag generation mode. 1: When DMA has popped all data from the FIFO. 0: When AHB has pushed all data to the FIFO."]
pub struct OUT_EOF_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_EOF_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 8)) | ((value as u32 & 1) << 8);
        self.w
    }
}
#[doc = "Field `OUTDSCR_BURST_EN` reader - DMA outlink descriptor transfer mode configuration bit. 1: Prepare outlink descriptor with burst mode. 0: Prepare outlink descriptor with byte mode."]
pub struct OUTDSCR_BURST_EN_R(crate::FieldReader<bool, bool>);
impl OUTDSCR_BURST_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUTDSCR_BURST_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTDSCR_BURST_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUTDSCR_BURST_EN` writer - DMA outlink descriptor transfer mode configuration bit. 1: Prepare outlink descriptor with burst mode. 0: Prepare outlink descriptor with byte mode."]
pub struct OUTDSCR_BURST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTDSCR_BURST_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 9)) | ((value as u32 & 1) << 9);
        self.w
    }
}
#[doc = "Field `INDSCR_BURST_EN` reader - DMA inlink descriptor transfer mode configuration bit. 1: Prepare inlink descriptor with burst mode. 0: Prepare inlink descriptor with byte mode."]
pub struct INDSCR_BURST_EN_R(crate::FieldReader<bool, bool>);
impl INDSCR_BURST_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INDSCR_BURST_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INDSCR_BURST_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INDSCR_BURST_EN` writer - DMA inlink descriptor transfer mode configuration bit. 1: Prepare inlink descriptor with burst mode. 0: Prepare inlink descriptor with byte mode."]
pub struct INDSCR_BURST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> INDSCR_BURST_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 10)) | ((value as u32 & 1) << 10);
        self.w
    }
}
#[doc = "Field `OUT_DATA_BURST_EN` reader - Transmitter data transfer mode configuration bit. 1: Prepare out data with burst mode. 0: Prepare out data with byte mode."]
pub struct OUT_DATA_BURST_EN_R(crate::FieldReader<bool, bool>);
impl OUT_DATA_BURST_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUT_DATA_BURST_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_DATA_BURST_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT_DATA_BURST_EN` writer - Transmitter data transfer mode configuration bit. 1: Prepare out data with burst mode. 0: Prepare out data with byte mode."]
pub struct OUT_DATA_BURST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_DATA_BURST_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 11)) | ((value as u32 & 1) << 11);
        self.w
    }
}
#[doc = "Field `CHECK_OWNER` reader - Set this bit to enable check owner bit by hardware."]
pub struct CHECK_OWNER_R(crate::FieldReader<bool, bool>);
impl CHECK_OWNER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHECK_OWNER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHECK_OWNER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHECK_OWNER` writer - Set this bit to enable check owner bit by hardware."]
pub struct CHECK_OWNER_W<'a> {
    w: &'a mut W,
}
impl<'a> CHECK_OWNER_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 12)) | ((value as u32 & 1) << 12);
        self.w
    }
}
#[doc = "Field `MEM_TRANS_EN` reader - Reserved."]
pub struct MEM_TRANS_EN_R(crate::FieldReader<bool, bool>);
impl MEM_TRANS_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MEM_TRANS_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM_TRANS_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM_TRANS_EN` writer - Reserved."]
pub struct MEM_TRANS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_TRANS_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 13)) | ((value as u32 & 1) << 13);
        self.w
    }
}
#[doc = "Field `EXT_MEM_BK_SIZE` reader - DMA access external memory block size. 0: 16 bytes. 1: 32 bytes. 2: 64 bytes. 3: reserved."]
pub struct EXT_MEM_BK_SIZE_R(crate::FieldReader<u8, u8>);
impl EXT_MEM_BK_SIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EXT_MEM_BK_SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXT_MEM_BK_SIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXT_MEM_BK_SIZE` writer - DMA access external memory block size. 0: 16 bytes. 1: 32 bytes. 2: 64 bytes. 3: reserved."]
pub struct EXT_MEM_BK_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> EXT_MEM_BK_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 14)) | ((value as u32 & 3) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Set this bit to reset in-DMA FSM. Set this bit before the DMA configuration."]
    #[inline(always)]
    pub fn in_rst(&self) -> IN_RST_R {
        IN_RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to reset out-DMA FSM. Set this bit before the DMA configuration."]
    #[inline(always)]
    pub fn out_rst(&self) -> OUT_RST_R {
        OUT_RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to reset AHB interface cmdFIFO of DMA. Set this bit before the DMA configuration."]
    #[inline(always)]
    pub fn ahbm_fifo_rst(&self) -> AHBM_FIFO_RST_R {
        AHBM_FIFO_RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to reset AHB interface of DMA. Set this bit before the DMA configuration."]
    #[inline(always)]
    pub fn ahbm_rst(&self) -> AHBM_RST_R {
        AHBM_RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set this bit to loop test inlink."]
    #[inline(always)]
    pub fn out_loop_test(&self) -> OUT_LOOP_TEST_R {
        OUT_LOOP_TEST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set this bit to loop test outlink."]
    #[inline(always)]
    pub fn in_loop_test(&self) -> IN_LOOP_TEST_R {
        IN_LOOP_TEST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Set this bit to enable outlink-written-back automatically when out buffer is transmitted done."]
    #[inline(always)]
    pub fn out_auto_wrback(&self) -> OUT_AUTO_WRBACK_R {
        OUT_AUTO_WRBACK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Reserved."]
    #[inline(always)]
    pub fn out_no_restart_clr(&self) -> OUT_NO_RESTART_CLR_R {
        OUT_NO_RESTART_CLR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DMA out EOF flag generation mode. 1: When DMA has popped all data from the FIFO. 0: When AHB has pushed all data to the FIFO."]
    #[inline(always)]
    pub fn out_eof_mode(&self) -> OUT_EOF_MODE_R {
        OUT_EOF_MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DMA outlink descriptor transfer mode configuration bit. 1: Prepare outlink descriptor with burst mode. 0: Prepare outlink descriptor with byte mode."]
    #[inline(always)]
    pub fn outdscr_burst_en(&self) -> OUTDSCR_BURST_EN_R {
        OUTDSCR_BURST_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DMA inlink descriptor transfer mode configuration bit. 1: Prepare inlink descriptor with burst mode. 0: Prepare inlink descriptor with byte mode."]
    #[inline(always)]
    pub fn indscr_burst_en(&self) -> INDSCR_BURST_EN_R {
        INDSCR_BURST_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Transmitter data transfer mode configuration bit. 1: Prepare out data with burst mode. 0: Prepare out data with byte mode."]
    #[inline(always)]
    pub fn out_data_burst_en(&self) -> OUT_DATA_BURST_EN_R {
        OUT_DATA_BURST_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Set this bit to enable check owner bit by hardware."]
    #[inline(always)]
    pub fn check_owner(&self) -> CHECK_OWNER_R {
        CHECK_OWNER_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Reserved."]
    #[inline(always)]
    pub fn mem_trans_en(&self) -> MEM_TRANS_EN_R {
        MEM_TRANS_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - DMA access external memory block size. 0: 16 bytes. 1: 32 bytes. 2: 64 bytes. 3: reserved."]
    #[inline(always)]
    pub fn ext_mem_bk_size(&self) -> EXT_MEM_BK_SIZE_R {
        EXT_MEM_BK_SIZE_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to reset in-DMA FSM. Set this bit before the DMA configuration."]
    #[inline(always)]
    pub fn in_rst(&mut self) -> IN_RST_W {
        IN_RST_W { w: self }
    }
    #[doc = "Bit 1 - Set this bit to reset out-DMA FSM. Set this bit before the DMA configuration."]
    #[inline(always)]
    pub fn out_rst(&mut self) -> OUT_RST_W {
        OUT_RST_W { w: self }
    }
    #[doc = "Bit 2 - Set this bit to reset AHB interface cmdFIFO of DMA. Set this bit before the DMA configuration."]
    #[inline(always)]
    pub fn ahbm_fifo_rst(&mut self) -> AHBM_FIFO_RST_W {
        AHBM_FIFO_RST_W { w: self }
    }
    #[doc = "Bit 3 - Set this bit to reset AHB interface of DMA. Set this bit before the DMA configuration."]
    #[inline(always)]
    pub fn ahbm_rst(&mut self) -> AHBM_RST_W {
        AHBM_RST_W { w: self }
    }
    #[doc = "Bit 4 - Set this bit to loop test inlink."]
    #[inline(always)]
    pub fn out_loop_test(&mut self) -> OUT_LOOP_TEST_W {
        OUT_LOOP_TEST_W { w: self }
    }
    #[doc = "Bit 5 - Set this bit to loop test outlink."]
    #[inline(always)]
    pub fn in_loop_test(&mut self) -> IN_LOOP_TEST_W {
        IN_LOOP_TEST_W { w: self }
    }
    #[doc = "Bit 6 - Set this bit to enable outlink-written-back automatically when out buffer is transmitted done."]
    #[inline(always)]
    pub fn out_auto_wrback(&mut self) -> OUT_AUTO_WRBACK_W {
        OUT_AUTO_WRBACK_W { w: self }
    }
    #[doc = "Bit 7 - Reserved."]
    #[inline(always)]
    pub fn out_no_restart_clr(&mut self) -> OUT_NO_RESTART_CLR_W {
        OUT_NO_RESTART_CLR_W { w: self }
    }
    #[doc = "Bit 8 - DMA out EOF flag generation mode. 1: When DMA has popped all data from the FIFO. 0: When AHB has pushed all data to the FIFO."]
    #[inline(always)]
    pub fn out_eof_mode(&mut self) -> OUT_EOF_MODE_W {
        OUT_EOF_MODE_W { w: self }
    }
    #[doc = "Bit 9 - DMA outlink descriptor transfer mode configuration bit. 1: Prepare outlink descriptor with burst mode. 0: Prepare outlink descriptor with byte mode."]
    #[inline(always)]
    pub fn outdscr_burst_en(&mut self) -> OUTDSCR_BURST_EN_W {
        OUTDSCR_BURST_EN_W { w: self }
    }
    #[doc = "Bit 10 - DMA inlink descriptor transfer mode configuration bit. 1: Prepare inlink descriptor with burst mode. 0: Prepare inlink descriptor with byte mode."]
    #[inline(always)]
    pub fn indscr_burst_en(&mut self) -> INDSCR_BURST_EN_W {
        INDSCR_BURST_EN_W { w: self }
    }
    #[doc = "Bit 11 - Transmitter data transfer mode configuration bit. 1: Prepare out data with burst mode. 0: Prepare out data with byte mode."]
    #[inline(always)]
    pub fn out_data_burst_en(&mut self) -> OUT_DATA_BURST_EN_W {
        OUT_DATA_BURST_EN_W { w: self }
    }
    #[doc = "Bit 12 - Set this bit to enable check owner bit by hardware."]
    #[inline(always)]
    pub fn check_owner(&mut self) -> CHECK_OWNER_W {
        CHECK_OWNER_W { w: self }
    }
    #[doc = "Bit 13 - Reserved."]
    #[inline(always)]
    pub fn mem_trans_en(&mut self) -> MEM_TRANS_EN_W {
        MEM_TRANS_EN_W { w: self }
    }
    #[doc = "Bits 14:15 - DMA access external memory block size. 0: 16 bytes. 1: 32 bytes. 2: 64 bytes. 3: reserved."]
    #[inline(always)]
    pub fn ext_mem_bk_size(&mut self) -> EXT_MEM_BK_SIZE_W {
        EXT_MEM_BK_SIZE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S DMA configuration register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lc_conf]
(index.html) module"]
pub struct LC_CONF_SPEC;
impl crate::RegisterSpec for LC_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lc_conf::R]
(R) reader structure"]
impl crate::Readable for LC_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lc_conf::W]
(W) writer structure"]
impl crate::Writable for LC_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LC_CONF to value 0x0100"]
impl crate::Resettable for LC_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0100
    }
}