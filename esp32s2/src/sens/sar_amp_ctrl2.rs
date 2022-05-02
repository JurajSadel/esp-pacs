#[doc = "Register `SAR_AMP_CTRL2` reader"]
pub struct R(crate::R<SAR_AMP_CTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_AMP_CTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_AMP_CTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_AMP_CTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_AMP_CTRL2` writer"]
pub struct W(crate::W<SAR_AMP_CTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_AMP_CTRL2_SPEC>;
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
impl From<crate::W<SAR_AMP_CTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_AMP_CTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAR1_DAC_XPD_FSM_IDLE` reader - "]
pub struct SAR1_DAC_XPD_FSM_IDLE_R(crate::FieldReader<bool>);
impl SAR1_DAC_XPD_FSM_IDLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SAR1_DAC_XPD_FSM_IDLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAR1_DAC_XPD_FSM_IDLE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAR1_DAC_XPD_FSM_IDLE` writer - "]
pub struct SAR1_DAC_XPD_FSM_IDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR1_DAC_XPD_FSM_IDLE_W<'a> {
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
#[doc = "Field `XPD_SAR_AMP_FSM_IDLE` reader - "]
pub struct XPD_SAR_AMP_FSM_IDLE_R(crate::FieldReader<bool>);
impl XPD_SAR_AMP_FSM_IDLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XPD_SAR_AMP_FSM_IDLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XPD_SAR_AMP_FSM_IDLE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XPD_SAR_AMP_FSM_IDLE` writer - "]
pub struct XPD_SAR_AMP_FSM_IDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> XPD_SAR_AMP_FSM_IDLE_W<'a> {
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
#[doc = "Field `AMP_RST_FB_FSM_IDLE` reader - "]
pub struct AMP_RST_FB_FSM_IDLE_R(crate::FieldReader<bool>);
impl AMP_RST_FB_FSM_IDLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AMP_RST_FB_FSM_IDLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AMP_RST_FB_FSM_IDLE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AMP_RST_FB_FSM_IDLE` writer - "]
pub struct AMP_RST_FB_FSM_IDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> AMP_RST_FB_FSM_IDLE_W<'a> {
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
#[doc = "Field `AMP_SHORT_REF_FSM_IDLE` reader - "]
pub struct AMP_SHORT_REF_FSM_IDLE_R(crate::FieldReader<bool>);
impl AMP_SHORT_REF_FSM_IDLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AMP_SHORT_REF_FSM_IDLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AMP_SHORT_REF_FSM_IDLE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AMP_SHORT_REF_FSM_IDLE` writer - "]
pub struct AMP_SHORT_REF_FSM_IDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> AMP_SHORT_REF_FSM_IDLE_W<'a> {
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
#[doc = "Field `AMP_SHORT_REF_GND_FSM_IDLE` reader - "]
pub struct AMP_SHORT_REF_GND_FSM_IDLE_R(crate::FieldReader<bool>);
impl AMP_SHORT_REF_GND_FSM_IDLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AMP_SHORT_REF_GND_FSM_IDLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AMP_SHORT_REF_GND_FSM_IDLE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AMP_SHORT_REF_GND_FSM_IDLE` writer - "]
pub struct AMP_SHORT_REF_GND_FSM_IDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> AMP_SHORT_REF_GND_FSM_IDLE_W<'a> {
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
#[doc = "Field `XPD_SAR_FSM_IDLE` reader - "]
pub struct XPD_SAR_FSM_IDLE_R(crate::FieldReader<bool>);
impl XPD_SAR_FSM_IDLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XPD_SAR_FSM_IDLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XPD_SAR_FSM_IDLE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XPD_SAR_FSM_IDLE` writer - "]
pub struct XPD_SAR_FSM_IDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> XPD_SAR_FSM_IDLE_W<'a> {
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
#[doc = "Field `SAR_RSTB_FSM_IDLE` reader - "]
pub struct SAR_RSTB_FSM_IDLE_R(crate::FieldReader<bool>);
impl SAR_RSTB_FSM_IDLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SAR_RSTB_FSM_IDLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAR_RSTB_FSM_IDLE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAR_RSTB_FSM_IDLE` writer - "]
pub struct SAR_RSTB_FSM_IDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR_RSTB_FSM_IDLE_W<'a> {
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
#[doc = "Field `SAR_AMP_WAIT3` reader - "]
pub struct SAR_AMP_WAIT3_R(crate::FieldReader<u16>);
impl SAR_AMP_WAIT3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        SAR_AMP_WAIT3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAR_AMP_WAIT3_R {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAR_AMP_WAIT3` writer - "]
pub struct SAR_AMP_WAIT3_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR_AMP_WAIT3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sar1_dac_xpd_fsm_idle(&self) -> SAR1_DAC_XPD_FSM_IDLE_R {
        SAR1_DAC_XPD_FSM_IDLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn xpd_sar_amp_fsm_idle(&self) -> XPD_SAR_AMP_FSM_IDLE_R {
        XPD_SAR_AMP_FSM_IDLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn amp_rst_fb_fsm_idle(&self) -> AMP_RST_FB_FSM_IDLE_R {
        AMP_RST_FB_FSM_IDLE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn amp_short_ref_fsm_idle(&self) -> AMP_SHORT_REF_FSM_IDLE_R {
        AMP_SHORT_REF_FSM_IDLE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn amp_short_ref_gnd_fsm_idle(&self) -> AMP_SHORT_REF_GND_FSM_IDLE_R {
        AMP_SHORT_REF_GND_FSM_IDLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn xpd_sar_fsm_idle(&self) -> XPD_SAR_FSM_IDLE_R {
        XPD_SAR_FSM_IDLE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn sar_rstb_fsm_idle(&self) -> SAR_RSTB_FSM_IDLE_R {
        SAR_RSTB_FSM_IDLE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn sar_amp_wait3(&self) -> SAR_AMP_WAIT3_R {
        SAR_AMP_WAIT3_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sar1_dac_xpd_fsm_idle(&mut self) -> SAR1_DAC_XPD_FSM_IDLE_W {
        SAR1_DAC_XPD_FSM_IDLE_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn xpd_sar_amp_fsm_idle(&mut self) -> XPD_SAR_AMP_FSM_IDLE_W {
        XPD_SAR_AMP_FSM_IDLE_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn amp_rst_fb_fsm_idle(&mut self) -> AMP_RST_FB_FSM_IDLE_W {
        AMP_RST_FB_FSM_IDLE_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn amp_short_ref_fsm_idle(&mut self) -> AMP_SHORT_REF_FSM_IDLE_W {
        AMP_SHORT_REF_FSM_IDLE_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn amp_short_ref_gnd_fsm_idle(&mut self) -> AMP_SHORT_REF_GND_FSM_IDLE_W {
        AMP_SHORT_REF_GND_FSM_IDLE_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn xpd_sar_fsm_idle(&mut self) -> XPD_SAR_FSM_IDLE_W {
        XPD_SAR_FSM_IDLE_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn sar_rstb_fsm_idle(&mut self) -> SAR_RSTB_FSM_IDLE_W {
        SAR_RSTB_FSM_IDLE_W { w: self }
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn sar_amp_wait3(&mut self) -> SAR_AMP_WAIT3_W {
        SAR_AMP_WAIT3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AMP control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_amp_ctrl2](index.html) module"]
pub struct SAR_AMP_CTRL2_SPEC;
impl crate::RegisterSpec for SAR_AMP_CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_amp_ctrl2::R](R) reader structure"]
impl crate::Readable for SAR_AMP_CTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_amp_ctrl2::W](W) writer structure"]
impl crate::Writable for SAR_AMP_CTRL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAR_AMP_CTRL2 to value 0x000a_0000"]
impl crate::Resettable for SAR_AMP_CTRL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x000a_0000
    }
}
