#[doc = "Register `PRO_DRAM0_1` reader"]
pub struct R(crate::R<PRO_DRAM0_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_DRAM0_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_DRAM0_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_DRAM0_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRO_DRAM0_1` writer"]
pub struct W(crate::W<PRO_DRAM0_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRO_DRAM0_1_SPEC>;
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
impl From<crate::W<PRO_DRAM0_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRO_DRAM0_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRO_DRAM0_SRAM_0_R` reader - Setting to 1 grants DBUS0 permission to read SRAM Block 0."]
pub struct PRO_DRAM0_SRAM_0_R_R(crate::FieldReader<bool, bool>);
impl PRO_DRAM0_SRAM_0_R_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_DRAM0_SRAM_0_R_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_DRAM0_SRAM_0_R_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_DRAM0_SRAM_0_R` writer - Setting to 1 grants DBUS0 permission to read SRAM Block 0."]
pub struct PRO_DRAM0_SRAM_0_R_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_DRAM0_SRAM_0_R_W<'a> {
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
#[doc = "Field `PRO_DRAM0_SRAM_0_W` reader - Setting to 1 grants DBUS0 permission to write SRAM Block 0."]
pub struct PRO_DRAM0_SRAM_0_W_R(crate::FieldReader<bool, bool>);
impl PRO_DRAM0_SRAM_0_W_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_DRAM0_SRAM_0_W_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_DRAM0_SRAM_0_W_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_DRAM0_SRAM_0_W` writer - Setting to 1 grants DBUS0 permission to write SRAM Block 0."]
pub struct PRO_DRAM0_SRAM_0_W_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_DRAM0_SRAM_0_W_W<'a> {
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
#[doc = "Field `PRO_DRAM0_SRAM_1_R` reader - Setting to 1 grants DBUS0 permission to read SRAM Block 1."]
pub struct PRO_DRAM0_SRAM_1_R_R(crate::FieldReader<bool, bool>);
impl PRO_DRAM0_SRAM_1_R_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_DRAM0_SRAM_1_R_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_DRAM0_SRAM_1_R_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_DRAM0_SRAM_1_R` writer - Setting to 1 grants DBUS0 permission to read SRAM Block 1."]
pub struct PRO_DRAM0_SRAM_1_R_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_DRAM0_SRAM_1_R_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `PRO_DRAM0_SRAM_1_W` reader - Setting to 1 grants DBUS0 permission to write SRAM Block 1."]
pub struct PRO_DRAM0_SRAM_1_W_R(crate::FieldReader<bool, bool>);
impl PRO_DRAM0_SRAM_1_W_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_DRAM0_SRAM_1_W_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_DRAM0_SRAM_1_W_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_DRAM0_SRAM_1_W` writer - Setting to 1 grants DBUS0 permission to write SRAM Block 1."]
pub struct PRO_DRAM0_SRAM_1_W_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_DRAM0_SRAM_1_W_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `PRO_DRAM0_SRAM_2_R` reader - Setting to 1 grants DBUS0 permission to read SRAM Block 2."]
pub struct PRO_DRAM0_SRAM_2_R_R(crate::FieldReader<bool, bool>);
impl PRO_DRAM0_SRAM_2_R_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_DRAM0_SRAM_2_R_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_DRAM0_SRAM_2_R_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_DRAM0_SRAM_2_R` writer - Setting to 1 grants DBUS0 permission to read SRAM Block 2."]
pub struct PRO_DRAM0_SRAM_2_R_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_DRAM0_SRAM_2_R_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `PRO_DRAM0_SRAM_2_W` reader - Setting to 1 grants DBUS0 permission to write SRAM Block 2."]
pub struct PRO_DRAM0_SRAM_2_W_R(crate::FieldReader<bool, bool>);
impl PRO_DRAM0_SRAM_2_W_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_DRAM0_SRAM_2_W_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_DRAM0_SRAM_2_W_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_DRAM0_SRAM_2_W` writer - Setting to 1 grants DBUS0 permission to write SRAM Block 2."]
pub struct PRO_DRAM0_SRAM_2_W_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_DRAM0_SRAM_2_W_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `PRO_DRAM0_SRAM_3_R` reader - Setting to 1 grants DBUS0 permission to read SRAM Block 3."]
pub struct PRO_DRAM0_SRAM_3_R_R(crate::FieldReader<bool, bool>);
impl PRO_DRAM0_SRAM_3_R_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_DRAM0_SRAM_3_R_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_DRAM0_SRAM_3_R_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_DRAM0_SRAM_3_R` writer - Setting to 1 grants DBUS0 permission to read SRAM Block 3."]
pub struct PRO_DRAM0_SRAM_3_R_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_DRAM0_SRAM_3_R_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `PRO_DRAM0_SRAM_3_W` reader - Setting to 1 grants DBUS0 permission to write SRAM Block 3."]
pub struct PRO_DRAM0_SRAM_3_W_R(crate::FieldReader<bool, bool>);
impl PRO_DRAM0_SRAM_3_W_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_DRAM0_SRAM_3_W_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_DRAM0_SRAM_3_W_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_DRAM0_SRAM_3_W` writer - Setting to 1 grants DBUS0 permission to write SRAM Block 3."]
pub struct PRO_DRAM0_SRAM_3_W_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_DRAM0_SRAM_3_W_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `PRO_DRAM0_SRAM_4_SPLTADDR` reader - Configure the split address of SRAM Block 4-21 for DBUS0 access."]
pub struct PRO_DRAM0_SRAM_4_SPLTADDR_R(crate::FieldReader<u32, u32>);
impl PRO_DRAM0_SRAM_4_SPLTADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        PRO_DRAM0_SRAM_4_SPLTADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_DRAM0_SRAM_4_SPLTADDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_DRAM0_SRAM_4_SPLTADDR` writer - Configure the split address of SRAM Block 4-21 for DBUS0 access."]
pub struct PRO_DRAM0_SRAM_4_SPLTADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_DRAM0_SRAM_4_SPLTADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0001_ffff << 8)) | ((value as u32 & 0x0001_ffff) << 8);
        self.w
    }
}
#[doc = "Field `PRO_DRAM0_SRAM_4_L_R` reader - Setting to 1 grants DBUS0 permission to read SRAM Block 4-21 low address region."]
pub struct PRO_DRAM0_SRAM_4_L_R_R(crate::FieldReader<bool, bool>);
impl PRO_DRAM0_SRAM_4_L_R_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_DRAM0_SRAM_4_L_R_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_DRAM0_SRAM_4_L_R_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_DRAM0_SRAM_4_L_R` writer - Setting to 1 grants DBUS0 permission to read SRAM Block 4-21 low address region."]
pub struct PRO_DRAM0_SRAM_4_L_R_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_DRAM0_SRAM_4_L_R_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `PRO_DRAM0_SRAM_4_L_W` reader - Setting to 1 grants DBUS0 permission to write SRAM Block 4-21 low address region."]
pub struct PRO_DRAM0_SRAM_4_L_W_R(crate::FieldReader<bool, bool>);
impl PRO_DRAM0_SRAM_4_L_W_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_DRAM0_SRAM_4_L_W_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_DRAM0_SRAM_4_L_W_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_DRAM0_SRAM_4_L_W` writer - Setting to 1 grants DBUS0 permission to write SRAM Block 4-21 low address region."]
pub struct PRO_DRAM0_SRAM_4_L_W_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_DRAM0_SRAM_4_L_W_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `PRO_DRAM0_SRAM_4_H_R` reader - Setting to 1 grants DBUS0 permission to read SRAM Block 4-21 high address region."]
pub struct PRO_DRAM0_SRAM_4_H_R_R(crate::FieldReader<bool, bool>);
impl PRO_DRAM0_SRAM_4_H_R_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_DRAM0_SRAM_4_H_R_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_DRAM0_SRAM_4_H_R_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_DRAM0_SRAM_4_H_R` writer - Setting to 1 grants DBUS0 permission to read SRAM Block 4-21 high address region."]
pub struct PRO_DRAM0_SRAM_4_H_R_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_DRAM0_SRAM_4_H_R_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `PRO_DRAM0_SRAM_4_H_W` reader - Setting to 1 grants DBUS0 permission to write SRAM Block 4-21 high address region."]
pub struct PRO_DRAM0_SRAM_4_H_W_R(crate::FieldReader<bool, bool>);
impl PRO_DRAM0_SRAM_4_H_W_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_DRAM0_SRAM_4_H_W_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_DRAM0_SRAM_4_H_W_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_DRAM0_SRAM_4_H_W` writer - Setting to 1 grants DBUS0 permission to write SRAM Block 4-21 high address region."]
pub struct PRO_DRAM0_SRAM_4_H_W_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_DRAM0_SRAM_4_H_W_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Setting to 1 grants DBUS0 permission to read SRAM Block 0."]
    #[inline(always)]
    pub fn pro_dram0_sram_0_r(&self) -> PRO_DRAM0_SRAM_0_R_R {
        PRO_DRAM0_SRAM_0_R_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Setting to 1 grants DBUS0 permission to write SRAM Block 0."]
    #[inline(always)]
    pub fn pro_dram0_sram_0_w(&self) -> PRO_DRAM0_SRAM_0_W_R {
        PRO_DRAM0_SRAM_0_W_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Setting to 1 grants DBUS0 permission to read SRAM Block 1."]
    #[inline(always)]
    pub fn pro_dram0_sram_1_r(&self) -> PRO_DRAM0_SRAM_1_R_R {
        PRO_DRAM0_SRAM_1_R_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Setting to 1 grants DBUS0 permission to write SRAM Block 1."]
    #[inline(always)]
    pub fn pro_dram0_sram_1_w(&self) -> PRO_DRAM0_SRAM_1_W_R {
        PRO_DRAM0_SRAM_1_W_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Setting to 1 grants DBUS0 permission to read SRAM Block 2."]
    #[inline(always)]
    pub fn pro_dram0_sram_2_r(&self) -> PRO_DRAM0_SRAM_2_R_R {
        PRO_DRAM0_SRAM_2_R_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Setting to 1 grants DBUS0 permission to write SRAM Block 2."]
    #[inline(always)]
    pub fn pro_dram0_sram_2_w(&self) -> PRO_DRAM0_SRAM_2_W_R {
        PRO_DRAM0_SRAM_2_W_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Setting to 1 grants DBUS0 permission to read SRAM Block 3."]
    #[inline(always)]
    pub fn pro_dram0_sram_3_r(&self) -> PRO_DRAM0_SRAM_3_R_R {
        PRO_DRAM0_SRAM_3_R_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Setting to 1 grants DBUS0 permission to write SRAM Block 3."]
    #[inline(always)]
    pub fn pro_dram0_sram_3_w(&self) -> PRO_DRAM0_SRAM_3_W_R {
        PRO_DRAM0_SRAM_3_W_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:24 - Configure the split address of SRAM Block 4-21 for DBUS0 access."]
    #[inline(always)]
    pub fn pro_dram0_sram_4_spltaddr(&self) -> PRO_DRAM0_SRAM_4_SPLTADDR_R {
        PRO_DRAM0_SRAM_4_SPLTADDR_R::new(((self.bits >> 8) & 0x0001_ffff) as u32)
    }
    #[doc = "Bit 25 - Setting to 1 grants DBUS0 permission to read SRAM Block 4-21 low address region."]
    #[inline(always)]
    pub fn pro_dram0_sram_4_l_r(&self) -> PRO_DRAM0_SRAM_4_L_R_R {
        PRO_DRAM0_SRAM_4_L_R_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Setting to 1 grants DBUS0 permission to write SRAM Block 4-21 low address region."]
    #[inline(always)]
    pub fn pro_dram0_sram_4_l_w(&self) -> PRO_DRAM0_SRAM_4_L_W_R {
        PRO_DRAM0_SRAM_4_L_W_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Setting to 1 grants DBUS0 permission to read SRAM Block 4-21 high address region."]
    #[inline(always)]
    pub fn pro_dram0_sram_4_h_r(&self) -> PRO_DRAM0_SRAM_4_H_R_R {
        PRO_DRAM0_SRAM_4_H_R_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Setting to 1 grants DBUS0 permission to write SRAM Block 4-21 high address region."]
    #[inline(always)]
    pub fn pro_dram0_sram_4_h_w(&self) -> PRO_DRAM0_SRAM_4_H_W_R {
        PRO_DRAM0_SRAM_4_H_W_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Setting to 1 grants DBUS0 permission to read SRAM Block 0."]
    #[inline(always)]
    pub fn pro_dram0_sram_0_r(&mut self) -> PRO_DRAM0_SRAM_0_R_W {
        PRO_DRAM0_SRAM_0_R_W { w: self }
    }
    #[doc = "Bit 1 - Setting to 1 grants DBUS0 permission to write SRAM Block 0."]
    #[inline(always)]
    pub fn pro_dram0_sram_0_w(&mut self) -> PRO_DRAM0_SRAM_0_W_W {
        PRO_DRAM0_SRAM_0_W_W { w: self }
    }
    #[doc = "Bit 2 - Setting to 1 grants DBUS0 permission to read SRAM Block 1."]
    #[inline(always)]
    pub fn pro_dram0_sram_1_r(&mut self) -> PRO_DRAM0_SRAM_1_R_W {
        PRO_DRAM0_SRAM_1_R_W { w: self }
    }
    #[doc = "Bit 3 - Setting to 1 grants DBUS0 permission to write SRAM Block 1."]
    #[inline(always)]
    pub fn pro_dram0_sram_1_w(&mut self) -> PRO_DRAM0_SRAM_1_W_W {
        PRO_DRAM0_SRAM_1_W_W { w: self }
    }
    #[doc = "Bit 4 - Setting to 1 grants DBUS0 permission to read SRAM Block 2."]
    #[inline(always)]
    pub fn pro_dram0_sram_2_r(&mut self) -> PRO_DRAM0_SRAM_2_R_W {
        PRO_DRAM0_SRAM_2_R_W { w: self }
    }
    #[doc = "Bit 5 - Setting to 1 grants DBUS0 permission to write SRAM Block 2."]
    #[inline(always)]
    pub fn pro_dram0_sram_2_w(&mut self) -> PRO_DRAM0_SRAM_2_W_W {
        PRO_DRAM0_SRAM_2_W_W { w: self }
    }
    #[doc = "Bit 6 - Setting to 1 grants DBUS0 permission to read SRAM Block 3."]
    #[inline(always)]
    pub fn pro_dram0_sram_3_r(&mut self) -> PRO_DRAM0_SRAM_3_R_W {
        PRO_DRAM0_SRAM_3_R_W { w: self }
    }
    #[doc = "Bit 7 - Setting to 1 grants DBUS0 permission to write SRAM Block 3."]
    #[inline(always)]
    pub fn pro_dram0_sram_3_w(&mut self) -> PRO_DRAM0_SRAM_3_W_W {
        PRO_DRAM0_SRAM_3_W_W { w: self }
    }
    #[doc = "Bits 8:24 - Configure the split address of SRAM Block 4-21 for DBUS0 access."]
    #[inline(always)]
    pub fn pro_dram0_sram_4_spltaddr(&mut self) -> PRO_DRAM0_SRAM_4_SPLTADDR_W {
        PRO_DRAM0_SRAM_4_SPLTADDR_W { w: self }
    }
    #[doc = "Bit 25 - Setting to 1 grants DBUS0 permission to read SRAM Block 4-21 low address region."]
    #[inline(always)]
    pub fn pro_dram0_sram_4_l_r(&mut self) -> PRO_DRAM0_SRAM_4_L_R_W {
        PRO_DRAM0_SRAM_4_L_R_W { w: self }
    }
    #[doc = "Bit 26 - Setting to 1 grants DBUS0 permission to write SRAM Block 4-21 low address region."]
    #[inline(always)]
    pub fn pro_dram0_sram_4_l_w(&mut self) -> PRO_DRAM0_SRAM_4_L_W_W {
        PRO_DRAM0_SRAM_4_L_W_W { w: self }
    }
    #[doc = "Bit 27 - Setting to 1 grants DBUS0 permission to read SRAM Block 4-21 high address region."]
    #[inline(always)]
    pub fn pro_dram0_sram_4_h_r(&mut self) -> PRO_DRAM0_SRAM_4_H_R_W {
        PRO_DRAM0_SRAM_4_H_R_W { w: self }
    }
    #[doc = "Bit 28 - Setting to 1 grants DBUS0 permission to write SRAM Block 4-21 high address region."]
    #[inline(always)]
    pub fn pro_dram0_sram_4_h_w(&mut self) -> PRO_DRAM0_SRAM_4_H_W_W {
        PRO_DRAM0_SRAM_4_H_W_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DBUS permission control register 1.\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_dram0_1]
(index.html) module"]
pub struct PRO_DRAM0_1_SPEC;
impl crate::RegisterSpec for PRO_DRAM0_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_dram0_1::R]
(R) reader structure"]
impl crate::Readable for PRO_DRAM0_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pro_dram0_1::W]
(W) writer structure"]
impl crate::Writable for PRO_DRAM0_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRO_DRAM0_1 to value 0x1e00_00ff"]
impl crate::Resettable for PRO_DRAM0_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1e00_00ff
    }
}