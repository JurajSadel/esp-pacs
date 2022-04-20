#[doc = "Register `DMA_START` writer"]
pub struct W(crate::W<DMA_START_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_START_SPEC>;
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
impl From<crate::W<DMA_START_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_START_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA_START` writer - Start dma-sha."]
pub struct DMA_START_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_START_W<'a> {
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
impl W {
    #[doc = "Bit 0 - Start dma-sha."]
    #[inline(always)]
    pub fn dma_start(&mut self) -> DMA_START_W {
        DMA_START_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA configuration register 1.\n\nThis register you can [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_start]
(index.html) module"]
pub struct DMA_START_SPEC;
impl crate::RegisterSpec for DMA_START_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [dma_start::W]
(W) writer structure"]
impl crate::Writable for DMA_START_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_START to value 0"]
impl crate::Resettable for DMA_START_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}