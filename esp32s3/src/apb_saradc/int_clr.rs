#[doc = "Register `INT_CLR` writer"]
pub struct W(crate::W<INT_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_CLR_SPEC>;
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
impl From<crate::W<INT_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `THRES1_LOW_INT_CLR` writer - interrupt of thres1 low"]
pub struct THRES1_LOW_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> THRES1_LOW_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 26)) | ((value as u32 & 1) << 26);
        self.w
    }
}
#[doc = "Field `THRES0_LOW_INT_CLR` writer - interrupt of thres0 low"]
pub struct THRES0_LOW_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> THRES0_LOW_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 27)) | ((value as u32 & 1) << 27);
        self.w
    }
}
#[doc = "Field `THRES1_HIGH_INT_CLR` writer - interrupt of thres1 high"]
pub struct THRES1_HIGH_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> THRES1_HIGH_INT_CLR_W<'a> {
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
#[doc = "Field `THRES0_HIGH_INT_CLR` writer - interrupt of thres0 high"]
pub struct THRES0_HIGH_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> THRES0_HIGH_INT_CLR_W<'a> {
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
#[doc = "Field `APB_SARADC2_DONE_INT_CLR` writer - interrupt of sar2 done"]
pub struct APB_SARADC2_DONE_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC2_DONE_INT_CLR_W<'a> {
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
#[doc = "Field `APB_SARADC1_DONE_INT_CLR` writer - interrupt of sar1 done"]
pub struct APB_SARADC1_DONE_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC1_DONE_INT_CLR_W<'a> {
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
impl W {
    #[doc = "Bit 26 - interrupt of thres1 low"]
    #[inline(always)]
    pub fn thres1_low_int_clr(&mut self) -> THRES1_LOW_INT_CLR_W {
        THRES1_LOW_INT_CLR_W { w: self }
    }
    #[doc = "Bit 27 - interrupt of thres0 low"]
    #[inline(always)]
    pub fn thres0_low_int_clr(&mut self) -> THRES0_LOW_INT_CLR_W {
        THRES0_LOW_INT_CLR_W { w: self }
    }
    #[doc = "Bit 28 - interrupt of thres1 high"]
    #[inline(always)]
    pub fn thres1_high_int_clr(&mut self) -> THRES1_HIGH_INT_CLR_W {
        THRES1_HIGH_INT_CLR_W { w: self }
    }
    #[doc = "Bit 29 - interrupt of thres0 high"]
    #[inline(always)]
    pub fn thres0_high_int_clr(&mut self) -> THRES0_HIGH_INT_CLR_W {
        THRES0_HIGH_INT_CLR_W { w: self }
    }
    #[doc = "Bit 30 - interrupt of sar2 done"]
    #[inline(always)]
    pub fn apb_saradc2_done_int_clr(&mut self) -> APB_SARADC2_DONE_INT_CLR_W {
        APB_SARADC2_DONE_INT_CLR_W { w: self }
    }
    #[doc = "Bit 31 - interrupt of sar1 done"]
    #[inline(always)]
    pub fn apb_saradc1_done_int_clr(&mut self) -> APB_SARADC1_DONE_INT_CLR_W {
        APB_SARADC1_DONE_INT_CLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "clear interrupt\n\nThis register you can [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_clr]
(index.html) module"]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [int_clr::W]
(W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}