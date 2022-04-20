#[doc = "Register `THRES_CTRL` reader"]
pub struct R(crate::R<THRES_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<THRES_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<THRES_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<THRES_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `THRES_CTRL` writer"]
pub struct W(crate::W<THRES_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<THRES_CTRL_SPEC>;
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
impl From<crate::W<THRES_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<THRES_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLK_EN` reader - Clock gate enable."]
pub struct CLK_EN_R(crate::FieldReader<bool, bool>);
impl CLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK_EN` writer - Clock gate enable."]
pub struct CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_EN_W<'a> {
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
#[doc = "Field `ADC2_THRES_MODE` reader - 1: ADC_DATA > = threshold, generate interrupt. 0: ADC_DATA < threshold, generate interrupt."]
pub struct ADC2_THRES_MODE_R(crate::FieldReader<bool, bool>);
impl ADC2_THRES_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC2_THRES_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC2_THRES_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC2_THRES_MODE` writer - 1: ADC_DATA > = threshold, generate interrupt. 0: ADC_DATA < threshold, generate interrupt."]
pub struct ADC2_THRES_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC2_THRES_MODE_W<'a> {
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
#[doc = "Field `ADC1_THRES_MODE` reader - 1: ADC_DATA > = threshold, generate interrupt. 0: ADC_DATA < threshold, generate interrupt."]
pub struct ADC1_THRES_MODE_R(crate::FieldReader<bool, bool>);
impl ADC1_THRES_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC1_THRES_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC1_THRES_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC1_THRES_MODE` writer - 1: ADC_DATA > = threshold, generate interrupt. 0: ADC_DATA < threshold, generate interrupt."]
pub struct ADC1_THRES_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC1_THRES_MODE_W<'a> {
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
#[doc = "Field `ADC2_THRES` reader - ADC2 threshold."]
pub struct ADC2_THRES_R(crate::FieldReader<u16, u16>);
impl ADC2_THRES_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        ADC2_THRES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC2_THRES_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC2_THRES` writer - ADC2 threshold."]
pub struct ADC2_THRES_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC2_THRES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff << 4)) | ((value as u32 & 0x1fff) << 4);
        self.w
    }
}
#[doc = "Field `ADC1_THRES` reader - ADC1 threshold."]
pub struct ADC1_THRES_R(crate::FieldReader<u16, u16>);
impl ADC1_THRES_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        ADC1_THRES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC1_THRES_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC1_THRES` writer - ADC1 threshold."]
pub struct ADC1_THRES_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC1_THRES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff << 17)) | ((value as u32 & 0x1fff) << 17);
        self.w
    }
}
#[doc = "Field `ADC2_THRES_EN` reader - Enable ADC2 threshold monitor."]
pub struct ADC2_THRES_EN_R(crate::FieldReader<bool, bool>);
impl ADC2_THRES_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC2_THRES_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC2_THRES_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC2_THRES_EN` writer - Enable ADC2 threshold monitor."]
pub struct ADC2_THRES_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC2_THRES_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 30)) | ((value as u32 & 1) << 30);
        self.w
    }
}
#[doc = "Field `ADC1_THRES_EN` reader - Enable ADC1 threshold monitor."]
pub struct ADC1_THRES_EN_R(crate::FieldReader<bool, bool>);
impl ADC1_THRES_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC1_THRES_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC1_THRES_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC1_THRES_EN` writer - Enable ADC1 threshold monitor."]
pub struct ADC1_THRES_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC1_THRES_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 31)) | ((value as u32 & 1) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Clock gate enable."]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - 1: ADC_DATA > = threshold, generate interrupt. 0: ADC_DATA < threshold, generate interrupt."]
    #[inline(always)]
    pub fn adc2_thres_mode(&self) -> ADC2_THRES_MODE_R {
        ADC2_THRES_MODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1: ADC_DATA > = threshold, generate interrupt. 0: ADC_DATA < threshold, generate interrupt."]
    #[inline(always)]
    pub fn adc1_thres_mode(&self) -> ADC1_THRES_MODE_R {
        ADC1_THRES_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:16 - ADC2 threshold."]
    #[inline(always)]
    pub fn adc2_thres(&self) -> ADC2_THRES_R {
        ADC2_THRES_R::new(((self.bits >> 4) & 0x1fff) as u16)
    }
    #[doc = "Bits 17:29 - ADC1 threshold."]
    #[inline(always)]
    pub fn adc1_thres(&self) -> ADC1_THRES_R {
        ADC1_THRES_R::new(((self.bits >> 17) & 0x1fff) as u16)
    }
    #[doc = "Bit 30 - Enable ADC2 threshold monitor."]
    #[inline(always)]
    pub fn adc2_thres_en(&self) -> ADC2_THRES_EN_R {
        ADC2_THRES_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable ADC1 threshold monitor."]
    #[inline(always)]
    pub fn adc1_thres_en(&self) -> ADC1_THRES_EN_R {
        ADC1_THRES_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock gate enable."]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W {
        CLK_EN_W { w: self }
    }
    #[doc = "Bit 2 - 1: ADC_DATA > = threshold, generate interrupt. 0: ADC_DATA < threshold, generate interrupt."]
    #[inline(always)]
    pub fn adc2_thres_mode(&mut self) -> ADC2_THRES_MODE_W {
        ADC2_THRES_MODE_W { w: self }
    }
    #[doc = "Bit 3 - 1: ADC_DATA > = threshold, generate interrupt. 0: ADC_DATA < threshold, generate interrupt."]
    #[inline(always)]
    pub fn adc1_thres_mode(&mut self) -> ADC1_THRES_MODE_W {
        ADC1_THRES_MODE_W { w: self }
    }
    #[doc = "Bits 4:16 - ADC2 threshold."]
    #[inline(always)]
    pub fn adc2_thres(&mut self) -> ADC2_THRES_W {
        ADC2_THRES_W { w: self }
    }
    #[doc = "Bits 17:29 - ADC1 threshold."]
    #[inline(always)]
    pub fn adc1_thres(&mut self) -> ADC1_THRES_W {
        ADC1_THRES_W { w: self }
    }
    #[doc = "Bit 30 - Enable ADC2 threshold monitor."]
    #[inline(always)]
    pub fn adc2_thres_en(&mut self) -> ADC2_THRES_EN_W {
        ADC2_THRES_EN_W { w: self }
    }
    #[doc = "Bit 31 - Enable ADC1 threshold monitor."]
    #[inline(always)]
    pub fn adc1_thres_en(&mut self) -> ADC1_THRES_EN_W {
        ADC1_THRES_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configure monitor threshold for DIG ADC2\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [thres_ctrl]
(index.html) module"]
pub struct THRES_CTRL_SPEC;
impl crate::RegisterSpec for THRES_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [thres_ctrl::R]
(R) reader structure"]
impl crate::Readable for THRES_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [thres_ctrl::W]
(W) writer structure"]
impl crate::Writable for THRES_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets THRES_CTRL to value 0"]
impl crate::Resettable for THRES_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}