#[doc = "Register `BLK3_RDATA4` reader"]
pub struct R(crate::R<BLK3_RDATA4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLK3_RDATA4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLK3_RDATA4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLK3_RDATA4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLK3_RDATA4` writer"]
pub struct W(crate::W<BLK3_RDATA4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLK3_RDATA4_SPEC>;
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
impl From<crate::W<BLK3_RDATA4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLK3_RDATA4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BLK3_DOUT4` reader - read for BLOCK3"]
pub struct BLK3_DOUT4_R(crate::FieldReader<u32, u32>);
impl BLK3_DOUT4_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        BLK3_DOUT4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLK3_DOUT4_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RD_CAL_RESERVED` reader - Reserved for future calibration use. Indicated by EFUSE_RD_BLK3_PART_RESERVE"]
pub struct RD_CAL_RESERVED_R(crate::FieldReader<u16, u16>);
impl RD_CAL_RESERVED_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        RD_CAL_RESERVED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_CAL_RESERVED_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RD_CAL_RESERVED` writer - Reserved for future calibration use. Indicated by EFUSE_RD_BLK3_PART_RESERVE"]
pub struct RD_CAL_RESERVED_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_CAL_RESERVED_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - read for BLOCK3"]
    #[inline(always)]
    pub fn blk3_dout4(&self) -> BLK3_DOUT4_R {
        BLK3_DOUT4_R::new(self.bits)
    }
    #[doc = "Bits 0:15 - Reserved for future calibration use. Indicated by EFUSE_RD_BLK3_PART_RESERVE"]
    #[inline(always)]
    pub fn rd_cal_reserved(&self) -> RD_CAL_RESERVED_R {
        RD_CAL_RESERVED_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Reserved for future calibration use. Indicated by EFUSE_RD_BLK3_PART_RESERVE"]
    #[inline(always)]
    pub fn rd_cal_reserved(&mut self) -> RD_CAL_RESERVED_W {
        RD_CAL_RESERVED_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blk3_rdata4]
(index.html) module"]
pub struct BLK3_RDATA4_SPEC;
impl crate::RegisterSpec for BLK3_RDATA4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blk3_rdata4::R]
(R) reader structure"]
impl crate::Readable for BLK3_RDATA4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [blk3_rdata4::W]
(W) writer structure"]
impl crate::Writable for BLK3_RDATA4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BLK3_RDATA4 to value 0"]
impl crate::Resettable for BLK3_RDATA4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}