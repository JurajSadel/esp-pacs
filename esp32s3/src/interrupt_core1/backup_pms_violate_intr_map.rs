#[doc = "Register `BACKUP_PMS_VIOLATE_INTR_MAP` reader"]
pub struct R(crate::R<BACKUP_PMS_VIOLATE_INTR_MAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BACKUP_PMS_VIOLATE_INTR_MAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BACKUP_PMS_VIOLATE_INTR_MAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BACKUP_PMS_VIOLATE_INTR_MAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BACKUP_PMS_VIOLATE_INTR_MAP` writer"]
pub struct W(crate::W<BACKUP_PMS_VIOLATE_INTR_MAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BACKUP_PMS_VIOLATE_INTR_MAP_SPEC>;
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
impl From<crate::W<BACKUP_PMS_VIOLATE_INTR_MAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BACKUP_PMS_VIOLATE_INTR_MAP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BACKUP_PMS_VIOLATE_INTR_MAP` reader - this register used to map backup_pms_monitor_violatile interrupt to one of core1's external interrupt"]
pub struct BACKUP_PMS_VIOLATE_INTR_MAP_R(crate::FieldReader<u8, u8>);
impl BACKUP_PMS_VIOLATE_INTR_MAP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BACKUP_PMS_VIOLATE_INTR_MAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BACKUP_PMS_VIOLATE_INTR_MAP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BACKUP_PMS_VIOLATE_INTR_MAP` writer - this register used to map backup_pms_monitor_violatile interrupt to one of core1's external interrupt"]
pub struct BACKUP_PMS_VIOLATE_INTR_MAP_W<'a> {
    w: &'a mut W,
}
impl<'a> BACKUP_PMS_VIOLATE_INTR_MAP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - this register used to map backup_pms_monitor_violatile interrupt to one of core1's external interrupt"]
    #[inline(always)]
    pub fn backup_pms_violate_intr_map(&self) -> BACKUP_PMS_VIOLATE_INTR_MAP_R {
        BACKUP_PMS_VIOLATE_INTR_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - this register used to map backup_pms_monitor_violatile interrupt to one of core1's external interrupt"]
    #[inline(always)]
    pub fn backup_pms_violate_intr_map(&mut self) -> BACKUP_PMS_VIOLATE_INTR_MAP_W {
        BACKUP_PMS_VIOLATE_INTR_MAP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "backup_pms_monitor_violatile interrupt configuration register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [backup_pms_violate_intr_map]
(index.html) module"]
pub struct BACKUP_PMS_VIOLATE_INTR_MAP_SPEC;
impl crate::RegisterSpec for BACKUP_PMS_VIOLATE_INTR_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [backup_pms_violate_intr_map::R]
(R) reader structure"]
impl crate::Readable for BACKUP_PMS_VIOLATE_INTR_MAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [backup_pms_violate_intr_map::W]
(W) writer structure"]
impl crate::Writable for BACKUP_PMS_VIOLATE_INTR_MAP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BACKUP_PMS_VIOLATE_INTR_MAP to value 0x10"]
impl crate::Resettable for BACKUP_PMS_VIOLATE_INTR_MAP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}