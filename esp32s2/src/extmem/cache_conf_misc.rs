#[doc = "Register `CACHE_CONF_MISC` reader"]
pub struct R(crate::R<CACHE_CONF_MISC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CACHE_CONF_MISC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CACHE_CONF_MISC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CACHE_CONF_MISC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CACHE_CONF_MISC` writer"]
pub struct W(crate::W<CACHE_CONF_MISC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CACHE_CONF_MISC_SPEC>;
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
impl From<crate::W<CACHE_CONF_MISC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CACHE_CONF_MISC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRO_CACHE_IGNORE_PRELOAD_MMU_ENTRY_FAULT` reader - The bit is used to disable checking mmu entry fault by preload operation."]
pub struct PRO_CACHE_IGNORE_PRELOAD_MMU_ENTRY_FAULT_R(crate::FieldReader<bool, bool>);
impl PRO_CACHE_IGNORE_PRELOAD_MMU_ENTRY_FAULT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_CACHE_IGNORE_PRELOAD_MMU_ENTRY_FAULT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_CACHE_IGNORE_PRELOAD_MMU_ENTRY_FAULT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_CACHE_IGNORE_PRELOAD_MMU_ENTRY_FAULT` writer - The bit is used to disable checking mmu entry fault by preload operation."]
pub struct PRO_CACHE_IGNORE_PRELOAD_MMU_ENTRY_FAULT_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_CACHE_IGNORE_PRELOAD_MMU_ENTRY_FAULT_W<'a> {
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
        self.w.bits = (self.w.bits & !1) | (value as u32 & 1);
        self.w
    }
}
#[doc = "Field `PRO_CACHE_IGNORE_SYNC_MMU_ENTRY_FAULT` reader - The bit is used to disable checking mmu entry fault by sync operation."]
pub struct PRO_CACHE_IGNORE_SYNC_MMU_ENTRY_FAULT_R(crate::FieldReader<bool, bool>);
impl PRO_CACHE_IGNORE_SYNC_MMU_ENTRY_FAULT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_CACHE_IGNORE_SYNC_MMU_ENTRY_FAULT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_CACHE_IGNORE_SYNC_MMU_ENTRY_FAULT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_CACHE_IGNORE_SYNC_MMU_ENTRY_FAULT` writer - The bit is used to disable checking mmu entry fault by sync operation."]
pub struct PRO_CACHE_IGNORE_SYNC_MMU_ENTRY_FAULT_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_CACHE_IGNORE_SYNC_MMU_ENTRY_FAULT_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u32 & 1) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - The bit is used to disable checking mmu entry fault by preload operation."]
    #[inline(always)]
    pub fn pro_cache_ignore_preload_mmu_entry_fault(
        &self,
    ) -> PRO_CACHE_IGNORE_PRELOAD_MMU_ENTRY_FAULT_R {
        PRO_CACHE_IGNORE_PRELOAD_MMU_ENTRY_FAULT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to disable checking mmu entry fault by sync operation."]
    #[inline(always)]
    pub fn pro_cache_ignore_sync_mmu_entry_fault(&self) -> PRO_CACHE_IGNORE_SYNC_MMU_ENTRY_FAULT_R {
        PRO_CACHE_IGNORE_SYNC_MMU_ENTRY_FAULT_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to disable checking mmu entry fault by preload operation."]
    #[inline(always)]
    pub fn pro_cache_ignore_preload_mmu_entry_fault(
        &mut self,
    ) -> PRO_CACHE_IGNORE_PRELOAD_MMU_ENTRY_FAULT_W {
        PRO_CACHE_IGNORE_PRELOAD_MMU_ENTRY_FAULT_W { w: self }
    }
    #[doc = "Bit 1 - The bit is used to disable checking mmu entry fault by sync operation."]
    #[inline(always)]
    pub fn pro_cache_ignore_sync_mmu_entry_fault(
        &mut self,
    ) -> PRO_CACHE_IGNORE_SYNC_MMU_ENTRY_FAULT_W {
        PRO_CACHE_IGNORE_SYNC_MMU_ENTRY_FAULT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cache_conf_misc]
(index.html) module"]
pub struct CACHE_CONF_MISC_SPEC;
impl crate::RegisterSpec for CACHE_CONF_MISC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cache_conf_misc::R]
(R) reader structure"]
impl crate::Readable for CACHE_CONF_MISC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cache_conf_misc::W]
(W) writer structure"]
impl crate::Writable for CACHE_CONF_MISC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CACHE_CONF_MISC to value 0x03"]
impl crate::Resettable for CACHE_CONF_MISC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03
    }
}