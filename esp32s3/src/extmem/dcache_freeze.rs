#[doc = "Register `DCACHE_FREEZE` reader"]
pub struct R(crate::R<DCACHE_FREEZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCACHE_FREEZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCACHE_FREEZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCACHE_FREEZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCACHE_FREEZE` writer"]
pub struct W(crate::W<DCACHE_FREEZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCACHE_FREEZE_SPEC>;
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
impl From<crate::W<DCACHE_FREEZE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCACHE_FREEZE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENA` reader - The bit is used to enable dcache freeze mode"]
pub struct ENA_R(crate::FieldReader<bool, bool>);
impl ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENA` writer - The bit is used to enable dcache freeze mode"]
pub struct ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA_W<'a> {
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
#[doc = "Field `MODE` reader - The bit is used to configure freeze mode, 0: assert busy if CPU miss 1: assert hit if CPU miss"]
pub struct MODE_R(crate::FieldReader<bool, bool>);
impl MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE` writer - The bit is used to configure freeze mode, 0: assert busy if CPU miss 1: assert hit if CPU miss"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `DONE` reader - The bit is used to indicate dcache freeze success"]
pub struct DONE_R(crate::FieldReader<bool, bool>);
impl DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - The bit is used to enable dcache freeze mode"]
    #[inline(always)]
    pub fn ena(&self) -> ENA_R {
        ENA_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - The bit is used to configure freeze mode, 0: assert busy if CPU miss 1: assert hit if CPU miss"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - The bit is used to indicate dcache freeze success"]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to enable dcache freeze mode"]
    #[inline(always)]
    pub fn ena(&mut self) -> ENA_W {
        ENA_W { w: self }
    }
    #[doc = "Bit 1 - The bit is used to configure freeze mode, 0: assert busy if CPU miss 1: assert hit if CPU miss"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "******* Description ***********\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcache_freeze]
(index.html) module"]
pub struct DCACHE_FREEZE_SPEC;
impl crate::RegisterSpec for DCACHE_FREEZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcache_freeze::R]
(R) reader structure"]
impl crate::Readable for DCACHE_FREEZE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcache_freeze::W]
(W) writer structure"]
impl crate::Writable for DCACHE_FREEZE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCACHE_FREEZE to value 0x04"]
impl crate::Resettable for DCACHE_FREEZE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x04
    }
}