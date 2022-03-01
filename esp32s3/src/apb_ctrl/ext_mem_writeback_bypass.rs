#[doc = "Register `EXT_MEM_WRITEBACK_BYPASS` reader"]
pub struct R(crate::R<EXT_MEM_WRITEBACK_BYPASS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXT_MEM_WRITEBACK_BYPASS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXT_MEM_WRITEBACK_BYPASS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXT_MEM_WRITEBACK_BYPASS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXT_MEM_WRITEBACK_BYPASS` writer"]
pub struct W(crate::W<EXT_MEM_WRITEBACK_BYPASS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXT_MEM_WRITEBACK_BYPASS_SPEC>;
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
impl From<crate::W<EXT_MEM_WRITEBACK_BYPASS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXT_MEM_WRITEBACK_BYPASS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WRITEBACK_BYPASS` reader - Set 1 to bypass cache writeback request to external memory so that spi will not check its attribute."]
pub struct WRITEBACK_BYPASS_R(crate::FieldReader<bool, bool>);
impl WRITEBACK_BYPASS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WRITEBACK_BYPASS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRITEBACK_BYPASS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRITEBACK_BYPASS` writer - Set 1 to bypass cache writeback request to external memory so that spi will not check its attribute."]
pub struct WRITEBACK_BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> WRITEBACK_BYPASS_W<'a> {
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
    #[doc = "Bit 0 - Set 1 to bypass cache writeback request to external memory so that spi will not check its attribute."]
    #[inline(always)]
    pub fn writeback_bypass(&self) -> WRITEBACK_BYPASS_R {
        WRITEBACK_BYPASS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to bypass cache writeback request to external memory so that spi will not check its attribute."]
    #[inline(always)]
    pub fn writeback_bypass(&mut self) -> WRITEBACK_BYPASS_W {
        WRITEBACK_BYPASS_W { w: self }
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
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ext_mem_writeback_bypass]
(index.html) module"]
pub struct EXT_MEM_WRITEBACK_BYPASS_SPEC;
impl crate::RegisterSpec for EXT_MEM_WRITEBACK_BYPASS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ext_mem_writeback_bypass::R]
(R) reader structure"]
impl crate::Readable for EXT_MEM_WRITEBACK_BYPASS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ext_mem_writeback_bypass::W]
(W) writer structure"]
impl crate::Writable for EXT_MEM_WRITEBACK_BYPASS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXT_MEM_WRITEBACK_BYPASS to value 0"]
impl crate::Resettable for EXT_MEM_WRITEBACK_BYPASS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}