#[doc = "Register `CORE_0_SP_UNSTABLE` reader"]
pub struct R(crate::R<CORE_0_SP_UNSTABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_0_SP_UNSTABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_0_SP_UNSTABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_0_SP_UNSTABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORE_0_SP_UNSTABLE` writer"]
pub struct W(crate::W<CORE_0_SP_UNSTABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_0_SP_UNSTABLE_SPEC>;
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
impl From<crate::W<CORE_0_SP_UNSTABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_0_SP_UNSTABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_0_SP_UNSTABLE` reader - unstable period when window change,during this period no check stackpointer"]
pub struct CORE_0_SP_UNSTABLE_R(crate::FieldReader<u8, u8>);
impl CORE_0_SP_UNSTABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_0_SP_UNSTABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_SP_UNSTABLE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_SP_UNSTABLE` writer - unstable period when window change,during this period no check stackpointer"]
pub struct CORE_0_SP_UNSTABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_SP_UNSTABLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - unstable period when window change,during this period no check stackpointer"]
    #[inline(always)]
    pub fn core_0_sp_unstable(&self) -> CORE_0_SP_UNSTABLE_R {
        CORE_0_SP_UNSTABLE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - unstable period when window change,during this period no check stackpointer"]
    #[inline(always)]
    pub fn core_0_sp_unstable(&mut self) -> CORE_0_SP_UNSTABLE_W {
        CORE_0_SP_UNSTABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "core0 sp unstable configuration register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_0_sp_unstable]
(index.html) module"]
pub struct CORE_0_SP_UNSTABLE_SPEC;
impl crate::RegisterSpec for CORE_0_SP_UNSTABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_0_sp_unstable::R]
(R) reader structure"]
impl crate::Readable for CORE_0_SP_UNSTABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_0_sp_unstable::W]
(W) writer structure"]
impl crate::Writable for CORE_0_SP_UNSTABLE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CORE_0_SP_UNSTABLE to value 0x0b"]
impl crate::Resettable for CORE_0_SP_UNSTABLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0b
    }
}