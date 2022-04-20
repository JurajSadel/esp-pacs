#[doc = "Register `OUT_CONF1_CH0` reader"]
pub struct R(crate::R<OUT_CONF1_CH0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUT_CONF1_CH0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUT_CONF1_CH0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUT_CONF1_CH0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUT_CONF1_CH0` writer"]
pub struct W(crate::W<OUT_CONF1_CH0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUT_CONF1_CH0_SPEC>;
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
impl From<crate::W<OUT_CONF1_CH0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUT_CONF1_CH0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUT_CHECK_OWNER_CH0` reader - Set this bit to enable checking the owner attribute of the link descriptor."]
pub struct OUT_CHECK_OWNER_CH0_R(crate::FieldReader<bool, bool>);
impl OUT_CHECK_OWNER_CH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUT_CHECK_OWNER_CH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_CHECK_OWNER_CH0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT_CHECK_OWNER_CH0` writer - Set this bit to enable checking the owner attribute of the link descriptor."]
pub struct OUT_CHECK_OWNER_CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_CHECK_OWNER_CH0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 12)) | ((value as u32 & 1) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 12 - Set this bit to enable checking the owner attribute of the link descriptor."]
    #[inline(always)]
    pub fn out_check_owner_ch0(&self) -> OUT_CHECK_OWNER_CH0_R {
        OUT_CHECK_OWNER_CH0_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - Set this bit to enable checking the owner attribute of the link descriptor."]
    #[inline(always)]
    pub fn out_check_owner_ch0(&mut self) -> OUT_CHECK_OWNER_CH0_W {
        OUT_CHECK_OWNER_CH0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA_OUT_CONF1_CH0_REG.\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_conf1_ch0]
(index.html) module"]
pub struct OUT_CONF1_CH0_SPEC;
impl crate::RegisterSpec for OUT_CONF1_CH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [out_conf1_ch0::R]
(R) reader structure"]
impl crate::Readable for OUT_CONF1_CH0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [out_conf1_ch0::W]
(W) writer structure"]
impl crate::Writable for OUT_CONF1_CH0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OUT_CONF1_CH0 to value 0"]
impl crate::Resettable for OUT_CONF1_CH0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}