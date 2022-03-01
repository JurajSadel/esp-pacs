#[doc = "Register `PRO_DCACHE_LOCK0_SIZE` reader"]
pub struct R(crate::R<PRO_DCACHE_LOCK0_SIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_DCACHE_LOCK0_SIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_DCACHE_LOCK0_SIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_DCACHE_LOCK0_SIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRO_DCACHE_LOCK0_SIZE` writer"]
pub struct W(crate::W<PRO_DCACHE_LOCK0_SIZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRO_DCACHE_LOCK0_SIZE_SPEC>;
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
impl From<crate::W<PRO_DCACHE_LOCK0_SIZE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRO_DCACHE_LOCK0_SIZE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRO_DCACHE_LOCK0_SIZE` reader - The bits are used to configure the first length of data locking, which is combined with PRO_DCACHE_LOCK0_ADDR_REG"]
pub struct PRO_DCACHE_LOCK0_SIZE_R(crate::FieldReader<u16, u16>);
impl PRO_DCACHE_LOCK0_SIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        PRO_DCACHE_LOCK0_SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_DCACHE_LOCK0_SIZE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_DCACHE_LOCK0_SIZE` writer - The bits are used to configure the first length of data locking, which is combined with PRO_DCACHE_LOCK0_ADDR_REG"]
pub struct PRO_DCACHE_LOCK0_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_DCACHE_LOCK0_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - The bits are used to configure the first length of data locking, which is combined with PRO_DCACHE_LOCK0_ADDR_REG"]
    #[inline(always)]
    pub fn pro_dcache_lock0_size(&self) -> PRO_DCACHE_LOCK0_SIZE_R {
        PRO_DCACHE_LOCK0_SIZE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The bits are used to configure the first length of data locking, which is combined with PRO_DCACHE_LOCK0_ADDR_REG"]
    #[inline(always)]
    pub fn pro_dcache_lock0_size(&mut self) -> PRO_DCACHE_LOCK0_SIZE_W {
        PRO_DCACHE_LOCK0_SIZE_W { w: self }
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
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_dcache_lock0_size]
(index.html) module"]
pub struct PRO_DCACHE_LOCK0_SIZE_SPEC;
impl crate::RegisterSpec for PRO_DCACHE_LOCK0_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_dcache_lock0_size::R]
(R) reader structure"]
impl crate::Readable for PRO_DCACHE_LOCK0_SIZE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pro_dcache_lock0_size::W]
(W) writer structure"]
impl crate::Writable for PRO_DCACHE_LOCK0_SIZE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRO_DCACHE_LOCK0_SIZE to value 0"]
impl crate::Resettable for PRO_DCACHE_LOCK0_SIZE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}