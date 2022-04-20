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
#[doc = "Field `RX_INT_ENA` reader - Set this bit to 1 to enable receive interrupt."]
pub struct RX_INT_ENA_R(crate::FieldReader<bool, bool>);
impl RX_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_INT_ENA` writer - Set this bit to 1 to enable receive interrupt."]
pub struct RX_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_INT_ENA_W<'a> {
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
#[doc = "Field `TX_INT_ENA` reader - Set this bit to 1 to enable transmit interrupt."]
pub struct TX_INT_ENA_R(crate::FieldReader<bool, bool>);
impl TX_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_INT_ENA` writer - Set this bit to 1 to enable transmit interrupt."]
pub struct TX_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_INT_ENA_W<'a> {
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
#[doc = "Field `ERR_WARN_INT_ENA` reader - Set this bit to 1 to enable error warning interrupt."]
pub struct ERR_WARN_INT_ENA_R(crate::FieldReader<bool, bool>);
impl ERR_WARN_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERR_WARN_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERR_WARN_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERR_WARN_INT_ENA` writer - Set this bit to 1 to enable error warning interrupt."]
pub struct ERR_WARN_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR_WARN_INT_ENA_W<'a> {
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
#[doc = "Field `OVERRUN_INT_ENA` reader - Set this bit to 1 to enable data overrun interrupt."]
pub struct OVERRUN_INT_ENA_R(crate::FieldReader<bool, bool>);
impl OVERRUN_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVERRUN_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVERRUN_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVERRUN_INT_ENA` writer - Set this bit to 1 to enable data overrun interrupt."]
pub struct OVERRUN_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERRUN_INT_ENA_W<'a> {
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
#[doc = "Field `ERR_PASSIVE_INT_ENA` reader - Set this bit to 1 to enable error passive interrupt."]
pub struct ERR_PASSIVE_INT_ENA_R(crate::FieldReader<bool, bool>);
impl ERR_PASSIVE_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERR_PASSIVE_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERR_PASSIVE_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERR_PASSIVE_INT_ENA` writer - Set this bit to 1 to enable error passive interrupt."]
pub struct ERR_PASSIVE_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR_PASSIVE_INT_ENA_W<'a> {
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
#[doc = "Field `ARB_LOST_INT_ENA` reader - Set this bit to 1 to enable arbitration lost interrupt."]
pub struct ARB_LOST_INT_ENA_R(crate::FieldReader<bool, bool>);
impl ARB_LOST_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ARB_LOST_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ARB_LOST_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARB_LOST_INT_ENA` writer - Set this bit to 1 to enable arbitration lost interrupt."]
pub struct ARB_LOST_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> ARB_LOST_INT_ENA_W<'a> {
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
#[doc = "Field `BUS_ERR_INT_ENA` reader - Set this bit to 1 to enable error interrupt."]
pub struct BUS_ERR_INT_ENA_R(crate::FieldReader<bool, bool>);
impl BUS_ERR_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BUS_ERR_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUS_ERR_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUS_ERR_INT_ENA` writer - Set this bit to 1 to enable error interrupt."]
pub struct BUS_ERR_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> BUS_ERR_INT_ENA_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Set this bit to 1 to enable receive interrupt."]
    #[inline(always)]
    pub fn rx_int_ena(&self) -> RX_INT_ENA_R {
        RX_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to 1 to enable transmit interrupt."]
    #[inline(always)]
    pub fn tx_int_ena(&self) -> TX_INT_ENA_R {
        TX_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to 1 to enable error warning interrupt."]
    #[inline(always)]
    pub fn err_warn_int_ena(&self) -> ERR_WARN_INT_ENA_R {
        ERR_WARN_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to 1 to enable data overrun interrupt."]
    #[inline(always)]
    pub fn overrun_int_ena(&self) -> OVERRUN_INT_ENA_R {
        OVERRUN_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Set this bit to 1 to enable error passive interrupt."]
    #[inline(always)]
    pub fn err_passive_int_ena(&self) -> ERR_PASSIVE_INT_ENA_R {
        ERR_PASSIVE_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Set this bit to 1 to enable arbitration lost interrupt."]
    #[inline(always)]
    pub fn arb_lost_int_ena(&self) -> ARB_LOST_INT_ENA_R {
        ARB_LOST_INT_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Set this bit to 1 to enable error interrupt."]
    #[inline(always)]
    pub fn bus_err_int_ena(&self) -> BUS_ERR_INT_ENA_R {
        BUS_ERR_INT_ENA_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to 1 to enable receive interrupt."]
    #[inline(always)]
    pub fn rx_int_ena(&mut self) -> RX_INT_ENA_W {
        RX_INT_ENA_W { w: self }
    }
    #[doc = "Bit 1 - Set this bit to 1 to enable transmit interrupt."]
    #[inline(always)]
    pub fn tx_int_ena(&mut self) -> TX_INT_ENA_W {
        TX_INT_ENA_W { w: self }
    }
    #[doc = "Bit 2 - Set this bit to 1 to enable error warning interrupt."]
    #[inline(always)]
    pub fn err_warn_int_ena(&mut self) -> ERR_WARN_INT_ENA_W {
        ERR_WARN_INT_ENA_W { w: self }
    }
    #[doc = "Bit 3 - Set this bit to 1 to enable data overrun interrupt."]
    #[inline(always)]
    pub fn overrun_int_ena(&mut self) -> OVERRUN_INT_ENA_W {
        OVERRUN_INT_ENA_W { w: self }
    }
    #[doc = "Bit 5 - Set this bit to 1 to enable error passive interrupt."]
    #[inline(always)]
    pub fn err_passive_int_ena(&mut self) -> ERR_PASSIVE_INT_ENA_W {
        ERR_PASSIVE_INT_ENA_W { w: self }
    }
    #[doc = "Bit 6 - Set this bit to 1 to enable arbitration lost interrupt."]
    #[inline(always)]
    pub fn arb_lost_int_ena(&mut self) -> ARB_LOST_INT_ENA_W {
        ARB_LOST_INT_ENA_W { w: self }
    }
    #[doc = "Bit 7 - Set this bit to 1 to enable error interrupt."]
    #[inline(always)]
    pub fn bus_err_int_ena(&mut self) -> BUS_ERR_INT_ENA_W {
        BUS_ERR_INT_ENA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`]
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