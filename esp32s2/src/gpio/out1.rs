#[doc = "Register `OUT1` reader"]
pub struct R(crate::R<OUT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUT1` writer"]
pub struct W(crate::W<OUT1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUT1_SPEC>;
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
impl From<crate::W<OUT1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUT1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA_ORIG` reader - GPIO32 ~ 53 output value in simple GPIO output mode. The values of bit0 ~ bit13 correspond to GPIO32 ~ GPIO45. Bit14 ~ bit21 are invalid."]
pub struct DATA_ORIG_R(crate::FieldReader<u32, u32>);
impl DATA_ORIG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        DATA_ORIG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_ORIG_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA_ORIG` writer - GPIO32 ~ 53 output value in simple GPIO output mode. The values of bit0 ~ bit13 correspond to GPIO32 ~ GPIO45. Bit14 ~ bit21 are invalid."]
pub struct DATA_ORIG_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_ORIG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x003f_ffff) | (value as u32 & 0x003f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:21 - GPIO32 ~ 53 output value in simple GPIO output mode. The values of bit0 ~ bit13 correspond to GPIO32 ~ GPIO45. Bit14 ~ bit21 are invalid."]
    #[inline(always)]
    pub fn data_orig(&self) -> DATA_ORIG_R {
        DATA_ORIG_R::new((self.bits & 0x003f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:21 - GPIO32 ~ 53 output value in simple GPIO output mode. The values of bit0 ~ bit13 correspond to GPIO32 ~ GPIO45. Bit14 ~ bit21 are invalid."]
    #[inline(always)]
    pub fn data_orig(&mut self) -> DATA_ORIG_W {
        DATA_ORIG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO32 ~ 53 output register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out1]
(index.html) module"]
pub struct OUT1_SPEC;
impl crate::RegisterSpec for OUT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [out1::R]
(R) reader structure"]
impl crate::Readable for OUT1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [out1::W]
(W) writer structure"]
impl crate::Writable for OUT1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OUT1 to value 0"]
impl crate::Resettable for OUT1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}