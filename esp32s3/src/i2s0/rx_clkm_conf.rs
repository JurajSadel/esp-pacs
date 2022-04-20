#[doc = "Register `RX_CLKM_CONF` reader"]
pub struct R(crate::R<RX_CLKM_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_CLKM_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_CLKM_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_CLKM_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RX_CLKM_CONF` writer"]
pub struct W(crate::W<RX_CLKM_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RX_CLKM_CONF_SPEC>;
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
impl From<crate::W<RX_CLKM_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RX_CLKM_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_CLKM_DIV_NUM` reader - Integral I2S clock divider value"]
pub struct RX_CLKM_DIV_NUM_R(crate::FieldReader<u8, u8>);
impl RX_CLKM_DIV_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RX_CLKM_DIV_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_CLKM_DIV_NUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_CLKM_DIV_NUM` writer - Integral I2S clock divider value"]
pub struct RX_CLKM_DIV_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_CLKM_DIV_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `RX_CLK_ACTIVE` reader - I2S Rx module clock enable signal."]
pub struct RX_CLK_ACTIVE_R(crate::FieldReader<bool, bool>);
impl RX_CLK_ACTIVE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_CLK_ACTIVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_CLK_ACTIVE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_CLK_ACTIVE` writer - I2S Rx module clock enable signal."]
pub struct RX_CLK_ACTIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_CLK_ACTIVE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 26)) | ((value as u32 & 1) << 26);
        self.w
    }
}
#[doc = "Field `RX_CLK_SEL` reader - Select I2S Rx module source clock. 0: no clock. 1: APLL. 2: CLK160. 3: I2S_MCLK_in."]
pub struct RX_CLK_SEL_R(crate::FieldReader<u8, u8>);
impl RX_CLK_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RX_CLK_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_CLK_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_CLK_SEL` writer - Select I2S Rx module source clock. 0: no clock. 1: APLL. 2: CLK160. 3: I2S_MCLK_in."]
pub struct RX_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_CLK_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 27)) | ((value as u32 & 3) << 27);
        self.w
    }
}
#[doc = "Field `MCLK_SEL` reader - 0: UseI2S Tx module clock as I2S_MCLK_OUT. 1: UseI2S Rx module clock as I2S_MCLK_OUT."]
pub struct MCLK_SEL_R(crate::FieldReader<bool, bool>);
impl MCLK_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MCLK_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCLK_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCLK_SEL` writer - 0: UseI2S Tx module clock as I2S_MCLK_OUT. 1: UseI2S Rx module clock as I2S_MCLK_OUT."]
pub struct MCLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MCLK_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 29)) | ((value as u32 & 1) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Integral I2S clock divider value"]
    #[inline(always)]
    pub fn rx_clkm_div_num(&self) -> RX_CLKM_DIV_NUM_R {
        RX_CLKM_DIV_NUM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 26 - I2S Rx module clock enable signal."]
    #[inline(always)]
    pub fn rx_clk_active(&self) -> RX_CLK_ACTIVE_R {
        RX_CLK_ACTIVE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28 - Select I2S Rx module source clock. 0: no clock. 1: APLL. 2: CLK160. 3: I2S_MCLK_in."]
    #[inline(always)]
    pub fn rx_clk_sel(&self) -> RX_CLK_SEL_R {
        RX_CLK_SEL_R::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bit 29 - 0: UseI2S Tx module clock as I2S_MCLK_OUT. 1: UseI2S Rx module clock as I2S_MCLK_OUT."]
    #[inline(always)]
    pub fn mclk_sel(&self) -> MCLK_SEL_R {
        MCLK_SEL_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Integral I2S clock divider value"]
    #[inline(always)]
    pub fn rx_clkm_div_num(&mut self) -> RX_CLKM_DIV_NUM_W {
        RX_CLKM_DIV_NUM_W { w: self }
    }
    #[doc = "Bit 26 - I2S Rx module clock enable signal."]
    #[inline(always)]
    pub fn rx_clk_active(&mut self) -> RX_CLK_ACTIVE_W {
        RX_CLK_ACTIVE_W { w: self }
    }
    #[doc = "Bits 27:28 - Select I2S Rx module source clock. 0: no clock. 1: APLL. 2: CLK160. 3: I2S_MCLK_in."]
    #[inline(always)]
    pub fn rx_clk_sel(&mut self) -> RX_CLK_SEL_W {
        RX_CLK_SEL_W { w: self }
    }
    #[doc = "Bit 29 - 0: UseI2S Tx module clock as I2S_MCLK_OUT. 1: UseI2S Rx module clock as I2S_MCLK_OUT."]
    #[inline(always)]
    pub fn mclk_sel(&mut self) -> MCLK_SEL_W {
        MCLK_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S RX clock configure register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_clkm_conf]
(index.html) module"]
pub struct RX_CLKM_CONF_SPEC;
impl crate::RegisterSpec for RX_CLKM_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_clkm_conf::R]
(R) reader structure"]
impl crate::Readable for RX_CLKM_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rx_clkm_conf::W]
(W) writer structure"]
impl crate::Writable for RX_CLKM_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RX_CLKM_CONF to value 0x02"]
impl crate::Resettable for RX_CLKM_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}