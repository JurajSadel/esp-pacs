#[doc = "Register `Core_1_STATUSTABLE12` reader"]
pub struct R(crate::R<CORE_1_STATUSTABLE12_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_1_STATUSTABLE12_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_1_STATUSTABLE12_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_1_STATUSTABLE12_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `Core_1_STATUSTABLE12` writer"]
pub struct W(crate::W<CORE_1_STATUSTABLE12_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_1_STATUSTABLE12_SPEC>;
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
impl From<crate::W<CORE_1_STATUSTABLE12_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_1_STATUSTABLE12_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_1_FROM_WORLD_12` reader - This bit is used to confirm world before enter entry 12"]
pub struct CORE_1_FROM_WORLD_12_R(crate::FieldReader<bool, bool>);
impl CORE_1_FROM_WORLD_12_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CORE_1_FROM_WORLD_12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_1_FROM_WORLD_12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_1_FROM_WORLD_12` writer - This bit is used to confirm world before enter entry 12"]
pub struct CORE_1_FROM_WORLD_12_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_1_FROM_WORLD_12_W<'a> {
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
#[doc = "Field `CORE_1_FROM_ENTRY_12` reader - This filed is used to confirm in which entry before enter entry 12"]
pub struct CORE_1_FROM_ENTRY_12_R(crate::FieldReader<u8, u8>);
impl CORE_1_FROM_ENTRY_12_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_1_FROM_ENTRY_12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_1_FROM_ENTRY_12_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_1_FROM_ENTRY_12` writer - This filed is used to confirm in which entry before enter entry 12"]
pub struct CORE_1_FROM_ENTRY_12_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_1_FROM_ENTRY_12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 1)) | ((value as u32 & 0x0f) << 1);
        self.w
    }
}
#[doc = "Field `CORE_1_CURRENT_12` reader - This bit is used to confirm whether the current state is in entry 12"]
pub struct CORE_1_CURRENT_12_R(crate::FieldReader<bool, bool>);
impl CORE_1_CURRENT_12_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CORE_1_CURRENT_12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_1_CURRENT_12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_1_CURRENT_12` writer - This bit is used to confirm whether the current state is in entry 12"]
pub struct CORE_1_CURRENT_12_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_1_CURRENT_12_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 5)) | ((value as u32 & 1) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - This bit is used to confirm world before enter entry 12"]
    #[inline(always)]
    pub fn core_1_from_world_12(&self) -> CORE_1_FROM_WORLD_12_R {
        CORE_1_FROM_WORLD_12_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - This filed is used to confirm in which entry before enter entry 12"]
    #[inline(always)]
    pub fn core_1_from_entry_12(&self) -> CORE_1_FROM_ENTRY_12_R {
        CORE_1_FROM_ENTRY_12_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 5 - This bit is used to confirm whether the current state is in entry 12"]
    #[inline(always)]
    pub fn core_1_current_12(&self) -> CORE_1_CURRENT_12_R {
        CORE_1_CURRENT_12_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is used to confirm world before enter entry 12"]
    #[inline(always)]
    pub fn core_1_from_world_12(&mut self) -> CORE_1_FROM_WORLD_12_W {
        CORE_1_FROM_WORLD_12_W { w: self }
    }
    #[doc = "Bits 1:4 - This filed is used to confirm in which entry before enter entry 12"]
    #[inline(always)]
    pub fn core_1_from_entry_12(&mut self) -> CORE_1_FROM_ENTRY_12_W {
        CORE_1_FROM_ENTRY_12_W { w: self }
    }
    #[doc = "Bit 5 - This bit is used to confirm whether the current state is in entry 12"]
    #[inline(always)]
    pub fn core_1_current_12(&mut self) -> CORE_1_CURRENT_12_W {
        CORE_1_CURRENT_12_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status register of world switch of entry 12\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_1_statustable12]
(index.html) module"]
pub struct CORE_1_STATUSTABLE12_SPEC;
impl crate::RegisterSpec for CORE_1_STATUSTABLE12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_1_statustable12::R]
(R) reader structure"]
impl crate::Readable for CORE_1_STATUSTABLE12_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_1_statustable12::W]
(W) writer structure"]
impl crate::Writable for CORE_1_STATUSTABLE12_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets Core_1_STATUSTABLE12 to value 0"]
impl crate::Resettable for CORE_1_STATUSTABLE12_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}