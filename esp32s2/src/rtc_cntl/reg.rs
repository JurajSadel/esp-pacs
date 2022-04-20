#[doc = "Register `REG` reader"]
pub struct R(crate::R<REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REG` writer"]
pub struct W(crate::W<REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REG_SPEC>;
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
impl From<crate::W<REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIG_REG_DBIAS_SLP` reader - Configures the regulation factor for the digital system voltage regulator when the CPU is in sleep status."]
pub struct DIG_REG_DBIAS_SLP_R(crate::FieldReader<u8, u8>);
impl DIG_REG_DBIAS_SLP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIG_REG_DBIAS_SLP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIG_REG_DBIAS_SLP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIG_REG_DBIAS_SLP` writer - Configures the regulation factor for the digital system voltage regulator when the CPU is in sleep status."]
pub struct DIG_REG_DBIAS_SLP_W<'a> {
    w: &'a mut W,
}
impl<'a> DIG_REG_DBIAS_SLP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 8)) | ((value as u32 & 7) << 8);
        self.w
    }
}
#[doc = "Field `DIG_REG_DBIAS_WAK` reader - Configures the regulation factor for the digital system voltage regulator when the CPU is in active status."]
pub struct DIG_REG_DBIAS_WAK_R(crate::FieldReader<u8, u8>);
impl DIG_REG_DBIAS_WAK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIG_REG_DBIAS_WAK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIG_REG_DBIAS_WAK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIG_REG_DBIAS_WAK` writer - Configures the regulation factor for the digital system voltage regulator when the CPU is in active status."]
pub struct DIG_REG_DBIAS_WAK_W<'a> {
    w: &'a mut W,
}
impl<'a> DIG_REG_DBIAS_WAK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 11)) | ((value as u32 & 7) << 11);
        self.w
    }
}
#[doc = "Field `SCK_DCAP` reader - Configures the frequency of the RTC clocks."]
pub struct SCK_DCAP_R(crate::FieldReader<u8, u8>);
impl SCK_DCAP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SCK_DCAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCK_DCAP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCK_DCAP` writer - Configures the frequency of the RTC clocks."]
pub struct SCK_DCAP_W<'a> {
    w: &'a mut W,
}
impl<'a> SCK_DCAP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 14)) | ((value as u32 & 0xff) << 14);
        self.w
    }
}
#[doc = "Field `DBIAS_SLP` reader - Configures the regulation factor for the low-power voltage regulator when the CPU is in sleep status."]
pub struct DBIAS_SLP_R(crate::FieldReader<u8, u8>);
impl DBIAS_SLP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DBIAS_SLP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBIAS_SLP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBIAS_SLP` writer - Configures the regulation factor for the low-power voltage regulator when the CPU is in sleep status."]
pub struct DBIAS_SLP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBIAS_SLP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 22)) | ((value as u32 & 7) << 22);
        self.w
    }
}
#[doc = "Field `DBIAS_WAK` reader - Configures the regulation factor for the low-power voltage regulator when the CPU is in active status."]
pub struct DBIAS_WAK_R(crate::FieldReader<u8, u8>);
impl DBIAS_WAK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DBIAS_WAK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBIAS_WAK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBIAS_WAK` writer - Configures the regulation factor for the low-power voltage regulator when the CPU is in active status."]
pub struct DBIAS_WAK_W<'a> {
    w: &'a mut W,
}
impl<'a> DBIAS_WAK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 25)) | ((value as u32 & 7) << 25);
        self.w
    }
}
#[doc = "Field `DBOOST_FORCE_PD` reader - RTC_DBOOST force power down"]
pub struct DBOOST_FORCE_PD_R(crate::FieldReader<bool, bool>);
impl DBOOST_FORCE_PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DBOOST_FORCE_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBOOST_FORCE_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBOOST_FORCE_PD` writer - RTC_DBOOST force power down"]
pub struct DBOOST_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> DBOOST_FORCE_PD_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 28)) | ((value as u32 & 1) << 28);
        self.w
    }
}
#[doc = "Field `DBOOST_FORCE_PU` reader - RTC_DBOOST force power up"]
pub struct DBOOST_FORCE_PU_R(crate::FieldReader<bool, bool>);
impl DBOOST_FORCE_PU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DBOOST_FORCE_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBOOST_FORCE_PU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBOOST_FORCE_PU` writer - RTC_DBOOST force power up"]
pub struct DBOOST_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> DBOOST_FORCE_PU_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 29)) | ((value as u32 & 1) << 29);
        self.w
    }
}
#[doc = "Field `REGULATOR_FORCE_PD` reader - Set this bit to FPD the RTC_REG, which means decreasing its voltage to 0.8 V or lower."]
pub struct REGULATOR_FORCE_PD_R(crate::FieldReader<bool, bool>);
impl REGULATOR_FORCE_PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGULATOR_FORCE_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REGULATOR_FORCE_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGULATOR_FORCE_PD` writer - Set this bit to FPD the RTC_REG, which means decreasing its voltage to 0.8 V or lower."]
pub struct REGULATOR_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> REGULATOR_FORCE_PD_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 30)) | ((value as u32 & 1) << 30);
        self.w
    }
}
#[doc = "Field `REGULATOR_FORCE_PU` reader - Set this bit to FPU the RTC_REG."]
pub struct REGULATOR_FORCE_PU_R(crate::FieldReader<bool, bool>);
impl REGULATOR_FORCE_PU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGULATOR_FORCE_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REGULATOR_FORCE_PU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGULATOR_FORCE_PU` writer - Set this bit to FPU the RTC_REG."]
pub struct REGULATOR_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> REGULATOR_FORCE_PU_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 31)) | ((value as u32 & 1) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:10 - Configures the regulation factor for the digital system voltage regulator when the CPU is in sleep status."]
    #[inline(always)]
    pub fn dig_reg_dbias_slp(&self) -> DIG_REG_DBIAS_SLP_R {
        DIG_REG_DBIAS_SLP_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:13 - Configures the regulation factor for the digital system voltage regulator when the CPU is in active status."]
    #[inline(always)]
    pub fn dig_reg_dbias_wak(&self) -> DIG_REG_DBIAS_WAK_R {
        DIG_REG_DBIAS_WAK_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bits 14:21 - Configures the frequency of the RTC clocks."]
    #[inline(always)]
    pub fn sck_dcap(&self) -> SCK_DCAP_R {
        SCK_DCAP_R::new(((self.bits >> 14) & 0xff) as u8)
    }
    #[doc = "Bits 22:24 - Configures the regulation factor for the low-power voltage regulator when the CPU is in sleep status."]
    #[inline(always)]
    pub fn dbias_slp(&self) -> DBIAS_SLP_R {
        DBIAS_SLP_R::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bits 25:27 - Configures the regulation factor for the low-power voltage regulator when the CPU is in active status."]
    #[inline(always)]
    pub fn dbias_wak(&self) -> DBIAS_WAK_R {
        DBIAS_WAK_R::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bit 28 - RTC_DBOOST force power down"]
    #[inline(always)]
    pub fn dboost_force_pd(&self) -> DBOOST_FORCE_PD_R {
        DBOOST_FORCE_PD_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - RTC_DBOOST force power up"]
    #[inline(always)]
    pub fn dboost_force_pu(&self) -> DBOOST_FORCE_PU_R {
        DBOOST_FORCE_PU_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Set this bit to FPD the RTC_REG, which means decreasing its voltage to 0.8 V or lower."]
    #[inline(always)]
    pub fn regulator_force_pd(&self) -> REGULATOR_FORCE_PD_R {
        REGULATOR_FORCE_PD_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Set this bit to FPU the RTC_REG."]
    #[inline(always)]
    pub fn regulator_force_pu(&self) -> REGULATOR_FORCE_PU_R {
        REGULATOR_FORCE_PU_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 8:10 - Configures the regulation factor for the digital system voltage regulator when the CPU is in sleep status."]
    #[inline(always)]
    pub fn dig_reg_dbias_slp(&mut self) -> DIG_REG_DBIAS_SLP_W {
        DIG_REG_DBIAS_SLP_W { w: self }
    }
    #[doc = "Bits 11:13 - Configures the regulation factor for the digital system voltage regulator when the CPU is in active status."]
    #[inline(always)]
    pub fn dig_reg_dbias_wak(&mut self) -> DIG_REG_DBIAS_WAK_W {
        DIG_REG_DBIAS_WAK_W { w: self }
    }
    #[doc = "Bits 14:21 - Configures the frequency of the RTC clocks."]
    #[inline(always)]
    pub fn sck_dcap(&mut self) -> SCK_DCAP_W {
        SCK_DCAP_W { w: self }
    }
    #[doc = "Bits 22:24 - Configures the regulation factor for the low-power voltage regulator when the CPU is in sleep status."]
    #[inline(always)]
    pub fn dbias_slp(&mut self) -> DBIAS_SLP_W {
        DBIAS_SLP_W { w: self }
    }
    #[doc = "Bits 25:27 - Configures the regulation factor for the low-power voltage regulator when the CPU is in active status."]
    #[inline(always)]
    pub fn dbias_wak(&mut self) -> DBIAS_WAK_W {
        DBIAS_WAK_W { w: self }
    }
    #[doc = "Bit 28 - RTC_DBOOST force power down"]
    #[inline(always)]
    pub fn dboost_force_pd(&mut self) -> DBOOST_FORCE_PD_W {
        DBOOST_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 29 - RTC_DBOOST force power up"]
    #[inline(always)]
    pub fn dboost_force_pu(&mut self) -> DBOOST_FORCE_PU_W {
        DBOOST_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 30 - Set this bit to FPD the RTC_REG, which means decreasing its voltage to 0.8 V or lower."]
    #[inline(always)]
    pub fn regulator_force_pd(&mut self) -> REGULATOR_FORCE_PD_W {
        REGULATOR_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 31 - Set this bit to FPU the RTC_REG."]
    #[inline(always)]
    pub fn regulator_force_pu(&mut self) -> REGULATOR_FORCE_PU_W {
        REGULATOR_FORCE_PU_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC/DIG regulator configuration register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg]
(index.html) module"]
pub struct REG_SPEC;
impl crate::RegisterSpec for REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg::R]
(R) reader structure"]
impl crate::Readable for REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reg::W]
(W) writer structure"]
impl crate::Writable for REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REG to value 0xa900_2400"]
impl crate::Resettable for REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xa900_2400
    }
}