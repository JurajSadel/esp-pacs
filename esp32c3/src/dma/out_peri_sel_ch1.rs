#[doc = "Register `OUT_PERI_SEL_CH1` reader"]
pub struct R(crate::R<OUT_PERI_SEL_CH1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUT_PERI_SEL_CH1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUT_PERI_SEL_CH1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUT_PERI_SEL_CH1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUT_PERI_SEL_CH1` writer"]
pub struct W(crate::W<OUT_PERI_SEL_CH1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUT_PERI_SEL_CH1_SPEC>;
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
impl From<crate::W<OUT_PERI_SEL_CH1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUT_PERI_SEL_CH1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PERI_OUT_SEL_CH1` reader - This register is used to select peripheral for Tx channel 1. 0:SPI2. 1: reserved. 2: UHCI0. 3: I2S0. 4: reserved. 5: reserved. 6: AES. 7: SHA. 8: ADC_DAC."]
pub struct PERI_OUT_SEL_CH1_R(crate::FieldReader<u8, u8>);
impl PERI_OUT_SEL_CH1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PERI_OUT_SEL_CH1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PERI_OUT_SEL_CH1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PERI_OUT_SEL_CH1` writer - This register is used to select peripheral for Tx channel 1. 0:SPI2. 1: reserved. 2: UHCI0. 3: I2S0. 4: reserved. 5: reserved. 6: AES. 7: SHA. 8: ADC_DAC."]
pub struct PERI_OUT_SEL_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> PERI_OUT_SEL_CH1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - This register is used to select peripheral for Tx channel 1. 0:SPI2. 1: reserved. 2: UHCI0. 3: I2S0. 4: reserved. 5: reserved. 6: AES. 7: SHA. 8: ADC_DAC."]
    #[inline(always)]
    pub fn peri_out_sel_ch1(&self) -> PERI_OUT_SEL_CH1_R {
        PERI_OUT_SEL_CH1_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - This register is used to select peripheral for Tx channel 1. 0:SPI2. 1: reserved. 2: UHCI0. 3: I2S0. 4: reserved. 5: reserved. 6: AES. 7: SHA. 8: ADC_DAC."]
    #[inline(always)]
    pub fn peri_out_sel_ch1(&mut self) -> PERI_OUT_SEL_CH1_W {
        PERI_OUT_SEL_CH1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA_OUT_PERI_SEL_CH1_REG.\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_peri_sel_ch1]
(index.html) module"]
pub struct OUT_PERI_SEL_CH1_SPEC;
impl crate::RegisterSpec for OUT_PERI_SEL_CH1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [out_peri_sel_ch1::R]
(R) reader structure"]
impl crate::Readable for OUT_PERI_SEL_CH1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [out_peri_sel_ch1::W]
(W) writer structure"]
impl crate::Writable for OUT_PERI_SEL_CH1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OUT_PERI_SEL_CH1 to value 0x3f"]
impl crate::Resettable for OUT_PERI_SEL_CH1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3f
    }
}