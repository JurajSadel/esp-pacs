#[doc = "Register `EXT_WAKEUP_CONF` reader"]
pub struct R(crate::R<EXT_WAKEUP_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXT_WAKEUP_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXT_WAKEUP_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXT_WAKEUP_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXT_WAKEUP_CONF` writer"]
pub struct W(crate::W<EXT_WAKEUP_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXT_WAKEUP_CONF_SPEC>;
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
impl From<crate::W<EXT_WAKEUP_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXT_WAKEUP_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO_WAKEUP_FILTER` reader - Set this bit to enable the GPIO wakeup event filter."]
pub struct GPIO_WAKEUP_FILTER_R(crate::FieldReader<bool, bool>);
impl GPIO_WAKEUP_FILTER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO_WAKEUP_FILTER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO_WAKEUP_FILTER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_WAKEUP_FILTER` writer - Set this bit to enable the GPIO wakeup event filter."]
pub struct GPIO_WAKEUP_FILTER_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_WAKEUP_FILTER_W<'a> {
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
#[doc = "Field `EXT_WAKEUP0_LV` reader - 0: external wakeup 0 at low level 1: external wakeup 0 at high level"]
pub struct EXT_WAKEUP0_LV_R(crate::FieldReader<bool, bool>);
impl EXT_WAKEUP0_LV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXT_WAKEUP0_LV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXT_WAKEUP0_LV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXT_WAKEUP0_LV` writer - 0: external wakeup 0 at low level 1: external wakeup 0 at high level"]
pub struct EXT_WAKEUP0_LV_W<'a> {
    w: &'a mut W,
}
impl<'a> EXT_WAKEUP0_LV_W<'a> {
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
#[doc = "Field `EXT_WAKEUP1_LV` reader - 0: external wakeup 1 at low level 1: external wakeup 1 at high level"]
pub struct EXT_WAKEUP1_LV_R(crate::FieldReader<bool, bool>);
impl EXT_WAKEUP1_LV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXT_WAKEUP1_LV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXT_WAKEUP1_LV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXT_WAKEUP1_LV` writer - 0: external wakeup 1 at low level 1: external wakeup 1 at high level"]
pub struct EXT_WAKEUP1_LV_W<'a> {
    w: &'a mut W,
}
impl<'a> EXT_WAKEUP1_LV_W<'a> {
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
    #[doc = "Bit 29 - Set this bit to enable the GPIO wakeup event filter."]
    #[inline(always)]
    pub fn gpio_wakeup_filter(&self) -> GPIO_WAKEUP_FILTER_R {
        GPIO_WAKEUP_FILTER_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 0: external wakeup 0 at low level 1: external wakeup 0 at high level"]
    #[inline(always)]
    pub fn ext_wakeup0_lv(&self) -> EXT_WAKEUP0_LV_R {
        EXT_WAKEUP0_LV_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 0: external wakeup 1 at low level 1: external wakeup 1 at high level"]
    #[inline(always)]
    pub fn ext_wakeup1_lv(&self) -> EXT_WAKEUP1_LV_R {
        EXT_WAKEUP1_LV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 29 - Set this bit to enable the GPIO wakeup event filter."]
    #[inline(always)]
    pub fn gpio_wakeup_filter(&mut self) -> GPIO_WAKEUP_FILTER_W {
        GPIO_WAKEUP_FILTER_W { w: self }
    }
    #[doc = "Bit 30 - 0: external wakeup 0 at low level 1: external wakeup 0 at high level"]
    #[inline(always)]
    pub fn ext_wakeup0_lv(&mut self) -> EXT_WAKEUP0_LV_W {
        EXT_WAKEUP0_LV_W { w: self }
    }
    #[doc = "Bit 31 - 0: external wakeup 1 at low level 1: external wakeup 1 at high level"]
    #[inline(always)]
    pub fn ext_wakeup1_lv(&mut self) -> EXT_WAKEUP1_LV_W {
        EXT_WAKEUP1_LV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO wakeup configuration register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ext_wakeup_conf]
(index.html) module"]
pub struct EXT_WAKEUP_CONF_SPEC;
impl crate::RegisterSpec for EXT_WAKEUP_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ext_wakeup_conf::R]
(R) reader structure"]
impl crate::Readable for EXT_WAKEUP_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ext_wakeup_conf::W]
(W) writer structure"]
impl crate::Writable for EXT_WAKEUP_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXT_WAKEUP_CONF to value 0"]
impl crate::Resettable for EXT_WAKEUP_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}