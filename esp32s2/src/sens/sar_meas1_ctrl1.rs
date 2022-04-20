#[doc = "Register `SAR_MEAS1_CTRL1` reader"]
pub struct R(crate::R<SAR_MEAS1_CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_MEAS1_CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_MEAS1_CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_MEAS1_CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_MEAS1_CTRL1` writer"]
pub struct W(crate::W<SAR_MEAS1_CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_MEAS1_CTRL1_SPEC>;
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
impl From<crate::W<SAR_MEAS1_CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_MEAS1_CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC_SARADC_RESET` reader - SAR ADC software reset."]
pub struct RTC_SARADC_RESET_R(crate::FieldReader<bool, bool>);
impl RTC_SARADC_RESET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_SARADC_RESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_SARADC_RESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_SARADC_RESET` writer - SAR ADC software reset."]
pub struct RTC_SARADC_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_SARADC_RESET_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 22)) | ((value as u32 & 1) << 22);
        self.w
    }
}
#[doc = "Field `RTC_SARADC_CLKGATE_EN` reader - Enable bit of SAR ADC clock gate."]
pub struct RTC_SARADC_CLKGATE_EN_R(crate::FieldReader<bool, bool>);
impl RTC_SARADC_CLKGATE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_SARADC_CLKGATE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_SARADC_CLKGATE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_SARADC_CLKGATE_EN` writer - Enable bit of SAR ADC clock gate."]
pub struct RTC_SARADC_CLKGATE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_SARADC_CLKGATE_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 23)) | ((value as u32 & 1) << 23);
        self.w
    }
}
#[doc = "Field `FORCE_XPD_AMP` reader - "]
pub struct FORCE_XPD_AMP_R(crate::FieldReader<u8, u8>);
impl FORCE_XPD_AMP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FORCE_XPD_AMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FORCE_XPD_AMP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FORCE_XPD_AMP` writer - "]
pub struct FORCE_XPD_AMP_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_XPD_AMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 24)) | ((value as u32 & 3) << 24);
        self.w
    }
}
#[doc = "Field `AMP_RST_FB_FORCE` reader - "]
pub struct AMP_RST_FB_FORCE_R(crate::FieldReader<u8, u8>);
impl AMP_RST_FB_FORCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        AMP_RST_FB_FORCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AMP_RST_FB_FORCE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AMP_RST_FB_FORCE` writer - "]
pub struct AMP_RST_FB_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> AMP_RST_FB_FORCE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 26)) | ((value as u32 & 3) << 26);
        self.w
    }
}
#[doc = "Field `AMP_SHORT_REF_FORCE` reader - "]
pub struct AMP_SHORT_REF_FORCE_R(crate::FieldReader<u8, u8>);
impl AMP_SHORT_REF_FORCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        AMP_SHORT_REF_FORCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AMP_SHORT_REF_FORCE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AMP_SHORT_REF_FORCE` writer - "]
pub struct AMP_SHORT_REF_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> AMP_SHORT_REF_FORCE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 28)) | ((value as u32 & 3) << 28);
        self.w
    }
}
#[doc = "Field `AMP_SHORT_REF_GND_FORCE` reader - "]
pub struct AMP_SHORT_REF_GND_FORCE_R(crate::FieldReader<u8, u8>);
impl AMP_SHORT_REF_GND_FORCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        AMP_SHORT_REF_GND_FORCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AMP_SHORT_REF_GND_FORCE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AMP_SHORT_REF_GND_FORCE` writer - "]
pub struct AMP_SHORT_REF_GND_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> AMP_SHORT_REF_GND_FORCE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 30)) | ((value as u32 & 3) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bit 22 - SAR ADC software reset."]
    #[inline(always)]
    pub fn rtc_saradc_reset(&self) -> RTC_SARADC_RESET_R {
        RTC_SARADC_RESET_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable bit of SAR ADC clock gate."]
    #[inline(always)]
    pub fn rtc_saradc_clkgate_en(&self) -> RTC_SARADC_CLKGATE_EN_R {
        RTC_SARADC_CLKGATE_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn force_xpd_amp(&self) -> FORCE_XPD_AMP_R {
        FORCE_XPD_AMP_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn amp_rst_fb_force(&self) -> AMP_RST_FB_FORCE_R {
        AMP_RST_FB_FORCE_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn amp_short_ref_force(&self) -> AMP_SHORT_REF_FORCE_R {
        AMP_SHORT_REF_FORCE_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn amp_short_ref_gnd_force(&self) -> AMP_SHORT_REF_GND_FORCE_R {
        AMP_SHORT_REF_GND_FORCE_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 22 - SAR ADC software reset."]
    #[inline(always)]
    pub fn rtc_saradc_reset(&mut self) -> RTC_SARADC_RESET_W {
        RTC_SARADC_RESET_W { w: self }
    }
    #[doc = "Bit 23 - Enable bit of SAR ADC clock gate."]
    #[inline(always)]
    pub fn rtc_saradc_clkgate_en(&mut self) -> RTC_SARADC_CLKGATE_EN_W {
        RTC_SARADC_CLKGATE_EN_W { w: self }
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn force_xpd_amp(&mut self) -> FORCE_XPD_AMP_W {
        FORCE_XPD_AMP_W { w: self }
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn amp_rst_fb_force(&mut self) -> AMP_RST_FB_FORCE_W {
        AMP_RST_FB_FORCE_W { w: self }
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn amp_short_ref_force(&mut self) -> AMP_SHORT_REF_FORCE_W {
        AMP_SHORT_REF_FORCE_W { w: self }
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn amp_short_ref_gnd_force(&mut self) -> AMP_SHORT_REF_GND_FORCE_W {
        AMP_SHORT_REF_GND_FORCE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configure RTC ADC1 controller\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_meas1_ctrl1]
(index.html) module"]
pub struct SAR_MEAS1_CTRL1_SPEC;
impl crate::RegisterSpec for SAR_MEAS1_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_meas1_ctrl1::R]
(R) reader structure"]
impl crate::Readable for SAR_MEAS1_CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_meas1_ctrl1::W]
(W) writer structure"]
impl crate::Writable for SAR_MEAS1_CTRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAR_MEAS1_CTRL1 to value 0"]
impl crate::Resettable for SAR_MEAS1_CTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}