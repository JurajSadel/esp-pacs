#[doc = "Register `REDUNDANCY_SIG1` reader"]
pub struct R(crate::R<REDUNDANCY_SIG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REDUNDANCY_SIG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REDUNDANCY_SIG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REDUNDANCY_SIG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REDUNDANCY_SIG1` writer"]
pub struct W(crate::W<REDUNDANCY_SIG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REDUNDANCY_SIG1_SPEC>;
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
impl From<crate::W<REDUNDANCY_SIG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REDUNDANCY_SIG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CACHE_REDCY_SIG1` reader - Those bits are prepared for ECO."]
pub type CACHE_REDCY_SIG1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CACHE_REDCY_SIG1` writer - Those bits are prepared for ECO."]
pub type CACHE_REDCY_SIG1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, REDUNDANCY_SIG1_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Those bits are prepared for ECO."]
    #[inline(always)]
    pub fn cache_redcy_sig1(&self) -> CACHE_REDCY_SIG1_R {
        CACHE_REDCY_SIG1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Those bits are prepared for ECO."]
    #[inline(always)]
    #[must_use]
    pub fn cache_redcy_sig1(&mut self) -> CACHE_REDCY_SIG1_W<0> {
        CACHE_REDCY_SIG1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cache redundancy signal 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [redundancy_sig1](index.html) module"]
pub struct REDUNDANCY_SIG1_SPEC;
impl crate::RegisterSpec for REDUNDANCY_SIG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [redundancy_sig1::R](R) reader structure"]
impl crate::Readable for REDUNDANCY_SIG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [redundancy_sig1::W](W) writer structure"]
impl crate::Writable for REDUNDANCY_SIG1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REDUNDANCY_SIG1 to value 0"]
impl crate::Resettable for REDUNDANCY_SIG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}