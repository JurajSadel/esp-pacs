#[doc = "Register `CH7CONF0` reader"]
pub struct R(crate::R<CH7CONF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH7CONF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH7CONF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH7CONF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH7CONF0` writer"]
pub struct W(crate::W<CH7CONF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH7CONF0_SPEC>;
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
impl From<crate::W<CH7CONF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH7CONF0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIV_CNT_CH7` reader - This register is used to configure the frequency divider's factor in channel7."]
pub struct DIV_CNT_CH7_R(crate::FieldReader<u8, u8>);
impl DIV_CNT_CH7_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIV_CNT_CH7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIV_CNT_CH7_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIV_CNT_CH7` writer - This register is used to configure the frequency divider's factor in channel7."]
pub struct DIV_CNT_CH7_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_CNT_CH7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `IDLE_THRES_CH7` reader - In receive mode when the counter's value is bigger than reg_idle_thres_ch7 then the receive process is done."]
pub struct IDLE_THRES_CH7_R(crate::FieldReader<u16, u16>);
impl IDLE_THRES_CH7_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        IDLE_THRES_CH7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDLE_THRES_CH7_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDLE_THRES_CH7` writer - In receive mode when the counter's value is bigger than reg_idle_thres_ch7 then the receive process is done."]
pub struct IDLE_THRES_CH7_W<'a> {
    w: &'a mut W,
}
impl<'a> IDLE_THRES_CH7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 8)) | ((value as u32 & 0xffff) << 8);
        self.w
    }
}
#[doc = "Field `MEM_SIZE_CH7` reader - This register is used to configure the the amount of memory blocks allocated to channel7."]
pub struct MEM_SIZE_CH7_R(crate::FieldReader<u8, u8>);
impl MEM_SIZE_CH7_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MEM_SIZE_CH7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM_SIZE_CH7_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM_SIZE_CH7` writer - This register is used to configure the the amount of memory blocks allocated to channel7."]
pub struct MEM_SIZE_CH7_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_SIZE_CH7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `CARRIER_EN_CH7` reader - This is the carrier modulation enable control bit for channel7."]
pub struct CARRIER_EN_CH7_R(crate::FieldReader<bool, bool>);
impl CARRIER_EN_CH7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CARRIER_EN_CH7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CARRIER_EN_CH7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CARRIER_EN_CH7` writer - This is the carrier modulation enable control bit for channel7."]
pub struct CARRIER_EN_CH7_W<'a> {
    w: &'a mut W,
}
impl<'a> CARRIER_EN_CH7_W<'a> {
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
#[doc = "Field `CARRIER_OUT_LV_CH7` reader - This bit is used to configure carrier wave's position for channel7.1'b1:add on low level 1'b0:add on high level."]
pub struct CARRIER_OUT_LV_CH7_R(crate::FieldReader<bool, bool>);
impl CARRIER_OUT_LV_CH7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CARRIER_OUT_LV_CH7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CARRIER_OUT_LV_CH7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CARRIER_OUT_LV_CH7` writer - This bit is used to configure carrier wave's position for channel7.1'b1:add on low level 1'b0:add on high level."]
pub struct CARRIER_OUT_LV_CH7_W<'a> {
    w: &'a mut W,
}
impl<'a> CARRIER_OUT_LV_CH7_W<'a> {
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
impl R {
    #[doc = "Bits 0:7 - This register is used to configure the frequency divider's factor in channel7."]
    #[inline(always)]
    pub fn div_cnt_ch7(&self) -> DIV_CNT_CH7_R {
        DIV_CNT_CH7_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:23 - In receive mode when the counter's value is bigger than reg_idle_thres_ch7 then the receive process is done."]
    #[inline(always)]
    pub fn idle_thres_ch7(&self) -> IDLE_THRES_CH7_R {
        IDLE_THRES_CH7_R::new(((self.bits >> 8) & 0xffff) as u16)
    }
    #[doc = "Bits 24:27 - This register is used to configure the the amount of memory blocks allocated to channel7."]
    #[inline(always)]
    pub fn mem_size_ch7(&self) -> MEM_SIZE_CH7_R {
        MEM_SIZE_CH7_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - This is the carrier modulation enable control bit for channel7."]
    #[inline(always)]
    pub fn carrier_en_ch7(&self) -> CARRIER_EN_CH7_R {
        CARRIER_EN_CH7_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - This bit is used to configure carrier wave's position for channel7.1'b1:add on low level 1'b0:add on high level."]
    #[inline(always)]
    pub fn carrier_out_lv_ch7(&self) -> CARRIER_OUT_LV_CH7_R {
        CARRIER_OUT_LV_CH7_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - This register is used to configure the frequency divider's factor in channel7."]
    #[inline(always)]
    pub fn div_cnt_ch7(&mut self) -> DIV_CNT_CH7_W {
        DIV_CNT_CH7_W { w: self }
    }
    #[doc = "Bits 8:23 - In receive mode when the counter's value is bigger than reg_idle_thres_ch7 then the receive process is done."]
    #[inline(always)]
    pub fn idle_thres_ch7(&mut self) -> IDLE_THRES_CH7_W {
        IDLE_THRES_CH7_W { w: self }
    }
    #[doc = "Bits 24:27 - This register is used to configure the the amount of memory blocks allocated to channel7."]
    #[inline(always)]
    pub fn mem_size_ch7(&mut self) -> MEM_SIZE_CH7_W {
        MEM_SIZE_CH7_W { w: self }
    }
    #[doc = "Bit 28 - This is the carrier modulation enable control bit for channel7."]
    #[inline(always)]
    pub fn carrier_en_ch7(&mut self) -> CARRIER_EN_CH7_W {
        CARRIER_EN_CH7_W { w: self }
    }
    #[doc = "Bit 29 - This bit is used to configure carrier wave's position for channel7.1'b1:add on low level 1'b0:add on high level."]
    #[inline(always)]
    pub fn carrier_out_lv_ch7(&mut self) -> CARRIER_OUT_LV_CH7_W {
        CARRIER_OUT_LV_CH7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch7conf0]
(index.html) module"]
pub struct CH7CONF0_SPEC;
impl crate::RegisterSpec for CH7CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch7conf0::R]
(R) reader structure"]
impl crate::Readable for CH7CONF0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch7conf0::W]
(W) writer structure"]
impl crate::Writable for CH7CONF0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CH7CONF0 to value 0x3110_0002"]
impl crate::Resettable for CH7CONF0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3110_0002
    }
}