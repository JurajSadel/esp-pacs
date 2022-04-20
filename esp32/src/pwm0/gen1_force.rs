#[doc = "Register `GEN1_FORCE` reader"]
pub struct R(crate::R<GEN1_FORCE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GEN1_FORCE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GEN1_FORCE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GEN1_FORCE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GEN1_FORCE` writer"]
pub struct W(crate::W<GEN1_FORCE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GEN1_FORCE_SPEC>;
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
impl From<crate::W<GEN1_FORCE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GEN1_FORCE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GEN1_CNTUFORCE_UPMETHOD` reader - "]
pub struct GEN1_CNTUFORCE_UPMETHOD_R(crate::FieldReader<u8, u8>);
impl GEN1_CNTUFORCE_UPMETHOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        GEN1_CNTUFORCE_UPMETHOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GEN1_CNTUFORCE_UPMETHOD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GEN1_CNTUFORCE_UPMETHOD` writer - "]
pub struct GEN1_CNTUFORCE_UPMETHOD_W<'a> {
    w: &'a mut W,
}
impl<'a> GEN1_CNTUFORCE_UPMETHOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "Field `GEN1_A_CNTUFORCE_MODE` reader - "]
pub struct GEN1_A_CNTUFORCE_MODE_R(crate::FieldReader<u8, u8>);
impl GEN1_A_CNTUFORCE_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        GEN1_A_CNTUFORCE_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GEN1_A_CNTUFORCE_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GEN1_A_CNTUFORCE_MODE` writer - "]
pub struct GEN1_A_CNTUFORCE_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> GEN1_A_CNTUFORCE_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 6)) | ((value as u32 & 3) << 6);
        self.w
    }
}
#[doc = "Field `GEN1_B_CNTUFORCE_MODE` reader - "]
pub struct GEN1_B_CNTUFORCE_MODE_R(crate::FieldReader<u8, u8>);
impl GEN1_B_CNTUFORCE_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        GEN1_B_CNTUFORCE_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GEN1_B_CNTUFORCE_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GEN1_B_CNTUFORCE_MODE` writer - "]
pub struct GEN1_B_CNTUFORCE_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> GEN1_B_CNTUFORCE_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 8)) | ((value as u32 & 3) << 8);
        self.w
    }
}
#[doc = "Field `GEN1_A_NCIFORCE` reader - "]
pub struct GEN1_A_NCIFORCE_R(crate::FieldReader<bool, bool>);
impl GEN1_A_NCIFORCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GEN1_A_NCIFORCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GEN1_A_NCIFORCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GEN1_A_NCIFORCE` writer - "]
pub struct GEN1_A_NCIFORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> GEN1_A_NCIFORCE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 10)) | ((value as u32 & 1) << 10);
        self.w
    }
}
#[doc = "Field `GEN1_A_NCIFORCE_MODE` reader - "]
pub struct GEN1_A_NCIFORCE_MODE_R(crate::FieldReader<u8, u8>);
impl GEN1_A_NCIFORCE_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        GEN1_A_NCIFORCE_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GEN1_A_NCIFORCE_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GEN1_A_NCIFORCE_MODE` writer - "]
pub struct GEN1_A_NCIFORCE_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> GEN1_A_NCIFORCE_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 11)) | ((value as u32 & 3) << 11);
        self.w
    }
}
#[doc = "Field `GEN1_B_NCIFORCE` reader - "]
pub struct GEN1_B_NCIFORCE_R(crate::FieldReader<bool, bool>);
impl GEN1_B_NCIFORCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GEN1_B_NCIFORCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GEN1_B_NCIFORCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GEN1_B_NCIFORCE` writer - "]
pub struct GEN1_B_NCIFORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> GEN1_B_NCIFORCE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 13)) | ((value as u32 & 1) << 13);
        self.w
    }
}
#[doc = "Field `GEN1_B_NCIFORCE_MODE` reader - "]
pub struct GEN1_B_NCIFORCE_MODE_R(crate::FieldReader<u8, u8>);
impl GEN1_B_NCIFORCE_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        GEN1_B_NCIFORCE_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GEN1_B_NCIFORCE_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GEN1_B_NCIFORCE_MODE` writer - "]
pub struct GEN1_B_NCIFORCE_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> GEN1_B_NCIFORCE_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 14)) | ((value as u32 & 3) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn gen1_cntuforce_upmethod(&self) -> GEN1_CNTUFORCE_UPMETHOD_R {
        GEN1_CNTUFORCE_UPMETHOD_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn gen1_a_cntuforce_mode(&self) -> GEN1_A_CNTUFORCE_MODE_R {
        GEN1_A_CNTUFORCE_MODE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn gen1_b_cntuforce_mode(&self) -> GEN1_B_CNTUFORCE_MODE_R {
        GEN1_B_CNTUFORCE_MODE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn gen1_a_nciforce(&self) -> GEN1_A_NCIFORCE_R {
        GEN1_A_NCIFORCE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    pub fn gen1_a_nciforce_mode(&self) -> GEN1_A_NCIFORCE_MODE_R {
        GEN1_A_NCIFORCE_MODE_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn gen1_b_nciforce(&self) -> GEN1_B_NCIFORCE_R {
        GEN1_B_NCIFORCE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn gen1_b_nciforce_mode(&self) -> GEN1_B_NCIFORCE_MODE_R {
        GEN1_B_NCIFORCE_MODE_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn gen1_cntuforce_upmethod(&mut self) -> GEN1_CNTUFORCE_UPMETHOD_W {
        GEN1_CNTUFORCE_UPMETHOD_W { w: self }
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn gen1_a_cntuforce_mode(&mut self) -> GEN1_A_CNTUFORCE_MODE_W {
        GEN1_A_CNTUFORCE_MODE_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn gen1_b_cntuforce_mode(&mut self) -> GEN1_B_CNTUFORCE_MODE_W {
        GEN1_B_CNTUFORCE_MODE_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn gen1_a_nciforce(&mut self) -> GEN1_A_NCIFORCE_W {
        GEN1_A_NCIFORCE_W { w: self }
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    pub fn gen1_a_nciforce_mode(&mut self) -> GEN1_A_NCIFORCE_MODE_W {
        GEN1_A_NCIFORCE_MODE_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn gen1_b_nciforce(&mut self) -> GEN1_B_NCIFORCE_W {
        GEN1_B_NCIFORCE_W { w: self }
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn gen1_b_nciforce_mode(&mut self) -> GEN1_B_NCIFORCE_MODE_W {
        GEN1_B_NCIFORCE_MODE_W { w: self }
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
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gen1_force]
(index.html) module"]
pub struct GEN1_FORCE_SPEC;
impl crate::RegisterSpec for GEN1_FORCE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gen1_force::R]
(R) reader structure"]
impl crate::Readable for GEN1_FORCE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gen1_force::W]
(W) writer structure"]
impl crate::Writable for GEN1_FORCE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GEN1_FORCE to value 0x20"]
impl crate::Resettable for GEN1_FORCE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x20
    }
}