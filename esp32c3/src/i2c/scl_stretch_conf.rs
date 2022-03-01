#[doc = "Register `SCL_STRETCH_CONF` reader"]
pub struct R(crate::R<SCL_STRETCH_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCL_STRETCH_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCL_STRETCH_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCL_STRETCH_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCL_STRETCH_CONF` writer"]
pub struct W(crate::W<SCL_STRETCH_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCL_STRETCH_CONF_SPEC>;
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
impl From<crate::W<SCL_STRETCH_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCL_STRETCH_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STRETCH_PROTECT_NUM` reader - reg_stretch_protect_num"]
pub struct STRETCH_PROTECT_NUM_R(crate::FieldReader<u16, u16>);
impl STRETCH_PROTECT_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        STRETCH_PROTECT_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STRETCH_PROTECT_NUM_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STRETCH_PROTECT_NUM` writer - reg_stretch_protect_num"]
pub struct STRETCH_PROTECT_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> STRETCH_PROTECT_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
#[doc = "Field `SLAVE_SCL_STRETCH_EN` reader - reg_slave_scl_stretch_en"]
pub struct SLAVE_SCL_STRETCH_EN_R(crate::FieldReader<bool, bool>);
impl SLAVE_SCL_STRETCH_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLAVE_SCL_STRETCH_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLAVE_SCL_STRETCH_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLAVE_SCL_STRETCH_EN` writer - reg_slave_scl_stretch_en"]
pub struct SLAVE_SCL_STRETCH_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE_SCL_STRETCH_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `SLAVE_SCL_STRETCH_CLR` writer - reg_slave_scl_stretch_clr"]
pub struct SLAVE_SCL_STRETCH_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE_SCL_STRETCH_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `SLAVE_BYTE_ACK_CTL_EN` reader - reg_slave_byte_ack_ctl_en"]
pub struct SLAVE_BYTE_ACK_CTL_EN_R(crate::FieldReader<bool, bool>);
impl SLAVE_BYTE_ACK_CTL_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLAVE_BYTE_ACK_CTL_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLAVE_BYTE_ACK_CTL_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLAVE_BYTE_ACK_CTL_EN` writer - reg_slave_byte_ack_ctl_en"]
pub struct SLAVE_BYTE_ACK_CTL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE_BYTE_ACK_CTL_EN_W<'a> {
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
#[doc = "Field `SLAVE_BYTE_ACK_LVL` reader - reg_slave_byte_ack_lvl"]
pub struct SLAVE_BYTE_ACK_LVL_R(crate::FieldReader<bool, bool>);
impl SLAVE_BYTE_ACK_LVL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLAVE_BYTE_ACK_LVL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLAVE_BYTE_ACK_LVL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLAVE_BYTE_ACK_LVL` writer - reg_slave_byte_ack_lvl"]
pub struct SLAVE_BYTE_ACK_LVL_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE_BYTE_ACK_LVL_W<'a> {
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
impl R {
    #[doc = "Bits 0:9 - reg_stretch_protect_num"]
    #[inline(always)]
    pub fn stretch_protect_num(&self) -> STRETCH_PROTECT_NUM_R {
        STRETCH_PROTECT_NUM_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - reg_slave_scl_stretch_en"]
    #[inline(always)]
    pub fn slave_scl_stretch_en(&self) -> SLAVE_SCL_STRETCH_EN_R {
        SLAVE_SCL_STRETCH_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 12 - reg_slave_byte_ack_ctl_en"]
    #[inline(always)]
    pub fn slave_byte_ack_ctl_en(&self) -> SLAVE_BYTE_ACK_CTL_EN_R {
        SLAVE_BYTE_ACK_CTL_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - reg_slave_byte_ack_lvl"]
    #[inline(always)]
    pub fn slave_byte_ack_lvl(&self) -> SLAVE_BYTE_ACK_LVL_R {
        SLAVE_BYTE_ACK_LVL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - reg_stretch_protect_num"]
    #[inline(always)]
    pub fn stretch_protect_num(&mut self) -> STRETCH_PROTECT_NUM_W {
        STRETCH_PROTECT_NUM_W { w: self }
    }
    #[doc = "Bit 10 - reg_slave_scl_stretch_en"]
    #[inline(always)]
    pub fn slave_scl_stretch_en(&mut self) -> SLAVE_SCL_STRETCH_EN_W {
        SLAVE_SCL_STRETCH_EN_W { w: self }
    }
    #[doc = "Bit 11 - reg_slave_scl_stretch_clr"]
    #[inline(always)]
    pub fn slave_scl_stretch_clr(&mut self) -> SLAVE_SCL_STRETCH_CLR_W {
        SLAVE_SCL_STRETCH_CLR_W { w: self }
    }
    #[doc = "Bit 12 - reg_slave_byte_ack_ctl_en"]
    #[inline(always)]
    pub fn slave_byte_ack_ctl_en(&mut self) -> SLAVE_BYTE_ACK_CTL_EN_W {
        SLAVE_BYTE_ACK_CTL_EN_W { w: self }
    }
    #[doc = "Bit 13 - reg_slave_byte_ack_lvl"]
    #[inline(always)]
    pub fn slave_byte_ack_lvl(&mut self) -> SLAVE_BYTE_ACK_LVL_W {
        SLAVE_BYTE_ACK_LVL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C_SCL_STRETCH_CONF_REG\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scl_stretch_conf]
(index.html) module"]
pub struct SCL_STRETCH_CONF_SPEC;
impl crate::RegisterSpec for SCL_STRETCH_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scl_stretch_conf::R]
(R) reader structure"]
impl crate::Readable for SCL_STRETCH_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scl_stretch_conf::W]
(W) writer structure"]
impl crate::Writable for SCL_STRETCH_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCL_STRETCH_CONF to value 0"]
impl crate::Resettable for SCL_STRETCH_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}