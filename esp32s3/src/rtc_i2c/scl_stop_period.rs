#[doc = "Register `SCL_STOP_PERIOD` reader"]
pub struct R(crate::R<SCL_STOP_PERIOD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCL_STOP_PERIOD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCL_STOP_PERIOD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCL_STOP_PERIOD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCL_STOP_PERIOD` writer"]
pub struct W(crate::W<SCL_STOP_PERIOD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCL_STOP_PERIOD_SPEC>;
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
impl From<crate::W<SCL_STOP_PERIOD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCL_STOP_PERIOD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCL_STOP_PERIOD` reader - time period for SCL to stop after I2C end is triggered"]
pub struct SCL_STOP_PERIOD_R(crate::FieldReader<u32, u32>);
impl SCL_STOP_PERIOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        SCL_STOP_PERIOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCL_STOP_PERIOD_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCL_STOP_PERIOD` writer - time period for SCL to stop after I2C end is triggered"]
pub struct SCL_STOP_PERIOD_W<'a> {
    w: &'a mut W,
}
impl<'a> SCL_STOP_PERIOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | (value as u32 & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:19 - time period for SCL to stop after I2C end is triggered"]
    #[inline(always)]
    pub fn scl_stop_period(&self) -> SCL_STOP_PERIOD_R {
        SCL_STOP_PERIOD_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:19 - time period for SCL to stop after I2C end is triggered"]
    #[inline(always)]
    pub fn scl_stop_period(&mut self) -> SCL_STOP_PERIOD_W {
        SCL_STOP_PERIOD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configure scl stop period\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scl_stop_period]
(index.html) module"]
pub struct SCL_STOP_PERIOD_SPEC;
impl crate::RegisterSpec for SCL_STOP_PERIOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scl_stop_period::R]
(R) reader structure"]
impl crate::Readable for SCL_STOP_PERIOD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scl_stop_period::W]
(W) writer structure"]
impl crate::Writable for SCL_STOP_PERIOD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCL_STOP_PERIOD to value 0x08"]
impl crate::Resettable for SCL_STOP_PERIOD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x08
    }
}