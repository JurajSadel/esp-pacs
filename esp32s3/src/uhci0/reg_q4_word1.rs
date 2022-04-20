#[doc = "Register `REG_Q4_WORD1` reader"]
pub struct R(crate::R<REG_Q4_WORD1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG_Q4_WORD1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REG_Q4_WORD1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REG_Q4_WORD1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REG_Q4_WORD1` writer"]
pub struct W(crate::W<REG_Q4_WORD1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REG_Q4_WORD1_SPEC>;
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
impl From<crate::W<REG_Q4_WORD1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REG_Q4_WORD1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEND_Q4_WORD1` reader - This register is used as a quick_sent register when specified by UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM."]
pub struct SEND_Q4_WORD1_R(crate::FieldReader<u32, u32>);
impl SEND_Q4_WORD1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        SEND_Q4_WORD1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEND_Q4_WORD1_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEND_Q4_WORD1` writer - This register is used as a quick_sent register when specified by UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM."]
pub struct SEND_Q4_WORD1_W<'a> {
    w: &'a mut W,
}
impl<'a> SEND_Q4_WORD1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - This register is used as a quick_sent register when specified by UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM."]
    #[inline(always)]
    pub fn send_q4_word1(&self) -> SEND_Q4_WORD1_R {
        SEND_Q4_WORD1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This register is used as a quick_sent register when specified by UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM."]
    #[inline(always)]
    pub fn send_q4_word1(&mut self) -> SEND_Q4_WORD1_W {
        SEND_Q4_WORD1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Q4_WORD1 quick_sent register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_q4_word1]
(index.html) module"]
pub struct REG_Q4_WORD1_SPEC;
impl crate::RegisterSpec for REG_Q4_WORD1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg_q4_word1::R]
(R) reader structure"]
impl crate::Readable for REG_Q4_WORD1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reg_q4_word1::W]
(W) writer structure"]
impl crate::Writable for REG_Q4_WORD1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REG_Q4_WORD1 to value 0"]
impl crate::Resettable for REG_Q4_WORD1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}