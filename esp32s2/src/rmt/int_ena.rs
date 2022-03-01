#[doc = "Register `INT_ENA` reader"]
pub struct R(crate::R<INT_ENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_ENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_ENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_ENA` writer"]
pub struct W(crate::W<INT_ENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_ENA_SPEC>;
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
impl From<crate::W<INT_ENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_ENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0_TX_END_INT_ENA` reader - The interrupt enabled bit for CH0_TX_END_INT."]
pub struct CH0_TX_END_INT_ENA_R(crate::FieldReader<bool, bool>);
impl CH0_TX_END_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH0_TX_END_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH0_TX_END_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH0_TX_END_INT_ENA` writer - The interrupt enabled bit for CH0_TX_END_INT."]
pub struct CH0_TX_END_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0_TX_END_INT_ENA_W<'a> {
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
#[doc = "Field `CH0_RX_END_INT_ENA` reader - The interrupt enabled bit for CH0_RX_END_INT."]
pub struct CH0_RX_END_INT_ENA_R(crate::FieldReader<bool, bool>);
impl CH0_RX_END_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH0_RX_END_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH0_RX_END_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH0_RX_END_INT_ENA` writer - The interrupt enabled bit for CH0_RX_END_INT."]
pub struct CH0_RX_END_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0_RX_END_INT_ENA_W<'a> {
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
#[doc = "Field `CH0_ERR_INT_ENA` reader - The interrupt enabled bit for CH0_ERR_INT."]
pub struct CH0_ERR_INT_ENA_R(crate::FieldReader<bool, bool>);
impl CH0_ERR_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH0_ERR_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH0_ERR_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH0_ERR_INT_ENA` writer - The interrupt enabled bit for CH0_ERR_INT."]
pub struct CH0_ERR_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0_ERR_INT_ENA_W<'a> {
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
#[doc = "Field `CH1_TX_END_INT_ENA` reader - The interrupt enabled bit for CH1_TX_END_INT."]
pub struct CH1_TX_END_INT_ENA_R(crate::FieldReader<bool, bool>);
impl CH1_TX_END_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH1_TX_END_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH1_TX_END_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH1_TX_END_INT_ENA` writer - The interrupt enabled bit for CH1_TX_END_INT."]
pub struct CH1_TX_END_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1_TX_END_INT_ENA_W<'a> {
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
#[doc = "Field `CH1_RX_END_INT_ENA` reader - The interrupt enabled bit for CH1_RX_END_INT."]
pub struct CH1_RX_END_INT_ENA_R(crate::FieldReader<bool, bool>);
impl CH1_RX_END_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH1_RX_END_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH1_RX_END_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH1_RX_END_INT_ENA` writer - The interrupt enabled bit for CH1_RX_END_INT."]
pub struct CH1_RX_END_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1_RX_END_INT_ENA_W<'a> {
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
#[doc = "Field `CH1_ERR_INT_ENA` reader - The interrupt enabled bit for CH1_ERR_INT."]
pub struct CH1_ERR_INT_ENA_R(crate::FieldReader<bool, bool>);
impl CH1_ERR_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH1_ERR_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH1_ERR_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH1_ERR_INT_ENA` writer - The interrupt enabled bit for CH1_ERR_INT."]
pub struct CH1_ERR_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1_ERR_INT_ENA_W<'a> {
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
#[doc = "Field `CH2_TX_END_INT_ENA` reader - The interrupt enabled bit for CH2_TX_END_INT."]
pub struct CH2_TX_END_INT_ENA_R(crate::FieldReader<bool, bool>);
impl CH2_TX_END_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH2_TX_END_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH2_TX_END_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH2_TX_END_INT_ENA` writer - The interrupt enabled bit for CH2_TX_END_INT."]
pub struct CH2_TX_END_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2_TX_END_INT_ENA_W<'a> {
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
#[doc = "Field `CH2_RX_END_INT_ENA` reader - The interrupt enabled bit for CH2_RX_END_INT."]
pub struct CH2_RX_END_INT_ENA_R(crate::FieldReader<bool, bool>);
impl CH2_RX_END_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH2_RX_END_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH2_RX_END_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH2_RX_END_INT_ENA` writer - The interrupt enabled bit for CH2_RX_END_INT."]
pub struct CH2_RX_END_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2_RX_END_INT_ENA_W<'a> {
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
#[doc = "Field `CH2_ERR_INT_ENA` reader - The interrupt enabled bit for CH2_ERR_INT."]
pub struct CH2_ERR_INT_ENA_R(crate::FieldReader<bool, bool>);
impl CH2_ERR_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH2_ERR_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH2_ERR_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH2_ERR_INT_ENA` writer - The interrupt enabled bit for CH2_ERR_INT."]
pub struct CH2_ERR_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2_ERR_INT_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `CH3_TX_END_INT_ENA` reader - The interrupt enabled bit for CH3_TX_END_INT."]
pub struct CH3_TX_END_INT_ENA_R(crate::FieldReader<bool, bool>);
impl CH3_TX_END_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH3_TX_END_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH3_TX_END_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH3_TX_END_INT_ENA` writer - The interrupt enabled bit for CH3_TX_END_INT."]
pub struct CH3_TX_END_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3_TX_END_INT_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `CH3_RX_END_INT_ENA` reader - The interrupt enabled bit for CH3_RX_END_INT."]
pub struct CH3_RX_END_INT_ENA_R(crate::FieldReader<bool, bool>);
impl CH3_RX_END_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH3_RX_END_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH3_RX_END_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH3_RX_END_INT_ENA` writer - The interrupt enabled bit for CH3_RX_END_INT."]
pub struct CH3_RX_END_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3_RX_END_INT_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `CH3_ERR_INT_ENA` reader - The interrupt enabled bit for CH3_ERR_INT."]
pub struct CH3_ERR_INT_ENA_R(crate::FieldReader<bool, bool>);
impl CH3_ERR_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH3_ERR_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH3_ERR_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH3_ERR_INT_ENA` writer - The interrupt enabled bit for CH3_ERR_INT."]
pub struct CH3_ERR_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3_ERR_INT_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `CH0_TX_THR_EVENT_INT_ENA` reader - The interrupt enabled bit for CH0_TX_THR_EVENT_INT."]
pub struct CH0_TX_THR_EVENT_INT_ENA_R(crate::FieldReader<bool, bool>);
impl CH0_TX_THR_EVENT_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH0_TX_THR_EVENT_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH0_TX_THR_EVENT_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH0_TX_THR_EVENT_INT_ENA` writer - The interrupt enabled bit for CH0_TX_THR_EVENT_INT."]
pub struct CH0_TX_THR_EVENT_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0_TX_THR_EVENT_INT_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `CH1_TX_THR_EVENT_INT_ENA` reader - The interrupt enabled bit for CH1_TX_THR_EVENT_INT."]
pub struct CH1_TX_THR_EVENT_INT_ENA_R(crate::FieldReader<bool, bool>);
impl CH1_TX_THR_EVENT_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH1_TX_THR_EVENT_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH1_TX_THR_EVENT_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH1_TX_THR_EVENT_INT_ENA` writer - The interrupt enabled bit for CH1_TX_THR_EVENT_INT."]
pub struct CH1_TX_THR_EVENT_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1_TX_THR_EVENT_INT_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `CH2_TX_THR_EVENT_INT_ENA` reader - The interrupt enabled bit for CH2_TX_THR_EVENT_INT."]
pub struct CH2_TX_THR_EVENT_INT_ENA_R(crate::FieldReader<bool, bool>);
impl CH2_TX_THR_EVENT_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH2_TX_THR_EVENT_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH2_TX_THR_EVENT_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH2_TX_THR_EVENT_INT_ENA` writer - The interrupt enabled bit for CH2_TX_THR_EVENT_INT."]
pub struct CH2_TX_THR_EVENT_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2_TX_THR_EVENT_INT_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `CH3_TX_THR_EVENT_INT_ENA` reader - The interrupt enabled bit for CH3_TX_THR_EVENT_INT."]
pub struct CH3_TX_THR_EVENT_INT_ENA_R(crate::FieldReader<bool, bool>);
impl CH3_TX_THR_EVENT_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH3_TX_THR_EVENT_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH3_TX_THR_EVENT_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH3_TX_THR_EVENT_INT_ENA` writer - The interrupt enabled bit for CH3_TX_THR_EVENT_INT."]
pub struct CH3_TX_THR_EVENT_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3_TX_THR_EVENT_INT_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `CH0_TX_LOOP_INT_ENA` reader - The interrupt enabled bit for CH0_TX_LOOP_INT."]
pub struct CH0_TX_LOOP_INT_ENA_R(crate::FieldReader<bool, bool>);
impl CH0_TX_LOOP_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH0_TX_LOOP_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH0_TX_LOOP_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH0_TX_LOOP_INT_ENA` writer - The interrupt enabled bit for CH0_TX_LOOP_INT."]
pub struct CH0_TX_LOOP_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0_TX_LOOP_INT_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `CH1_TX_LOOP_INT_ENA` reader - The interrupt enabled bit for CH1_TX_LOOP_INT."]
pub struct CH1_TX_LOOP_INT_ENA_R(crate::FieldReader<bool, bool>);
impl CH1_TX_LOOP_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH1_TX_LOOP_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH1_TX_LOOP_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH1_TX_LOOP_INT_ENA` writer - The interrupt enabled bit for CH1_TX_LOOP_INT."]
pub struct CH1_TX_LOOP_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1_TX_LOOP_INT_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `CH2_TX_LOOP_INT_ENA` reader - The interrupt enabled bit for CH2_TX_LOOP_INT."]
pub struct CH2_TX_LOOP_INT_ENA_R(crate::FieldReader<bool, bool>);
impl CH2_TX_LOOP_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH2_TX_LOOP_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH2_TX_LOOP_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH2_TX_LOOP_INT_ENA` writer - The interrupt enabled bit for CH2_TX_LOOP_INT."]
pub struct CH2_TX_LOOP_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2_TX_LOOP_INT_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `CH3_TX_LOOP_INT_ENA` reader - The interrupt enabled bit for CH3_TX_LOOP_INT."]
pub struct CH3_TX_LOOP_INT_ENA_R(crate::FieldReader<bool, bool>);
impl CH3_TX_LOOP_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH3_TX_LOOP_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH3_TX_LOOP_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH3_TX_LOOP_INT_ENA` writer - The interrupt enabled bit for CH3_TX_LOOP_INT."]
pub struct CH3_TX_LOOP_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3_TX_LOOP_INT_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - The interrupt enabled bit for CH0_TX_END_INT."]
    #[inline(always)]
    pub fn ch0_tx_end_int_ena(&self) -> CH0_TX_END_INT_ENA_R {
        CH0_TX_END_INT_ENA_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - The interrupt enabled bit for CH0_RX_END_INT."]
    #[inline(always)]
    pub fn ch0_rx_end_int_ena(&self) -> CH0_RX_END_INT_ENA_R {
        CH0_RX_END_INT_ENA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - The interrupt enabled bit for CH0_ERR_INT."]
    #[inline(always)]
    pub fn ch0_err_int_ena(&self) -> CH0_ERR_INT_ENA_R {
        CH0_ERR_INT_ENA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - The interrupt enabled bit for CH1_TX_END_INT."]
    #[inline(always)]
    pub fn ch1_tx_end_int_ena(&self) -> CH1_TX_END_INT_ENA_R {
        CH1_TX_END_INT_ENA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - The interrupt enabled bit for CH1_RX_END_INT."]
    #[inline(always)]
    pub fn ch1_rx_end_int_ena(&self) -> CH1_RX_END_INT_ENA_R {
        CH1_RX_END_INT_ENA_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - The interrupt enabled bit for CH1_ERR_INT."]
    #[inline(always)]
    pub fn ch1_err_int_ena(&self) -> CH1_ERR_INT_ENA_R {
        CH1_ERR_INT_ENA_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - The interrupt enabled bit for CH2_TX_END_INT."]
    #[inline(always)]
    pub fn ch2_tx_end_int_ena(&self) -> CH2_TX_END_INT_ENA_R {
        CH2_TX_END_INT_ENA_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - The interrupt enabled bit for CH2_RX_END_INT."]
    #[inline(always)]
    pub fn ch2_rx_end_int_ena(&self) -> CH2_RX_END_INT_ENA_R {
        CH2_RX_END_INT_ENA_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - The interrupt enabled bit for CH2_ERR_INT."]
    #[inline(always)]
    pub fn ch2_err_int_ena(&self) -> CH2_ERR_INT_ENA_R {
        CH2_ERR_INT_ENA_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - The interrupt enabled bit for CH3_TX_END_INT."]
    #[inline(always)]
    pub fn ch3_tx_end_int_ena(&self) -> CH3_TX_END_INT_ENA_R {
        CH3_TX_END_INT_ENA_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - The interrupt enabled bit for CH3_RX_END_INT."]
    #[inline(always)]
    pub fn ch3_rx_end_int_ena(&self) -> CH3_RX_END_INT_ENA_R {
        CH3_RX_END_INT_ENA_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - The interrupt enabled bit for CH3_ERR_INT."]
    #[inline(always)]
    pub fn ch3_err_int_ena(&self) -> CH3_ERR_INT_ENA_R {
        CH3_ERR_INT_ENA_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - The interrupt enabled bit for CH0_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch0_tx_thr_event_int_ena(&self) -> CH0_TX_THR_EVENT_INT_ENA_R {
        CH0_TX_THR_EVENT_INT_ENA_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - The interrupt enabled bit for CH1_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch1_tx_thr_event_int_ena(&self) -> CH1_TX_THR_EVENT_INT_ENA_R {
        CH1_TX_THR_EVENT_INT_ENA_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - The interrupt enabled bit for CH2_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch2_tx_thr_event_int_ena(&self) -> CH2_TX_THR_EVENT_INT_ENA_R {
        CH2_TX_THR_EVENT_INT_ENA_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - The interrupt enabled bit for CH3_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch3_tx_thr_event_int_ena(&self) -> CH3_TX_THR_EVENT_INT_ENA_R {
        CH3_TX_THR_EVENT_INT_ENA_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - The interrupt enabled bit for CH0_TX_LOOP_INT."]
    #[inline(always)]
    pub fn ch0_tx_loop_int_ena(&self) -> CH0_TX_LOOP_INT_ENA_R {
        CH0_TX_LOOP_INT_ENA_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - The interrupt enabled bit for CH1_TX_LOOP_INT."]
    #[inline(always)]
    pub fn ch1_tx_loop_int_ena(&self) -> CH1_TX_LOOP_INT_ENA_R {
        CH1_TX_LOOP_INT_ENA_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - The interrupt enabled bit for CH2_TX_LOOP_INT."]
    #[inline(always)]
    pub fn ch2_tx_loop_int_ena(&self) -> CH2_TX_LOOP_INT_ENA_R {
        CH2_TX_LOOP_INT_ENA_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - The interrupt enabled bit for CH3_TX_LOOP_INT."]
    #[inline(always)]
    pub fn ch3_tx_loop_int_ena(&self) -> CH3_TX_LOOP_INT_ENA_R {
        CH3_TX_LOOP_INT_ENA_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The interrupt enabled bit for CH0_TX_END_INT."]
    #[inline(always)]
    pub fn ch0_tx_end_int_ena(&mut self) -> CH0_TX_END_INT_ENA_W {
        CH0_TX_END_INT_ENA_W { w: self }
    }
    #[doc = "Bit 1 - The interrupt enabled bit for CH0_RX_END_INT."]
    #[inline(always)]
    pub fn ch0_rx_end_int_ena(&mut self) -> CH0_RX_END_INT_ENA_W {
        CH0_RX_END_INT_ENA_W { w: self }
    }
    #[doc = "Bit 2 - The interrupt enabled bit for CH0_ERR_INT."]
    #[inline(always)]
    pub fn ch0_err_int_ena(&mut self) -> CH0_ERR_INT_ENA_W {
        CH0_ERR_INT_ENA_W { w: self }
    }
    #[doc = "Bit 3 - The interrupt enabled bit for CH1_TX_END_INT."]
    #[inline(always)]
    pub fn ch1_tx_end_int_ena(&mut self) -> CH1_TX_END_INT_ENA_W {
        CH1_TX_END_INT_ENA_W { w: self }
    }
    #[doc = "Bit 4 - The interrupt enabled bit for CH1_RX_END_INT."]
    #[inline(always)]
    pub fn ch1_rx_end_int_ena(&mut self) -> CH1_RX_END_INT_ENA_W {
        CH1_RX_END_INT_ENA_W { w: self }
    }
    #[doc = "Bit 5 - The interrupt enabled bit for CH1_ERR_INT."]
    #[inline(always)]
    pub fn ch1_err_int_ena(&mut self) -> CH1_ERR_INT_ENA_W {
        CH1_ERR_INT_ENA_W { w: self }
    }
    #[doc = "Bit 6 - The interrupt enabled bit for CH2_TX_END_INT."]
    #[inline(always)]
    pub fn ch2_tx_end_int_ena(&mut self) -> CH2_TX_END_INT_ENA_W {
        CH2_TX_END_INT_ENA_W { w: self }
    }
    #[doc = "Bit 7 - The interrupt enabled bit for CH2_RX_END_INT."]
    #[inline(always)]
    pub fn ch2_rx_end_int_ena(&mut self) -> CH2_RX_END_INT_ENA_W {
        CH2_RX_END_INT_ENA_W { w: self }
    }
    #[doc = "Bit 8 - The interrupt enabled bit for CH2_ERR_INT."]
    #[inline(always)]
    pub fn ch2_err_int_ena(&mut self) -> CH2_ERR_INT_ENA_W {
        CH2_ERR_INT_ENA_W { w: self }
    }
    #[doc = "Bit 9 - The interrupt enabled bit for CH3_TX_END_INT."]
    #[inline(always)]
    pub fn ch3_tx_end_int_ena(&mut self) -> CH3_TX_END_INT_ENA_W {
        CH3_TX_END_INT_ENA_W { w: self }
    }
    #[doc = "Bit 10 - The interrupt enabled bit for CH3_RX_END_INT."]
    #[inline(always)]
    pub fn ch3_rx_end_int_ena(&mut self) -> CH3_RX_END_INT_ENA_W {
        CH3_RX_END_INT_ENA_W { w: self }
    }
    #[doc = "Bit 11 - The interrupt enabled bit for CH3_ERR_INT."]
    #[inline(always)]
    pub fn ch3_err_int_ena(&mut self) -> CH3_ERR_INT_ENA_W {
        CH3_ERR_INT_ENA_W { w: self }
    }
    #[doc = "Bit 12 - The interrupt enabled bit for CH0_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch0_tx_thr_event_int_ena(&mut self) -> CH0_TX_THR_EVENT_INT_ENA_W {
        CH0_TX_THR_EVENT_INT_ENA_W { w: self }
    }
    #[doc = "Bit 13 - The interrupt enabled bit for CH1_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch1_tx_thr_event_int_ena(&mut self) -> CH1_TX_THR_EVENT_INT_ENA_W {
        CH1_TX_THR_EVENT_INT_ENA_W { w: self }
    }
    #[doc = "Bit 14 - The interrupt enabled bit for CH2_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch2_tx_thr_event_int_ena(&mut self) -> CH2_TX_THR_EVENT_INT_ENA_W {
        CH2_TX_THR_EVENT_INT_ENA_W { w: self }
    }
    #[doc = "Bit 15 - The interrupt enabled bit for CH3_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch3_tx_thr_event_int_ena(&mut self) -> CH3_TX_THR_EVENT_INT_ENA_W {
        CH3_TX_THR_EVENT_INT_ENA_W { w: self }
    }
    #[doc = "Bit 16 - The interrupt enabled bit for CH0_TX_LOOP_INT."]
    #[inline(always)]
    pub fn ch0_tx_loop_int_ena(&mut self) -> CH0_TX_LOOP_INT_ENA_W {
        CH0_TX_LOOP_INT_ENA_W { w: self }
    }
    #[doc = "Bit 17 - The interrupt enabled bit for CH1_TX_LOOP_INT."]
    #[inline(always)]
    pub fn ch1_tx_loop_int_ena(&mut self) -> CH1_TX_LOOP_INT_ENA_W {
        CH1_TX_LOOP_INT_ENA_W { w: self }
    }
    #[doc = "Bit 18 - The interrupt enabled bit for CH2_TX_LOOP_INT."]
    #[inline(always)]
    pub fn ch2_tx_loop_int_ena(&mut self) -> CH2_TX_LOOP_INT_ENA_W {
        CH2_TX_LOOP_INT_ENA_W { w: self }
    }
    #[doc = "Bit 19 - The interrupt enabled bit for CH3_TX_LOOP_INT."]
    #[inline(always)]
    pub fn ch3_tx_loop_int_ena(&mut self) -> CH3_TX_LOOP_INT_ENA_W {
        CH3_TX_LOOP_INT_ENA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt enable bits\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_ena]
(index.html) module"]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_ena::R]
(R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_ena::W]
(W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}