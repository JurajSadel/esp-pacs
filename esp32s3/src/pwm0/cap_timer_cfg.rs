#[doc = "Register `CAP_TIMER_CFG` reader"]
pub struct R(crate::R<CAP_TIMER_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAP_TIMER_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAP_TIMER_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAP_TIMER_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAP_TIMER_CFG` writer"]
pub struct W(crate::W<CAP_TIMER_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAP_TIMER_CFG_SPEC>;
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
impl From<crate::W<CAP_TIMER_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAP_TIMER_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAP_TIMER_EN` reader - When set, capture timer incrementing under APB_clk is enabled."]
pub struct CAP_TIMER_EN_R(crate::FieldReader<bool, bool>);
impl CAP_TIMER_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CAP_TIMER_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAP_TIMER_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAP_TIMER_EN` writer - When set, capture timer incrementing under APB_clk is enabled."]
pub struct CAP_TIMER_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP_TIMER_EN_W<'a> {
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
#[doc = "Field `CAP_SYNCI_EN` reader - When set, capture timer sync is enabled."]
pub struct CAP_SYNCI_EN_R(crate::FieldReader<bool, bool>);
impl CAP_SYNCI_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CAP_SYNCI_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAP_SYNCI_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAP_SYNCI_EN` writer - When set, capture timer sync is enabled."]
pub struct CAP_SYNCI_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP_SYNCI_EN_W<'a> {
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
#[doc = "Field `CAP_SYNCI_SEL` reader - capture module sync input selection. 0: none, 1: timer0 sync_out, 2: timer1 sync_out, 3: timer2 sync_out, 4: SYNC0 from GPIO matrix, 5: SYNC1 from GPIO matrix, 6: SYNC2 from GPIO matrix"]
pub struct CAP_SYNCI_SEL_R(crate::FieldReader<u8, u8>);
impl CAP_SYNCI_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CAP_SYNCI_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAP_SYNCI_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAP_SYNCI_SEL` writer - capture module sync input selection. 0: none, 1: timer0 sync_out, 2: timer1 sync_out, 3: timer2 sync_out, 4: SYNC0 from GPIO matrix, 5: SYNC1 from GPIO matrix, 6: SYNC2 from GPIO matrix"]
pub struct CAP_SYNCI_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP_SYNCI_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 2)) | ((value as u32 & 7) << 2);
        self.w
    }
}
#[doc = "Field `CAP_SYNC_SW` writer - Write 1 will force a capture timer sync, capture timer is loaded with value in phase register."]
pub struct CAP_SYNC_SW_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP_SYNC_SW_W<'a> {
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
impl R {
    #[doc = "Bit 0 - When set, capture timer incrementing under APB_clk is enabled."]
    #[inline(always)]
    pub fn cap_timer_en(&self) -> CAP_TIMER_EN_R {
        CAP_TIMER_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When set, capture timer sync is enabled."]
    #[inline(always)]
    pub fn cap_synci_en(&self) -> CAP_SYNCI_EN_R {
        CAP_SYNCI_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - capture module sync input selection. 0: none, 1: timer0 sync_out, 2: timer1 sync_out, 3: timer2 sync_out, 4: SYNC0 from GPIO matrix, 5: SYNC1 from GPIO matrix, 6: SYNC2 from GPIO matrix"]
    #[inline(always)]
    pub fn cap_synci_sel(&self) -> CAP_SYNCI_SEL_R {
        CAP_SYNCI_SEL_R::new(((self.bits >> 2) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - When set, capture timer incrementing under APB_clk is enabled."]
    #[inline(always)]
    pub fn cap_timer_en(&mut self) -> CAP_TIMER_EN_W {
        CAP_TIMER_EN_W { w: self }
    }
    #[doc = "Bit 1 - When set, capture timer sync is enabled."]
    #[inline(always)]
    pub fn cap_synci_en(&mut self) -> CAP_SYNCI_EN_W {
        CAP_SYNCI_EN_W { w: self }
    }
    #[doc = "Bits 2:4 - capture module sync input selection. 0: none, 1: timer0 sync_out, 2: timer1 sync_out, 3: timer2 sync_out, 4: SYNC0 from GPIO matrix, 5: SYNC1 from GPIO matrix, 6: SYNC2 from GPIO matrix"]
    #[inline(always)]
    pub fn cap_synci_sel(&mut self) -> CAP_SYNCI_SEL_W {
        CAP_SYNCI_SEL_W { w: self }
    }
    #[doc = "Bit 5 - Write 1 will force a capture timer sync, capture timer is loaded with value in phase register."]
    #[inline(always)]
    pub fn cap_sync_sw(&mut self) -> CAP_SYNC_SW_W {
        CAP_SYNC_SW_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configure capture timer\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cap_timer_cfg]
(index.html) module"]
pub struct CAP_TIMER_CFG_SPEC;
impl crate::RegisterSpec for CAP_TIMER_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cap_timer_cfg::R]
(R) reader structure"]
impl crate::Readable for CAP_TIMER_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cap_timer_cfg::W]
(W) writer structure"]
impl crate::Writable for CAP_TIMER_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAP_TIMER_CFG to value 0"]
impl crate::Resettable for CAP_TIMER_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}