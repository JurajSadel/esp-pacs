#[doc = "Register `JTAG_CTRL_5` writer"]
pub struct W(crate::W<JTAG_CTRL_5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<JTAG_CTRL_5_SPEC>;
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
impl From<crate::W<JTAG_CTRL_5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<JTAG_CTRL_5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CANCEL_EFUSE_DISABLE_JTAG_TEMPORARY_5` writer - Stores the 160 to 191 bits of the 256 bits register used to cancel the temporary disable of eFuse to JTAG."]
pub struct CANCEL_EFUSE_DISABLE_JTAG_TEMPORARY_5_W<'a> {
    w: &'a mut W,
}
impl<'a> CANCEL_EFUSE_DISABLE_JTAG_TEMPORARY_5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Stores the 160 to 191 bits of the 256 bits register used to cancel the temporary disable of eFuse to JTAG."]
    #[inline(always)]
    pub fn cancel_efuse_disable_jtag_temporary_5(
        &mut self,
    ) -> CANCEL_EFUSE_DISABLE_JTAG_TEMPORARY_5_W {
        CANCEL_EFUSE_DISABLE_JTAG_TEMPORARY_5_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "JTAG configuration register 5\n\nThis register you can [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jtag_ctrl_5]
(index.html) module"]
pub struct JTAG_CTRL_5_SPEC;
impl crate::RegisterSpec for JTAG_CTRL_5_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [jtag_ctrl_5::W]
(W) writer structure"]
impl crate::Writable for JTAG_CTRL_5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets JTAG_CTRL_5 to value 0"]
impl crate::Resettable for JTAG_CTRL_5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}