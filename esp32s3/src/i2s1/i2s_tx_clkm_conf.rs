#[doc = "Register `I2S_TX_CLKM_CONF` reader"]
pub struct R(crate::R<I2S_TX_CLKM_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2S_TX_CLKM_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2S_TX_CLKM_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2S_TX_CLKM_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2S_TX_CLKM_CONF` writer"]
pub struct W(crate::W<I2S_TX_CLKM_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2S_TX_CLKM_CONF_SPEC>;
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
impl From<crate::W<I2S_TX_CLKM_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2S_TX_CLKM_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2S_TX_CLKM_DIV_NUM` reader - Integral I2S TX clock divider value. f_I2S_CLK = f_I2S_CLK_S/(N+b/a). There will be (a-b) * n-div and b * (n+1)-div. So the average combination will be: for b <= a/2, z * \\[x * n-div + (n+1)-div\\]
 + y * n-div. For b > a/2, z * \\[n-div + x * (n+1)-div\\]
 + y * (n+1)-div."]
pub struct I2S_TX_CLKM_DIV_NUM_R(crate::FieldReader<u8, u8>);
impl I2S_TX_CLKM_DIV_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        I2S_TX_CLKM_DIV_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2S_TX_CLKM_DIV_NUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2S_TX_CLKM_DIV_NUM` writer - Integral I2S TX clock divider value. f_I2S_CLK = f_I2S_CLK_S/(N+b/a). There will be (a-b) * n-div and b * (n+1)-div. So the average combination will be: for b <= a/2, z * \\[x * n-div + (n+1)-div\\]
 + y * n-div. For b > a/2, z * \\[n-div + x * (n+1)-div\\]
 + y * (n+1)-div."]
pub struct I2S_TX_CLKM_DIV_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_TX_CLKM_DIV_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `I2S_TX_CLK_ACTIVE` reader - I2S Tx module clock enable signal."]
pub struct I2S_TX_CLK_ACTIVE_R(crate::FieldReader<bool, bool>);
impl I2S_TX_CLK_ACTIVE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2S_TX_CLK_ACTIVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2S_TX_CLK_ACTIVE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2S_TX_CLK_ACTIVE` writer - I2S Tx module clock enable signal."]
pub struct I2S_TX_CLK_ACTIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_TX_CLK_ACTIVE_W<'a> {
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
#[doc = "Field `I2S_TX_CLK_SEL` reader - Select I2S Tx module source clock. 0: XTAL clock. 1: APLL. 2: CLK160. 3: I2S_MCLK_in."]
pub struct I2S_TX_CLK_SEL_R(crate::FieldReader<u8, u8>);
impl I2S_TX_CLK_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        I2S_TX_CLK_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2S_TX_CLK_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2S_TX_CLK_SEL` writer - Select I2S Tx module source clock. 0: XTAL clock. 1: APLL. 2: CLK160. 3: I2S_MCLK_in."]
pub struct I2S_TX_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_TX_CLK_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 27)) | ((value as u32 & 3) << 27);
        self.w
    }
}
#[doc = "Field `I2S_CLK_EN` reader - Set this bit to enable clk gate"]
pub struct I2S_CLK_EN_R(crate::FieldReader<bool, bool>);
impl I2S_CLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2S_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2S_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2S_CLK_EN` writer - Set this bit to enable clk gate"]
pub struct I2S_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_CLK_EN_W<'a> {
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
    #[doc = "Bits 0:7 - Integral I2S TX clock divider value. f_I2S_CLK = f_I2S_CLK_S/(N+b/a). There will be (a-b) * n-div and b * (n+1)-div. So the average combination will be: for b <= a/2, z * \\[x * n-div + (n+1)-div\\]
 + y * n-div. For b > a/2, z * \\[n-div + x * (n+1)-div\\]
 + y * (n+1)-div."]
    #[inline(always)]
    pub fn i2s_tx_clkm_div_num(&self) -> I2S_TX_CLKM_DIV_NUM_R {
        I2S_TX_CLKM_DIV_NUM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 26 - I2S Tx module clock enable signal."]
    #[inline(always)]
    pub fn i2s_tx_clk_active(&self) -> I2S_TX_CLK_ACTIVE_R {
        I2S_TX_CLK_ACTIVE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28 - Select I2S Tx module source clock. 0: XTAL clock. 1: APLL. 2: CLK160. 3: I2S_MCLK_in."]
    #[inline(always)]
    pub fn i2s_tx_clk_sel(&self) -> I2S_TX_CLK_SEL_R {
        I2S_TX_CLK_SEL_R::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bit 29 - Set this bit to enable clk gate"]
    #[inline(always)]
    pub fn i2s_clk_en(&self) -> I2S_CLK_EN_R {
        I2S_CLK_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Integral I2S TX clock divider value. f_I2S_CLK = f_I2S_CLK_S/(N+b/a). There will be (a-b) * n-div and b * (n+1)-div. So the average combination will be: for b <= a/2, z * \\[x * n-div + (n+1)-div\\]
 + y * n-div. For b > a/2, z * \\[n-div + x * (n+1)-div\\]
 + y * (n+1)-div."]
    #[inline(always)]
    pub fn i2s_tx_clkm_div_num(&mut self) -> I2S_TX_CLKM_DIV_NUM_W {
        I2S_TX_CLKM_DIV_NUM_W { w: self }
    }
    #[doc = "Bit 26 - I2S Tx module clock enable signal."]
    #[inline(always)]
    pub fn i2s_tx_clk_active(&mut self) -> I2S_TX_CLK_ACTIVE_W {
        I2S_TX_CLK_ACTIVE_W { w: self }
    }
    #[doc = "Bits 27:28 - Select I2S Tx module source clock. 0: XTAL clock. 1: APLL. 2: CLK160. 3: I2S_MCLK_in."]
    #[inline(always)]
    pub fn i2s_tx_clk_sel(&mut self) -> I2S_TX_CLK_SEL_W {
        I2S_TX_CLK_SEL_W { w: self }
    }
    #[doc = "Bit 29 - Set this bit to enable clk gate"]
    #[inline(always)]
    pub fn i2s_clk_en(&mut self) -> I2S_CLK_EN_W {
        I2S_CLK_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S TX clock configure register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_tx_clkm_conf]
(index.html) module"]
pub struct I2S_TX_CLKM_CONF_SPEC;
impl crate::RegisterSpec for I2S_TX_CLKM_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2s_tx_clkm_conf::R]
(R) reader structure"]
impl crate::Readable for I2S_TX_CLKM_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2s_tx_clkm_conf::W]
(W) writer structure"]
impl crate::Writable for I2S_TX_CLKM_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2S_TX_CLKM_CONF to value 0x02"]
impl crate::Resettable for I2S_TX_CLKM_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}