#[doc = "Register `IN_POP_CH2` reader"]
pub struct R(crate::R<IN_POP_CH2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IN_POP_CH2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IN_POP_CH2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IN_POP_CH2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IN_POP_CH2` writer"]
pub struct W(crate::W<IN_POP_CH2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IN_POP_CH2_SPEC>;
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
impl From<crate::W<IN_POP_CH2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IN_POP_CH2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INFIFO_RDATA` reader - This register stores the data popping from DMA FIFO."]
pub type INFIFO_RDATA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `INFIFO_POP` reader - Set this bit to pop data from DMA FIFO."]
pub type INFIFO_POP_R = crate::BitReader<bool>;
#[doc = "Field `INFIFO_POP` writer - Set this bit to pop data from DMA FIFO."]
pub type INFIFO_POP_W<'a, const O: u8> = crate::BitWriter<'a, u32, IN_POP_CH2_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:11 - This register stores the data popping from DMA FIFO."]
    #[inline(always)]
    pub fn infifo_rdata(&self) -> INFIFO_RDATA_R {
        INFIFO_RDATA_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12 - Set this bit to pop data from DMA FIFO."]
    #[inline(always)]
    pub fn infifo_pop(&self) -> INFIFO_POP_R {
        INFIFO_POP_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - Set this bit to pop data from DMA FIFO."]
    #[inline(always)]
    pub fn infifo_pop(&mut self) -> INFIFO_POP_W<12> {
        INFIFO_POP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA_IN_POP_CH2_REG.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [in_pop_ch2](index.html) module"]
pub struct IN_POP_CH2_SPEC;
impl crate::RegisterSpec for IN_POP_CH2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [in_pop_ch2::R](R) reader structure"]
impl crate::Readable for IN_POP_CH2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [in_pop_ch2::W](W) writer structure"]
impl crate::Writable for IN_POP_CH2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IN_POP_CH2 to value 0x0800"]
impl crate::Resettable for IN_POP_CH2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0800
    }
}