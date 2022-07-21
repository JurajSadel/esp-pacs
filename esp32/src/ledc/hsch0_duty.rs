#[doc = "Register `HSCH0_DUTY` reader"]
pub struct R(crate::R<HSCH0_DUTY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSCH0_DUTY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSCH0_DUTY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSCH0_DUTY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HSCH0_DUTY` writer"]
pub struct W(crate::W<HSCH0_DUTY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSCH0_DUTY_SPEC>;
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
impl From<crate::W<HSCH0_DUTY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSCH0_DUTY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DUTY_HSCH0` reader - "]
pub type DUTY_HSCH0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DUTY_HSCH0` writer - "]
pub type DUTY_HSCH0_W<'a> = crate::FieldWriter<'a, u32, HSCH0_DUTY_SPEC, u32, u32, 25, 0>;
impl R {
    #[doc = "Bits 0:24"]
    #[inline(always)]
    pub fn duty_hsch0(&self) -> DUTY_HSCH0_R {
        DUTY_HSCH0_R::new((self.bits & 0x01ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:24"]
    #[inline(always)]
    pub fn duty_hsch0(&mut self) -> DUTY_HSCH0_W {
        DUTY_HSCH0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsch0_duty](index.html) module"]
pub struct HSCH0_DUTY_SPEC;
impl crate::RegisterSpec for HSCH0_DUTY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hsch0_duty::R](R) reader structure"]
impl crate::Readable for HSCH0_DUTY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hsch0_duty::W](W) writer structure"]
impl crate::Writable for HSCH0_DUTY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HSCH0_DUTY to value 0"]
impl crate::Resettable for HSCH0_DUTY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
