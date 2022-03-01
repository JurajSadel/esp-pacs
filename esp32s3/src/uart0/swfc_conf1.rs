#[doc = "Register `SWFC_CONF1` reader"]
pub struct R(crate::R<SWFC_CONF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWFC_CONF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWFC_CONF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWFC_CONF1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SWFC_CONF1` writer"]
pub struct W(crate::W<SWFC_CONF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWFC_CONF1_SPEC>;
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
impl From<crate::W<SWFC_CONF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWFC_CONF1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XON_THRESHOLD` reader - When the data amount in Rx-FIFO is less than this register value with uart_sw_flow_con_en set to 1, it will send a Xon char."]
pub struct XON_THRESHOLD_R(crate::FieldReader<u16, u16>);
impl XON_THRESHOLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        XON_THRESHOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XON_THRESHOLD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XON_THRESHOLD` writer - When the data amount in Rx-FIFO is less than this register value with uart_sw_flow_con_en set to 1, it will send a Xon char."]
pub struct XON_THRESHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> XON_THRESHOLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
#[doc = "Field `XON_CHAR` reader - This register stores the Xon flow control char."]
pub struct XON_CHAR_R(crate::FieldReader<u8, u8>);
impl XON_CHAR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        XON_CHAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XON_CHAR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XON_CHAR` writer - This register stores the Xon flow control char."]
pub struct XON_CHAR_W<'a> {
    w: &'a mut W,
}
impl<'a> XON_CHAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 10)) | ((value as u32 & 0xff) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - When the data amount in Rx-FIFO is less than this register value with uart_sw_flow_con_en set to 1, it will send a Xon char."]
    #[inline(always)]
    pub fn xon_threshold(&self) -> XON_THRESHOLD_R {
        XON_THRESHOLD_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:17 - This register stores the Xon flow control char."]
    #[inline(always)]
    pub fn xon_char(&self) -> XON_CHAR_R {
        XON_CHAR_R::new(((self.bits >> 10) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - When the data amount in Rx-FIFO is less than this register value with uart_sw_flow_con_en set to 1, it will send a Xon char."]
    #[inline(always)]
    pub fn xon_threshold(&mut self) -> XON_THRESHOLD_W {
        XON_THRESHOLD_W { w: self }
    }
    #[doc = "Bits 10:17 - This register stores the Xon flow control char."]
    #[inline(always)]
    pub fn xon_char(&mut self) -> XON_CHAR_W {
        XON_CHAR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Software flow-control character configuration\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swfc_conf1]
(index.html) module"]
pub struct SWFC_CONF1_SPEC;
impl crate::RegisterSpec for SWFC_CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [swfc_conf1::R]
(R) reader structure"]
impl crate::Readable for SWFC_CONF1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [swfc_conf1::W]
(W) writer structure"]
impl crate::Writable for SWFC_CONF1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SWFC_CONF1 to value 0x4400"]
impl crate::Resettable for SWFC_CONF1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4400
    }
}