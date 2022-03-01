#[doc = "Register `RESET_STATE` reader"]
pub struct R(crate::R<RESET_STATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESET_STATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESET_STATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESET_STATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RESET_STATE` writer"]
pub struct W(crate::W<RESET_STATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RESET_STATE_SPEC>;
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
impl From<crate::W<RESET_STATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RESET_STATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESET_CAUSE_PROCPU` reader - reset cause of PRO CPU"]
pub struct RESET_CAUSE_PROCPU_R(crate::FieldReader<u8, u8>);
impl RESET_CAUSE_PROCPU_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RESET_CAUSE_PROCPU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESET_CAUSE_PROCPU_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESET_CAUSE_APPCPU` reader - reset cause of APP CPU"]
pub struct RESET_CAUSE_APPCPU_R(crate::FieldReader<u8, u8>);
impl RESET_CAUSE_APPCPU_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RESET_CAUSE_APPCPU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESET_CAUSE_APPCPU_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APPCPU_STAT_VECTOR_SEL` reader - APP CPU state vector sel"]
pub struct APPCPU_STAT_VECTOR_SEL_R(crate::FieldReader<bool, bool>);
impl APPCPU_STAT_VECTOR_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APPCPU_STAT_VECTOR_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APPCPU_STAT_VECTOR_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APPCPU_STAT_VECTOR_SEL` writer - APP CPU state vector sel"]
pub struct APPCPU_STAT_VECTOR_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> APPCPU_STAT_VECTOR_SEL_W<'a> {
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
#[doc = "Field `PROCPU_STAT_VECTOR_SEL` reader - PRO CPU state vector sel"]
pub struct PROCPU_STAT_VECTOR_SEL_R(crate::FieldReader<bool, bool>);
impl PROCPU_STAT_VECTOR_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PROCPU_STAT_VECTOR_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROCPU_STAT_VECTOR_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROCPU_STAT_VECTOR_SEL` writer - PRO CPU state vector sel"]
pub struct PROCPU_STAT_VECTOR_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PROCPU_STAT_VECTOR_SEL_W<'a> {
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
#[doc = "Field `RESET_FLAG_PROCPU` reader - PRO CPU reset_flag"]
pub struct RESET_FLAG_PROCPU_R(crate::FieldReader<bool, bool>);
impl RESET_FLAG_PROCPU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RESET_FLAG_PROCPU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESET_FLAG_PROCPU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESET_FLAG_APPCPU` reader - APP CPU reset flag"]
pub struct RESET_FLAG_APPCPU_R(crate::FieldReader<bool, bool>);
impl RESET_FLAG_APPCPU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RESET_FLAG_APPCPU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESET_FLAG_APPCPU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESET_FLAG_PROCPU_CLR` writer - clear PRO CPU reset_flag"]
pub struct RESET_FLAG_PROCPU_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_FLAG_PROCPU_CLR_W<'a> {
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
#[doc = "Field `RESET_FLAG_APPCPU_CLR` writer - clear APP CPU reset flag"]
pub struct RESET_FLAG_APPCPU_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_FLAG_APPCPU_CLR_W<'a> {
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
#[doc = "Field `APPCPU_OCD_HALT_ON_RESET` reader - APPCPU OcdHaltOnReset"]
pub struct APPCPU_OCD_HALT_ON_RESET_R(crate::FieldReader<bool, bool>);
impl APPCPU_OCD_HALT_ON_RESET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APPCPU_OCD_HALT_ON_RESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APPCPU_OCD_HALT_ON_RESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APPCPU_OCD_HALT_ON_RESET` writer - APPCPU OcdHaltOnReset"]
pub struct APPCPU_OCD_HALT_ON_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> APPCPU_OCD_HALT_ON_RESET_W<'a> {
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
#[doc = "Field `PROCPU_OCD_HALT_ON_RESET` reader - PROCPU OcdHaltOnReset"]
pub struct PROCPU_OCD_HALT_ON_RESET_R(crate::FieldReader<bool, bool>);
impl PROCPU_OCD_HALT_ON_RESET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PROCPU_OCD_HALT_ON_RESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROCPU_OCD_HALT_ON_RESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROCPU_OCD_HALT_ON_RESET` writer - PROCPU OcdHaltOnReset"]
pub struct PROCPU_OCD_HALT_ON_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> PROCPU_OCD_HALT_ON_RESET_W<'a> {
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
#[doc = "Field `RESET_FLAG_JTAG_PROCPU` reader - jtag reset flag"]
pub struct RESET_FLAG_JTAG_PROCPU_R(crate::FieldReader<bool, bool>);
impl RESET_FLAG_JTAG_PROCPU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RESET_FLAG_JTAG_PROCPU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESET_FLAG_JTAG_PROCPU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESET_FLAG_JTAG_APPCPU` reader - jtag reset flag"]
pub struct RESET_FLAG_JTAG_APPCPU_R(crate::FieldReader<bool, bool>);
impl RESET_FLAG_JTAG_APPCPU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RESET_FLAG_JTAG_APPCPU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESET_FLAG_JTAG_APPCPU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESET_FLAG_JTAG_PROCPU_CLR` writer - clear jtag reset flag"]
pub struct RESET_FLAG_JTAG_PROCPU_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_FLAG_JTAG_PROCPU_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `RESET_FLAG_JTAG_APPCPU_CLR` writer - clear jtag reset flag"]
pub struct RESET_FLAG_JTAG_APPCPU_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_FLAG_JTAG_APPCPU_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `RTC_APP_DRESET_MASK` reader - bypass cpu1 dreset"]
pub struct RTC_APP_DRESET_MASK_R(crate::FieldReader<bool, bool>);
impl RTC_APP_DRESET_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_APP_DRESET_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_APP_DRESET_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_APP_DRESET_MASK` writer - bypass cpu1 dreset"]
pub struct RTC_APP_DRESET_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_APP_DRESET_MASK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `RTC_PRO_DRESET_MASK` reader - bypass cpu0 dreset"]
pub struct RTC_PRO_DRESET_MASK_R(crate::FieldReader<bool, bool>);
impl RTC_PRO_DRESET_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_PRO_DRESET_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_PRO_DRESET_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_PRO_DRESET_MASK` writer - bypass cpu0 dreset"]
pub struct RTC_PRO_DRESET_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_PRO_DRESET_MASK_W<'a> {
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
impl R {
    #[doc = "Bits 0:5 - reset cause of PRO CPU"]
    #[inline(always)]
    pub fn reset_cause_procpu(&self) -> RESET_CAUSE_PROCPU_R {
        RESET_CAUSE_PROCPU_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - reset cause of APP CPU"]
    #[inline(always)]
    pub fn reset_cause_appcpu(&self) -> RESET_CAUSE_APPCPU_R {
        RESET_CAUSE_APPCPU_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bit 12 - APP CPU state vector sel"]
    #[inline(always)]
    pub fn appcpu_stat_vector_sel(&self) -> APPCPU_STAT_VECTOR_SEL_R {
        APPCPU_STAT_VECTOR_SEL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - PRO CPU state vector sel"]
    #[inline(always)]
    pub fn procpu_stat_vector_sel(&self) -> PROCPU_STAT_VECTOR_SEL_R {
        PROCPU_STAT_VECTOR_SEL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - PRO CPU reset_flag"]
    #[inline(always)]
    pub fn reset_flag_procpu(&self) -> RESET_FLAG_PROCPU_R {
        RESET_FLAG_PROCPU_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - APP CPU reset flag"]
    #[inline(always)]
    pub fn reset_flag_appcpu(&self) -> RESET_FLAG_APPCPU_R {
        RESET_FLAG_APPCPU_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 18 - APPCPU OcdHaltOnReset"]
    #[inline(always)]
    pub fn appcpu_ocd_halt_on_reset(&self) -> APPCPU_OCD_HALT_ON_RESET_R {
        APPCPU_OCD_HALT_ON_RESET_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - PROCPU OcdHaltOnReset"]
    #[inline(always)]
    pub fn procpu_ocd_halt_on_reset(&self) -> PROCPU_OCD_HALT_ON_RESET_R {
        PROCPU_OCD_HALT_ON_RESET_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - jtag reset flag"]
    #[inline(always)]
    pub fn reset_flag_jtag_procpu(&self) -> RESET_FLAG_JTAG_PROCPU_R {
        RESET_FLAG_JTAG_PROCPU_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - jtag reset flag"]
    #[inline(always)]
    pub fn reset_flag_jtag_appcpu(&self) -> RESET_FLAG_JTAG_APPCPU_R {
        RESET_FLAG_JTAG_APPCPU_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 24 - bypass cpu1 dreset"]
    #[inline(always)]
    pub fn rtc_app_dreset_mask(&self) -> RTC_APP_DRESET_MASK_R {
        RTC_APP_DRESET_MASK_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - bypass cpu0 dreset"]
    #[inline(always)]
    pub fn rtc_pro_dreset_mask(&self) -> RTC_PRO_DRESET_MASK_R {
        RTC_PRO_DRESET_MASK_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - APP CPU state vector sel"]
    #[inline(always)]
    pub fn appcpu_stat_vector_sel(&mut self) -> APPCPU_STAT_VECTOR_SEL_W {
        APPCPU_STAT_VECTOR_SEL_W { w: self }
    }
    #[doc = "Bit 13 - PRO CPU state vector sel"]
    #[inline(always)]
    pub fn procpu_stat_vector_sel(&mut self) -> PROCPU_STAT_VECTOR_SEL_W {
        PROCPU_STAT_VECTOR_SEL_W { w: self }
    }
    #[doc = "Bit 16 - clear PRO CPU reset_flag"]
    #[inline(always)]
    pub fn reset_flag_procpu_clr(&mut self) -> RESET_FLAG_PROCPU_CLR_W {
        RESET_FLAG_PROCPU_CLR_W { w: self }
    }
    #[doc = "Bit 17 - clear APP CPU reset flag"]
    #[inline(always)]
    pub fn reset_flag_appcpu_clr(&mut self) -> RESET_FLAG_APPCPU_CLR_W {
        RESET_FLAG_APPCPU_CLR_W { w: self }
    }
    #[doc = "Bit 18 - APPCPU OcdHaltOnReset"]
    #[inline(always)]
    pub fn appcpu_ocd_halt_on_reset(&mut self) -> APPCPU_OCD_HALT_ON_RESET_W {
        APPCPU_OCD_HALT_ON_RESET_W { w: self }
    }
    #[doc = "Bit 19 - PROCPU OcdHaltOnReset"]
    #[inline(always)]
    pub fn procpu_ocd_halt_on_reset(&mut self) -> PROCPU_OCD_HALT_ON_RESET_W {
        PROCPU_OCD_HALT_ON_RESET_W { w: self }
    }
    #[doc = "Bit 22 - clear jtag reset flag"]
    #[inline(always)]
    pub fn reset_flag_jtag_procpu_clr(&mut self) -> RESET_FLAG_JTAG_PROCPU_CLR_W {
        RESET_FLAG_JTAG_PROCPU_CLR_W { w: self }
    }
    #[doc = "Bit 23 - clear jtag reset flag"]
    #[inline(always)]
    pub fn reset_flag_jtag_appcpu_clr(&mut self) -> RESET_FLAG_JTAG_APPCPU_CLR_W {
        RESET_FLAG_JTAG_APPCPU_CLR_W { w: self }
    }
    #[doc = "Bit 24 - bypass cpu1 dreset"]
    #[inline(always)]
    pub fn rtc_app_dreset_mask(&mut self) -> RTC_APP_DRESET_MASK_W {
        RTC_APP_DRESET_MASK_W { w: self }
    }
    #[doc = "Bit 25 - bypass cpu0 dreset"]
    #[inline(always)]
    pub fn rtc_pro_dreset_mask(&mut self) -> RTC_PRO_DRESET_MASK_W {
        RTC_PRO_DRESET_MASK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "get reset state\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reset_state]
(index.html) module"]
pub struct RESET_STATE_SPEC;
impl crate::RegisterSpec for RESET_STATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reset_state::R]
(R) reader structure"]
impl crate::Readable for RESET_STATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reset_state::W]
(W) writer structure"]
impl crate::Writable for RESET_STATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RESET_STATE to value 0x3000"]
impl crate::Resettable for RESET_STATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3000
    }
}