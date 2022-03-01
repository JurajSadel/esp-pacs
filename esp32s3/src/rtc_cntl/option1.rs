#[doc = "Register `OPTION1` reader"]
pub struct R(crate::R<OPTION1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPTION1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPTION1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPTION1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPTION1` writer"]
pub struct W(crate::W<OPTION1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPTION1_SPEC>;
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
impl From<crate::W<OPTION1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPTION1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FORCE_DOWNLOAD_BOOT` reader - force chip entry download boot by sw"]
pub struct FORCE_DOWNLOAD_BOOT_R(crate::FieldReader<bool, bool>);
impl FORCE_DOWNLOAD_BOOT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FORCE_DOWNLOAD_BOOT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FORCE_DOWNLOAD_BOOT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FORCE_DOWNLOAD_BOOT` writer - force chip entry download boot by sw"]
pub struct FORCE_DOWNLOAD_BOOT_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_DOWNLOAD_BOOT_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - force chip entry download boot by sw"]
    #[inline(always)]
    pub fn force_download_boot(&self) -> FORCE_DOWNLOAD_BOOT_R {
        FORCE_DOWNLOAD_BOOT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - force chip entry download boot by sw"]
    #[inline(always)]
    pub fn force_download_boot(&mut self) -> FORCE_DOWNLOAD_BOOT_W {
        FORCE_DOWNLOAD_BOOT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rtc common configure\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [option1]
(index.html) module"]
pub struct OPTION1_SPEC;
impl crate::RegisterSpec for OPTION1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [option1::R]
(R) reader structure"]
impl crate::Readable for OPTION1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [option1::W]
(W) writer structure"]
impl crate::Writable for OPTION1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OPTION1 to value 0"]
impl crate::Resettable for OPTION1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}