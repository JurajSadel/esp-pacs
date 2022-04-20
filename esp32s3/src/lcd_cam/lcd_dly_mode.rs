#[doc = "Register `LCD_DLY_MODE` reader"]
pub struct R(crate::R<LCD_DLY_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCD_DLY_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCD_DLY_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCD_DLY_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCD_DLY_MODE` writer"]
pub struct W(crate::W<LCD_DLY_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCD_DLY_MODE_SPEC>;
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
impl From<crate::W<LCD_DLY_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCD_DLY_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LCD_CD_MODE` reader - The output LCD_CD is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub struct LCD_CD_MODE_R(crate::FieldReader<u8, u8>);
impl LCD_CD_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LCD_CD_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCD_CD_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCD_CD_MODE` writer - The output LCD_CD is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub struct LCD_CD_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> LCD_CD_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !3) | (value as u32 & 3);
        self.w
    }
}
#[doc = "Field `LCD_DE_MODE` reader - The output LCD_DE is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub struct LCD_DE_MODE_R(crate::FieldReader<u8, u8>);
impl LCD_DE_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LCD_DE_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCD_DE_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCD_DE_MODE` writer - The output LCD_DE is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub struct LCD_DE_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> LCD_DE_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 2)) | ((value as u32 & 3) << 2);
        self.w
    }
}
#[doc = "Field `LCD_HSYNC_MODE` reader - The output LCD_HSYNC is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub struct LCD_HSYNC_MODE_R(crate::FieldReader<u8, u8>);
impl LCD_HSYNC_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LCD_HSYNC_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCD_HSYNC_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCD_HSYNC_MODE` writer - The output LCD_HSYNC is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub struct LCD_HSYNC_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> LCD_HSYNC_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 4)) | ((value as u32 & 3) << 4);
        self.w
    }
}
#[doc = "Field `LCD_VSYNC_MODE` reader - The output LCD_VSYNC is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub struct LCD_VSYNC_MODE_R(crate::FieldReader<u8, u8>);
impl LCD_VSYNC_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LCD_VSYNC_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCD_VSYNC_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCD_VSYNC_MODE` writer - The output LCD_VSYNC is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub struct LCD_VSYNC_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> LCD_VSYNC_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 6)) | ((value as u32 & 3) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - The output LCD_CD is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    pub fn lcd_cd_mode(&self) -> LCD_CD_MODE_R {
        LCD_CD_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - The output LCD_DE is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    pub fn lcd_de_mode(&self) -> LCD_DE_MODE_R {
        LCD_DE_MODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - The output LCD_HSYNC is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    pub fn lcd_hsync_mode(&self) -> LCD_HSYNC_MODE_R {
        LCD_HSYNC_MODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - The output LCD_VSYNC is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    pub fn lcd_vsync_mode(&self) -> LCD_VSYNC_MODE_R {
        LCD_VSYNC_MODE_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - The output LCD_CD is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    pub fn lcd_cd_mode(&mut self) -> LCD_CD_MODE_W {
        LCD_CD_MODE_W { w: self }
    }
    #[doc = "Bits 2:3 - The output LCD_DE is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    pub fn lcd_de_mode(&mut self) -> LCD_DE_MODE_W {
        LCD_DE_MODE_W { w: self }
    }
    #[doc = "Bits 4:5 - The output LCD_HSYNC is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    pub fn lcd_hsync_mode(&mut self) -> LCD_HSYNC_MODE_W {
        LCD_HSYNC_MODE_W { w: self }
    }
    #[doc = "Bits 6:7 - The output LCD_VSYNC is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    pub fn lcd_vsync_mode(&mut self) -> LCD_VSYNC_MODE_W {
        LCD_VSYNC_MODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD configuration register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcd_dly_mode]
(index.html) module"]
pub struct LCD_DLY_MODE_SPEC;
impl crate::RegisterSpec for LCD_DLY_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcd_dly_mode::R]
(R) reader structure"]
impl crate::Readable for LCD_DLY_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcd_dly_mode::W]
(W) writer structure"]
impl crate::Writable for LCD_DLY_MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LCD_DLY_MODE to value 0"]
impl crate::Resettable for LCD_DLY_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}