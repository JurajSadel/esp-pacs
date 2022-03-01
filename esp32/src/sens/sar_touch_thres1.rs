#[doc = "Register `SAR_TOUCH_THRES1` reader"]
pub struct R(crate::R<SAR_TOUCH_THRES1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_TOUCH_THRES1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_TOUCH_THRES1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_TOUCH_THRES1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_TOUCH_THRES1` writer"]
pub struct W(crate::W<SAR_TOUCH_THRES1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_TOUCH_THRES1_SPEC>;
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
impl From<crate::W<SAR_TOUCH_THRES1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_TOUCH_THRES1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOUCH_OUT_TH1` reader - the threshold for touch pad 1"]
pub struct TOUCH_OUT_TH1_R(crate::FieldReader<u16, u16>);
impl TOUCH_OUT_TH1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TOUCH_OUT_TH1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_OUT_TH1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_OUT_TH1` writer - the threshold for touch pad 1"]
pub struct TOUCH_OUT_TH1_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_OUT_TH1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `TOUCH_OUT_TH0` reader - the threshold for touch pad 0"]
pub struct TOUCH_OUT_TH0_R(crate::FieldReader<u16, u16>);
impl TOUCH_OUT_TH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TOUCH_OUT_TH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_OUT_TH0_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_OUT_TH0` writer - the threshold for touch pad 0"]
pub struct TOUCH_OUT_TH0_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_OUT_TH0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - the threshold for touch pad 1"]
    #[inline(always)]
    pub fn touch_out_th1(&self) -> TOUCH_OUT_TH1_R {
        TOUCH_OUT_TH1_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - the threshold for touch pad 0"]
    #[inline(always)]
    pub fn touch_out_th0(&self) -> TOUCH_OUT_TH0_R {
        TOUCH_OUT_TH0_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - the threshold for touch pad 1"]
    #[inline(always)]
    pub fn touch_out_th1(&mut self) -> TOUCH_OUT_TH1_W {
        TOUCH_OUT_TH1_W { w: self }
    }
    #[doc = "Bits 16:31 - the threshold for touch pad 0"]
    #[inline(always)]
    pub fn touch_out_th0(&mut self) -> TOUCH_OUT_TH0_W {
        TOUCH_OUT_TH0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_touch_thres1]
(index.html) module"]
pub struct SAR_TOUCH_THRES1_SPEC;
impl crate::RegisterSpec for SAR_TOUCH_THRES1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_touch_thres1::R]
(R) reader structure"]
impl crate::Readable for SAR_TOUCH_THRES1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_touch_thres1::W]
(W) writer structure"]
impl crate::Writable for SAR_TOUCH_THRES1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAR_TOUCH_THRES1 to value 0"]
impl crate::Resettable for SAR_TOUCH_THRES1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}