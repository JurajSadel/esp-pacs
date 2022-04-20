#[doc = "Register `DATA_9` writer"]
pub struct W(crate::W<DATA_9_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATA_9_SPEC>;
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
impl From<crate::W<DATA_9_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATA_9_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_BYTE_9` writer - Stored the 9th byte information of the data to be transmitted under operating mode."]
pub struct TX_BYTE_9_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_BYTE_9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:7 - Stored the 9th byte information of the data to be transmitted under operating mode."]
    #[inline(always)]
    pub fn tx_byte_9(&mut self) -> TX_BYTE_9_W {
        TX_BYTE_9_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data register 9\n\nThis register you can [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data_9]
(index.html) module"]
pub struct DATA_9_SPEC;
impl crate::RegisterSpec for DATA_9_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [data_9::W]
(W) writer structure"]
impl crate::Writable for DATA_9_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DATA_9 to value 0"]
impl crate::Resettable for DATA_9_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}