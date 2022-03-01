#[doc = "Register `FLASH_ACE3_SIZE` reader"]
pub struct R(crate::R<FLASH_ACE3_SIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_ACE3_SIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_ACE3_SIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_ACE3_SIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLASH_ACE3_SIZE` writer"]
pub struct W(crate::W<FLASH_ACE3_SIZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_ACE3_SIZE_SPEC>;
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
impl From<crate::W<FLASH_ACE3_SIZE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_ACE3_SIZE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLASH_ACE3_SIZE` reader - reg_flash_ace3_size"]
pub struct FLASH_ACE3_SIZE_R(crate::FieldReader<u16, u16>);
impl FLASH_ACE3_SIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        FLASH_ACE3_SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASH_ACE3_SIZE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH_ACE3_SIZE` writer - reg_flash_ace3_size"]
pub struct FLASH_ACE3_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_ACE3_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff) | (value as u32 & 0x1fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:12 - reg_flash_ace3_size"]
    #[inline(always)]
    pub fn flash_ace3_size(&self) -> FLASH_ACE3_SIZE_R {
        FLASH_ACE3_SIZE_R::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - reg_flash_ace3_size"]
    #[inline(always)]
    pub fn flash_ace3_size(&mut self) -> FLASH_ACE3_SIZE_W {
        FLASH_ACE3_SIZE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB_CTRL_FLASH_ACE3_SIZE_REG\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_ace3_size]
(index.html) module"]
pub struct FLASH_ACE3_SIZE_SPEC;
impl crate::RegisterSpec for FLASH_ACE3_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flash_ace3_size::R]
(R) reader structure"]
impl crate::Readable for FLASH_ACE3_SIZE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flash_ace3_size::W]
(W) writer structure"]
impl crate::Writable for FLASH_ACE3_SIZE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLASH_ACE3_SIZE to value 0x0400"]
impl crate::Resettable for FLASH_ACE3_SIZE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0400
    }
}