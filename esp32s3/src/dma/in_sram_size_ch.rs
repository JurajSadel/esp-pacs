#[doc = "Register `IN_SRAM_SIZE_CH%s` reader"]
pub struct R(crate::R<IN_SRAM_SIZE_CH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IN_SRAM_SIZE_CH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IN_SRAM_SIZE_CH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IN_SRAM_SIZE_CH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IN_SRAM_SIZE_CH%s` writer"]
pub struct W(crate::W<IN_SRAM_SIZE_CH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IN_SRAM_SIZE_CH_SPEC>;
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
impl From<crate::W<IN_SRAM_SIZE_CH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IN_SRAM_SIZE_CH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IN_SIZE_CH` reader - This register is used to configure the size of L2 Tx FIFO for Rx channel 0. 0:16 bytes. 1:24 bytes. 2:32 bytes. 3: 40 bytes. 4: 48 bytes. 5:56 bytes. 6: 64 bytes. 7: 72 bytes. 8: 80 bytes."]
pub struct IN_SIZE_CH_R(crate::FieldReader<u8, u8>);
impl IN_SIZE_CH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IN_SIZE_CH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN_SIZE_CH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN_SIZE_CH` writer - This register is used to configure the size of L2 Tx FIFO for Rx channel 0. 0:16 bytes. 1:24 bytes. 2:32 bytes. 3: 40 bytes. 4: 48 bytes. 5:56 bytes. 6: 64 bytes. 7: 72 bytes. 8: 80 bytes."]
pub struct IN_SIZE_CH_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_SIZE_CH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - This register is used to configure the size of L2 Tx FIFO for Rx channel 0. 0:16 bytes. 1:24 bytes. 2:32 bytes. 3: 40 bytes. 4: 48 bytes. 5:56 bytes. 6: 64 bytes. 7: 72 bytes. 8: 80 bytes."]
    #[inline(always)]
    pub fn in_size_ch(&self) -> IN_SIZE_CH_R {
        IN_SIZE_CH_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - This register is used to configure the size of L2 Tx FIFO for Rx channel 0. 0:16 bytes. 1:24 bytes. 2:32 bytes. 3: 40 bytes. 4: 48 bytes. 5:56 bytes. 6: 64 bytes. 7: 72 bytes. 8: 80 bytes."]
    #[inline(always)]
    pub fn in_size_ch(&mut self) -> IN_SIZE_CH_W {
        IN_SIZE_CH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive L2 FIFO depth of Rx channel 0\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [in_sram_size_ch]
(index.html) module"]
pub struct IN_SRAM_SIZE_CH_SPEC;
impl crate::RegisterSpec for IN_SRAM_SIZE_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [in_sram_size_ch::R]
(R) reader structure"]
impl crate::Readable for IN_SRAM_SIZE_CH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [in_sram_size_ch::W]
(W) writer structure"]
impl crate::Writable for IN_SRAM_SIZE_CH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IN_SRAM_SIZE_CH%s to value 0x0e"]
impl crate::Resettable for IN_SRAM_SIZE_CH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0e
    }
}