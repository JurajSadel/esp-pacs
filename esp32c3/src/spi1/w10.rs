#[doc = "Register `W10` reader"]
pub struct R(crate::R<W10_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<W10_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<W10_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<W10_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `W10` writer"]
pub struct W(crate::W<W10_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<W10_SPEC>;
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
impl From<crate::W<W10_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<W10_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUF10` reader - data buffer"]
pub struct BUF10_R(crate::FieldReader<u32, u32>);
impl BUF10_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        BUF10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUF10_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUF10` writer - data buffer"]
pub struct BUF10_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    pub fn buf10(&self) -> BUF10_R {
        BUF10_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    pub fn buf10(&mut self) -> BUF10_W {
        BUF10_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI1 memory data buffer10\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [w10]
(index.html) module"]
pub struct W10_SPEC;
impl crate::RegisterSpec for W10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [w10::R]
(R) reader structure"]
impl crate::Readable for W10_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [w10::W]
(W) writer structure"]
impl crate::Writable for W10_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets W10 to value 0"]
impl crate::Resettable for W10_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}