#[doc = "Register `DCACHE_ATOMIC_OPERATE_ENA` reader"]
pub struct R(crate::R<DCACHE_ATOMIC_OPERATE_ENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCACHE_ATOMIC_OPERATE_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCACHE_ATOMIC_OPERATE_ENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCACHE_ATOMIC_OPERATE_ENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCACHE_ATOMIC_OPERATE_ENA` writer"]
pub struct W(crate::W<DCACHE_ATOMIC_OPERATE_ENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCACHE_ATOMIC_OPERATE_ENA_SPEC>;
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
impl From<crate::W<DCACHE_ATOMIC_OPERATE_ENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCACHE_ATOMIC_OPERATE_ENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCACHE_ATOMIC_OPERATE_ENA` reader - The bit is used to activate dcache atomic operation protection. In this case, sync/lock/occupy operation can not interrupt miss-work. This feature does not work during invalidateAll operation."]
pub struct DCACHE_ATOMIC_OPERATE_ENA_R(crate::FieldReader<bool, bool>);
impl DCACHE_ATOMIC_OPERATE_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCACHE_ATOMIC_OPERATE_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCACHE_ATOMIC_OPERATE_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCACHE_ATOMIC_OPERATE_ENA` writer - The bit is used to activate dcache atomic operation protection. In this case, sync/lock/occupy operation can not interrupt miss-work. This feature does not work during invalidateAll operation."]
pub struct DCACHE_ATOMIC_OPERATE_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> DCACHE_ATOMIC_OPERATE_ENA_W<'a> {
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
    #[doc = "Bit 0 - The bit is used to activate dcache atomic operation protection. In this case, sync/lock/occupy operation can not interrupt miss-work. This feature does not work during invalidateAll operation."]
    #[inline(always)]
    pub fn dcache_atomic_operate_ena(&self) -> DCACHE_ATOMIC_OPERATE_ENA_R {
        DCACHE_ATOMIC_OPERATE_ENA_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to activate dcache atomic operation protection. In this case, sync/lock/occupy operation can not interrupt miss-work. This feature does not work during invalidateAll operation."]
    #[inline(always)]
    pub fn dcache_atomic_operate_ena(&mut self) -> DCACHE_ATOMIC_OPERATE_ENA_W {
        DCACHE_ATOMIC_OPERATE_ENA_W { w: self }
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
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcache_atomic_operate_ena]
(index.html) module"]
pub struct DCACHE_ATOMIC_OPERATE_ENA_SPEC;
impl crate::RegisterSpec for DCACHE_ATOMIC_OPERATE_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcache_atomic_operate_ena::R]
(R) reader structure"]
impl crate::Readable for DCACHE_ATOMIC_OPERATE_ENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcache_atomic_operate_ena::W]
(W) writer structure"]
impl crate::Writable for DCACHE_ATOMIC_OPERATE_ENA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCACHE_ATOMIC_OPERATE_ENA to value 0x01"]
impl crate::Resettable for DCACHE_ATOMIC_OPERATE_ENA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}