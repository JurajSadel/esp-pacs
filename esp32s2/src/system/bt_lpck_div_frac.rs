#[doc = "Register `BT_LPCK_DIV_FRAC` reader"]
pub struct R(crate::R<BT_LPCK_DIV_FRAC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BT_LPCK_DIV_FRAC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BT_LPCK_DIV_FRAC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BT_LPCK_DIV_FRAC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BT_LPCK_DIV_FRAC` writer"]
pub struct W(crate::W<BT_LPCK_DIV_FRAC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BT_LPCK_DIV_FRAC_SPEC>;
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
impl From<crate::W<BT_LPCK_DIV_FRAC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BT_LPCK_DIV_FRAC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPCLK_SEL_RTC_SLOW` reader - Set this bit to select RTC slow clock as the low power clock."]
pub struct LPCLK_SEL_RTC_SLOW_R(crate::FieldReader<bool, bool>);
impl LPCLK_SEL_RTC_SLOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LPCLK_SEL_RTC_SLOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPCLK_SEL_RTC_SLOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPCLK_SEL_RTC_SLOW` writer - Set this bit to select RTC slow clock as the low power clock."]
pub struct LPCLK_SEL_RTC_SLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> LPCLK_SEL_RTC_SLOW_W<'a> {
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
#[doc = "Field `LPCLK_SEL_8M` reader - Set this bit to select 8m clock as the low power clock."]
pub struct LPCLK_SEL_8M_R(crate::FieldReader<bool, bool>);
impl LPCLK_SEL_8M_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LPCLK_SEL_8M_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPCLK_SEL_8M_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPCLK_SEL_8M` writer - Set this bit to select 8m clock as the low power clock."]
pub struct LPCLK_SEL_8M_W<'a> {
    w: &'a mut W,
}
impl<'a> LPCLK_SEL_8M_W<'a> {
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
#[doc = "Field `LPCLK_SEL_XTAL` reader - Set this bit to select xtal clock as the low power clock."]
pub struct LPCLK_SEL_XTAL_R(crate::FieldReader<bool, bool>);
impl LPCLK_SEL_XTAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LPCLK_SEL_XTAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPCLK_SEL_XTAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPCLK_SEL_XTAL` writer - Set this bit to select xtal clock as the low power clock."]
pub struct LPCLK_SEL_XTAL_W<'a> {
    w: &'a mut W,
}
impl<'a> LPCLK_SEL_XTAL_W<'a> {
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
#[doc = "Field `LPCLK_SEL_XTAL32K` reader - Set this bit to select xtal32k clock as the low power clock."]
pub struct LPCLK_SEL_XTAL32K_R(crate::FieldReader<bool, bool>);
impl LPCLK_SEL_XTAL32K_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LPCLK_SEL_XTAL32K_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPCLK_SEL_XTAL32K_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPCLK_SEL_XTAL32K` writer - Set this bit to select xtal32k clock as the low power clock."]
pub struct LPCLK_SEL_XTAL32K_W<'a> {
    w: &'a mut W,
}
impl<'a> LPCLK_SEL_XTAL32K_W<'a> {
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
#[doc = "Field `LPCLK_RTC_EN` reader - Set this bit to enable the RTC low power clock."]
pub struct LPCLK_RTC_EN_R(crate::FieldReader<bool, bool>);
impl LPCLK_RTC_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LPCLK_RTC_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPCLK_RTC_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPCLK_RTC_EN` writer - Set this bit to enable the RTC low power clock."]
pub struct LPCLK_RTC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPCLK_RTC_EN_W<'a> {
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
    #[doc = "Bit 24 - Set this bit to select RTC slow clock as the low power clock."]
    #[inline(always)]
    pub fn lpclk_sel_rtc_slow(&self) -> LPCLK_SEL_RTC_SLOW_R {
        LPCLK_SEL_RTC_SLOW_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Set this bit to select 8m clock as the low power clock."]
    #[inline(always)]
    pub fn lpclk_sel_8m(&self) -> LPCLK_SEL_8M_R {
        LPCLK_SEL_8M_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Set this bit to select xtal clock as the low power clock."]
    #[inline(always)]
    pub fn lpclk_sel_xtal(&self) -> LPCLK_SEL_XTAL_R {
        LPCLK_SEL_XTAL_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Set this bit to select xtal32k clock as the low power clock."]
    #[inline(always)]
    pub fn lpclk_sel_xtal32k(&self) -> LPCLK_SEL_XTAL32K_R {
        LPCLK_SEL_XTAL32K_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Set this bit to enable the RTC low power clock."]
    #[inline(always)]
    pub fn lpclk_rtc_en(&self) -> LPCLK_RTC_EN_R {
        LPCLK_RTC_EN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - Set this bit to select RTC slow clock as the low power clock."]
    #[inline(always)]
    pub fn lpclk_sel_rtc_slow(&mut self) -> LPCLK_SEL_RTC_SLOW_W {
        LPCLK_SEL_RTC_SLOW_W { w: self }
    }
    #[doc = "Bit 25 - Set this bit to select 8m clock as the low power clock."]
    #[inline(always)]
    pub fn lpclk_sel_8m(&mut self) -> LPCLK_SEL_8M_W {
        LPCLK_SEL_8M_W { w: self }
    }
    #[doc = "Bit 26 - Set this bit to select xtal clock as the low power clock."]
    #[inline(always)]
    pub fn lpclk_sel_xtal(&mut self) -> LPCLK_SEL_XTAL_W {
        LPCLK_SEL_XTAL_W { w: self }
    }
    #[doc = "Bit 27 - Set this bit to select xtal32k clock as the low power clock."]
    #[inline(always)]
    pub fn lpclk_sel_xtal32k(&mut self) -> LPCLK_SEL_XTAL32K_W {
        LPCLK_SEL_XTAL32K_W { w: self }
    }
    #[doc = "Bit 28 - Set this bit to enable the RTC low power clock."]
    #[inline(always)]
    pub fn lpclk_rtc_en(&mut self) -> LPCLK_RTC_EN_W {
        LPCLK_RTC_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Divider fraction configuration register for low-power clock\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bt_lpck_div_frac]
(index.html) module"]
pub struct BT_LPCK_DIV_FRAC_SPEC;
impl crate::RegisterSpec for BT_LPCK_DIV_FRAC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bt_lpck_div_frac::R]
(R) reader structure"]
impl crate::Readable for BT_LPCK_DIV_FRAC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bt_lpck_div_frac::W]
(W) writer structure"]
impl crate::Writable for BT_LPCK_DIV_FRAC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BT_LPCK_DIV_FRAC to value 0x0200_0000"]
impl crate::Resettable for BT_LPCK_DIV_FRAC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200_0000
    }
}