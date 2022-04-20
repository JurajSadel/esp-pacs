#[doc = "Register `PRO_IRAM0_1` reader"]
pub struct R(crate::R<PRO_IRAM0_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_IRAM0_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_IRAM0_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_IRAM0_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRO_IRAM0_1` writer"]
pub struct W(crate::W<PRO_IRAM0_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRO_IRAM0_1_SPEC>;
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
impl From<crate::W<PRO_IRAM0_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRO_IRAM0_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRO_IRAM0_SRAM_0_F` reader - Setting to 1 grants IBUS permission to fetch SRAM Block 0."]
pub struct PRO_IRAM0_SRAM_0_F_R(crate::FieldReader<bool, bool>);
impl PRO_IRAM0_SRAM_0_F_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_IRAM0_SRAM_0_F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_IRAM0_SRAM_0_F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_IRAM0_SRAM_0_F` writer - Setting to 1 grants IBUS permission to fetch SRAM Block 0."]
pub struct PRO_IRAM0_SRAM_0_F_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_IRAM0_SRAM_0_F_W<'a> {
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
#[doc = "Field `PRO_IRAM0_SRAM_0_R` reader - Setting to 1 grants IBUS permission to read SRAM Block 0."]
pub struct PRO_IRAM0_SRAM_0_R_R(crate::FieldReader<bool, bool>);
impl PRO_IRAM0_SRAM_0_R_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_IRAM0_SRAM_0_R_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_IRAM0_SRAM_0_R_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_IRAM0_SRAM_0_R` writer - Setting to 1 grants IBUS permission to read SRAM Block 0."]
pub struct PRO_IRAM0_SRAM_0_R_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_IRAM0_SRAM_0_R_W<'a> {
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
#[doc = "Field `PRO_IRAM0_SRAM_0_W` reader - Setting to 1 grants IBUS permission to write SRAM Block 0."]
pub struct PRO_IRAM0_SRAM_0_W_R(crate::FieldReader<bool, bool>);
impl PRO_IRAM0_SRAM_0_W_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_IRAM0_SRAM_0_W_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_IRAM0_SRAM_0_W_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_IRAM0_SRAM_0_W` writer - Setting to 1 grants IBUS permission to write SRAM Block 0."]
pub struct PRO_IRAM0_SRAM_0_W_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_IRAM0_SRAM_0_W_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 2)) | ((value as u32 & 1) << 2);
        self.w
    }
}
#[doc = "Field `PRO_IRAM0_SRAM_1_F` reader - Setting to 1 grants IBUS permission to fetch SRAM Block 1."]
pub struct PRO_IRAM0_SRAM_1_F_R(crate::FieldReader<bool, bool>);
impl PRO_IRAM0_SRAM_1_F_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_IRAM0_SRAM_1_F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_IRAM0_SRAM_1_F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_IRAM0_SRAM_1_F` writer - Setting to 1 grants IBUS permission to fetch SRAM Block 1."]
pub struct PRO_IRAM0_SRAM_1_F_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_IRAM0_SRAM_1_F_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 3)) | ((value as u32 & 1) << 3);
        self.w
    }
}
#[doc = "Field `PRO_IRAM0_SRAM_1_R` reader - Setting to 1 grants IBUS permission to read SRAM Block 1."]
pub struct PRO_IRAM0_SRAM_1_R_R(crate::FieldReader<bool, bool>);
impl PRO_IRAM0_SRAM_1_R_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_IRAM0_SRAM_1_R_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_IRAM0_SRAM_1_R_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_IRAM0_SRAM_1_R` writer - Setting to 1 grants IBUS permission to read SRAM Block 1."]
pub struct PRO_IRAM0_SRAM_1_R_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_IRAM0_SRAM_1_R_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u32 & 1) << 4);
        self.w
    }
}
#[doc = "Field `PRO_IRAM0_SRAM_1_W` reader - Setting to 1 grants IBUS permission to write SRAM Block 1."]
pub struct PRO_IRAM0_SRAM_1_W_R(crate::FieldReader<bool, bool>);
impl PRO_IRAM0_SRAM_1_W_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_IRAM0_SRAM_1_W_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_IRAM0_SRAM_1_W_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_IRAM0_SRAM_1_W` writer - Setting to 1 grants IBUS permission to write SRAM Block 1."]
pub struct PRO_IRAM0_SRAM_1_W_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_IRAM0_SRAM_1_W_W<'a> {
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
#[doc = "Field `PRO_IRAM0_SRAM_2_F` reader - Setting to 1 grants IBUS permission to fetch SRAM Block 2."]
pub struct PRO_IRAM0_SRAM_2_F_R(crate::FieldReader<bool, bool>);
impl PRO_IRAM0_SRAM_2_F_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_IRAM0_SRAM_2_F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_IRAM0_SRAM_2_F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_IRAM0_SRAM_2_F` writer - Setting to 1 grants IBUS permission to fetch SRAM Block 2."]
pub struct PRO_IRAM0_SRAM_2_F_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_IRAM0_SRAM_2_F_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 6)) | ((value as u32 & 1) << 6);
        self.w
    }
}
#[doc = "Field `PRO_IRAM0_SRAM_2_R` reader - Setting to 1 grants IBUS permission to read SRAM Block 2."]
pub struct PRO_IRAM0_SRAM_2_R_R(crate::FieldReader<bool, bool>);
impl PRO_IRAM0_SRAM_2_R_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_IRAM0_SRAM_2_R_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_IRAM0_SRAM_2_R_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_IRAM0_SRAM_2_R` writer - Setting to 1 grants IBUS permission to read SRAM Block 2."]
pub struct PRO_IRAM0_SRAM_2_R_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_IRAM0_SRAM_2_R_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 7)) | ((value as u32 & 1) << 7);
        self.w
    }
}
#[doc = "Field `PRO_IRAM0_SRAM_2_W` reader - Setting to 1 grants IBUS permission to write SRAM Block 2."]
pub struct PRO_IRAM0_SRAM_2_W_R(crate::FieldReader<bool, bool>);
impl PRO_IRAM0_SRAM_2_W_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_IRAM0_SRAM_2_W_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_IRAM0_SRAM_2_W_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_IRAM0_SRAM_2_W` writer - Setting to 1 grants IBUS permission to write SRAM Block 2."]
pub struct PRO_IRAM0_SRAM_2_W_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_IRAM0_SRAM_2_W_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 8)) | ((value as u32 & 1) << 8);
        self.w
    }
}
#[doc = "Field `PRO_IRAM0_SRAM_3_F` reader - Setting to 1 grants IBUS permission to fetch SRAM Block 3."]
pub struct PRO_IRAM0_SRAM_3_F_R(crate::FieldReader<bool, bool>);
impl PRO_IRAM0_SRAM_3_F_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_IRAM0_SRAM_3_F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_IRAM0_SRAM_3_F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_IRAM0_SRAM_3_F` writer - Setting to 1 grants IBUS permission to fetch SRAM Block 3."]
pub struct PRO_IRAM0_SRAM_3_F_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_IRAM0_SRAM_3_F_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 9)) | ((value as u32 & 1) << 9);
        self.w
    }
}
#[doc = "Field `PRO_IRAM0_SRAM_3_R` reader - Setting to 1 grants IBUS permission to read SRAM Block 3."]
pub struct PRO_IRAM0_SRAM_3_R_R(crate::FieldReader<bool, bool>);
impl PRO_IRAM0_SRAM_3_R_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_IRAM0_SRAM_3_R_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_IRAM0_SRAM_3_R_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_IRAM0_SRAM_3_R` writer - Setting to 1 grants IBUS permission to read SRAM Block 3."]
pub struct PRO_IRAM0_SRAM_3_R_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_IRAM0_SRAM_3_R_W<'a> {
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
#[doc = "Field `PRO_IRAM0_SRAM_3_W` reader - Setting to 1 grants IBUS permission to write SRAM Block 3."]
pub struct PRO_IRAM0_SRAM_3_W_R(crate::FieldReader<bool, bool>);
impl PRO_IRAM0_SRAM_3_W_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_IRAM0_SRAM_3_W_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_IRAM0_SRAM_3_W_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_IRAM0_SRAM_3_W` writer - Setting to 1 grants IBUS permission to write SRAM Block 3."]
pub struct PRO_IRAM0_SRAM_3_W_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_IRAM0_SRAM_3_W_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 11)) | ((value as u32 & 1) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Setting to 1 grants IBUS permission to fetch SRAM Block 0."]
    #[inline(always)]
    pub fn pro_iram0_sram_0_f(&self) -> PRO_IRAM0_SRAM_0_F_R {
        PRO_IRAM0_SRAM_0_F_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Setting to 1 grants IBUS permission to read SRAM Block 0."]
    #[inline(always)]
    pub fn pro_iram0_sram_0_r(&self) -> PRO_IRAM0_SRAM_0_R_R {
        PRO_IRAM0_SRAM_0_R_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Setting to 1 grants IBUS permission to write SRAM Block 0."]
    #[inline(always)]
    pub fn pro_iram0_sram_0_w(&self) -> PRO_IRAM0_SRAM_0_W_R {
        PRO_IRAM0_SRAM_0_W_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Setting to 1 grants IBUS permission to fetch SRAM Block 1."]
    #[inline(always)]
    pub fn pro_iram0_sram_1_f(&self) -> PRO_IRAM0_SRAM_1_F_R {
        PRO_IRAM0_SRAM_1_F_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Setting to 1 grants IBUS permission to read SRAM Block 1."]
    #[inline(always)]
    pub fn pro_iram0_sram_1_r(&self) -> PRO_IRAM0_SRAM_1_R_R {
        PRO_IRAM0_SRAM_1_R_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Setting to 1 grants IBUS permission to write SRAM Block 1."]
    #[inline(always)]
    pub fn pro_iram0_sram_1_w(&self) -> PRO_IRAM0_SRAM_1_W_R {
        PRO_IRAM0_SRAM_1_W_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Setting to 1 grants IBUS permission to fetch SRAM Block 2."]
    #[inline(always)]
    pub fn pro_iram0_sram_2_f(&self) -> PRO_IRAM0_SRAM_2_F_R {
        PRO_IRAM0_SRAM_2_F_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Setting to 1 grants IBUS permission to read SRAM Block 2."]
    #[inline(always)]
    pub fn pro_iram0_sram_2_r(&self) -> PRO_IRAM0_SRAM_2_R_R {
        PRO_IRAM0_SRAM_2_R_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Setting to 1 grants IBUS permission to write SRAM Block 2."]
    #[inline(always)]
    pub fn pro_iram0_sram_2_w(&self) -> PRO_IRAM0_SRAM_2_W_R {
        PRO_IRAM0_SRAM_2_W_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Setting to 1 grants IBUS permission to fetch SRAM Block 3."]
    #[inline(always)]
    pub fn pro_iram0_sram_3_f(&self) -> PRO_IRAM0_SRAM_3_F_R {
        PRO_IRAM0_SRAM_3_F_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Setting to 1 grants IBUS permission to read SRAM Block 3."]
    #[inline(always)]
    pub fn pro_iram0_sram_3_r(&self) -> PRO_IRAM0_SRAM_3_R_R {
        PRO_IRAM0_SRAM_3_R_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Setting to 1 grants IBUS permission to write SRAM Block 3."]
    #[inline(always)]
    pub fn pro_iram0_sram_3_w(&self) -> PRO_IRAM0_SRAM_3_W_R {
        PRO_IRAM0_SRAM_3_W_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Setting to 1 grants IBUS permission to fetch SRAM Block 0."]
    #[inline(always)]
    pub fn pro_iram0_sram_0_f(&mut self) -> PRO_IRAM0_SRAM_0_F_W {
        PRO_IRAM0_SRAM_0_F_W { w: self }
    }
    #[doc = "Bit 1 - Setting to 1 grants IBUS permission to read SRAM Block 0."]
    #[inline(always)]
    pub fn pro_iram0_sram_0_r(&mut self) -> PRO_IRAM0_SRAM_0_R_W {
        PRO_IRAM0_SRAM_0_R_W { w: self }
    }
    #[doc = "Bit 2 - Setting to 1 grants IBUS permission to write SRAM Block 0."]
    #[inline(always)]
    pub fn pro_iram0_sram_0_w(&mut self) -> PRO_IRAM0_SRAM_0_W_W {
        PRO_IRAM0_SRAM_0_W_W { w: self }
    }
    #[doc = "Bit 3 - Setting to 1 grants IBUS permission to fetch SRAM Block 1."]
    #[inline(always)]
    pub fn pro_iram0_sram_1_f(&mut self) -> PRO_IRAM0_SRAM_1_F_W {
        PRO_IRAM0_SRAM_1_F_W { w: self }
    }
    #[doc = "Bit 4 - Setting to 1 grants IBUS permission to read SRAM Block 1."]
    #[inline(always)]
    pub fn pro_iram0_sram_1_r(&mut self) -> PRO_IRAM0_SRAM_1_R_W {
        PRO_IRAM0_SRAM_1_R_W { w: self }
    }
    #[doc = "Bit 5 - Setting to 1 grants IBUS permission to write SRAM Block 1."]
    #[inline(always)]
    pub fn pro_iram0_sram_1_w(&mut self) -> PRO_IRAM0_SRAM_1_W_W {
        PRO_IRAM0_SRAM_1_W_W { w: self }
    }
    #[doc = "Bit 6 - Setting to 1 grants IBUS permission to fetch SRAM Block 2."]
    #[inline(always)]
    pub fn pro_iram0_sram_2_f(&mut self) -> PRO_IRAM0_SRAM_2_F_W {
        PRO_IRAM0_SRAM_2_F_W { w: self }
    }
    #[doc = "Bit 7 - Setting to 1 grants IBUS permission to read SRAM Block 2."]
    #[inline(always)]
    pub fn pro_iram0_sram_2_r(&mut self) -> PRO_IRAM0_SRAM_2_R_W {
        PRO_IRAM0_SRAM_2_R_W { w: self }
    }
    #[doc = "Bit 8 - Setting to 1 grants IBUS permission to write SRAM Block 2."]
    #[inline(always)]
    pub fn pro_iram0_sram_2_w(&mut self) -> PRO_IRAM0_SRAM_2_W_W {
        PRO_IRAM0_SRAM_2_W_W { w: self }
    }
    #[doc = "Bit 9 - Setting to 1 grants IBUS permission to fetch SRAM Block 3."]
    #[inline(always)]
    pub fn pro_iram0_sram_3_f(&mut self) -> PRO_IRAM0_SRAM_3_F_W {
        PRO_IRAM0_SRAM_3_F_W { w: self }
    }
    #[doc = "Bit 10 - Setting to 1 grants IBUS permission to read SRAM Block 3."]
    #[inline(always)]
    pub fn pro_iram0_sram_3_r(&mut self) -> PRO_IRAM0_SRAM_3_R_W {
        PRO_IRAM0_SRAM_3_R_W { w: self }
    }
    #[doc = "Bit 11 - Setting to 1 grants IBUS permission to write SRAM Block 3."]
    #[inline(always)]
    pub fn pro_iram0_sram_3_w(&mut self) -> PRO_IRAM0_SRAM_3_W_W {
        PRO_IRAM0_SRAM_3_W_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IBUS permission control register 1.\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_iram0_1]
(index.html) module"]
pub struct PRO_IRAM0_1_SPEC;
impl crate::RegisterSpec for PRO_IRAM0_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_iram0_1::R]
(R) reader structure"]
impl crate::Readable for PRO_IRAM0_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pro_iram0_1::W]
(W) writer structure"]
impl crate::Writable for PRO_IRAM0_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRO_IRAM0_1 to value 0x0fff"]
impl crate::Resettable for PRO_IRAM0_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0fff
    }
}