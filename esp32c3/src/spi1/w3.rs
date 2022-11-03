#[doc = "Register `W3` reader"]
pub struct R(crate::R<W3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<W3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<W3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<W3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `W3` writer"]
pub struct W(crate::W<W3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<W3_SPEC>;
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
impl From<crate::W<W3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<W3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUF3` reader - data buffer"]
pub type BUF3_R = crate::FieldReader<u32, u32>;
#[doc = "Field `BUF3` writer - data buffer"]
pub type BUF3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, W3_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    pub fn buf3(&self) -> BUF3_R {
        BUF3_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    pub fn buf3(&mut self) -> BUF3_W<0> {
        BUF3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI1 memory data buffer3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [w3](index.html) module"]
pub struct W3_SPEC;
impl crate::RegisterSpec for W3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [w3::R](R) reader structure"]
impl crate::Readable for W3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [w3::W](W) writer structure"]
impl crate::Writable for W3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets W3 to value 0"]
impl crate::Resettable for W3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}