#[doc = "Register `CACHE_SCTRL` reader"]
pub struct R(crate::R<CACHE_SCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CACHE_SCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CACHE_SCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CACHE_SCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CACHE_SCTRL` writer"]
pub struct W(crate::W<CACHE_SCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CACHE_SCTRL_SPEC>;
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
impl From<crate::W<CACHE_SCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CACHE_SCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USR_SRAM_DIO` reader - For SPI0 In the spi sram mode spi dual I/O mode enable 1: enable 0:disable"]
pub struct USR_SRAM_DIO_R(crate::FieldReader<bool, bool>);
impl USR_SRAM_DIO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USR_SRAM_DIO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USR_SRAM_DIO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USR_SRAM_DIO` writer - For SPI0 In the spi sram mode spi dual I/O mode enable 1: enable 0:disable"]
pub struct USR_SRAM_DIO_W<'a> {
    w: &'a mut W,
}
impl<'a> USR_SRAM_DIO_W<'a> {
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
#[doc = "Field `USR_SRAM_QIO` reader - For SPI0 In the spi sram mode spi quad I/O mode enable 1: enable 0:disable"]
pub struct USR_SRAM_QIO_R(crate::FieldReader<bool, bool>);
impl USR_SRAM_QIO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USR_SRAM_QIO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USR_SRAM_QIO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USR_SRAM_QIO` writer - For SPI0 In the spi sram mode spi quad I/O mode enable 1: enable 0:disable"]
pub struct USR_SRAM_QIO_W<'a> {
    w: &'a mut W,
}
impl<'a> USR_SRAM_QIO_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 2)) | ((value as u32 & 1) << 2);
        self.w
    }
}
#[doc = "Field `USR_WR_SRAM_DUMMY` reader - For SPI0 In the spi sram mode it is the enable bit of dummy phase for write operations."]
pub struct USR_WR_SRAM_DUMMY_R(crate::FieldReader<bool, bool>);
impl USR_WR_SRAM_DUMMY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USR_WR_SRAM_DUMMY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USR_WR_SRAM_DUMMY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USR_WR_SRAM_DUMMY` writer - For SPI0 In the spi sram mode it is the enable bit of dummy phase for write operations."]
pub struct USR_WR_SRAM_DUMMY_W<'a> {
    w: &'a mut W,
}
impl<'a> USR_WR_SRAM_DUMMY_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 3)) | ((value as u32 & 1) << 3);
        self.w
    }
}
#[doc = "Field `USR_RD_SRAM_DUMMY` reader - For SPI0 In the spi sram mode it is the enable bit of dummy phase for read operations."]
pub struct USR_RD_SRAM_DUMMY_R(crate::FieldReader<bool, bool>);
impl USR_RD_SRAM_DUMMY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USR_RD_SRAM_DUMMY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USR_RD_SRAM_DUMMY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USR_RD_SRAM_DUMMY` writer - For SPI0 In the spi sram mode it is the enable bit of dummy phase for read operations."]
pub struct USR_RD_SRAM_DUMMY_W<'a> {
    w: &'a mut W,
}
impl<'a> USR_RD_SRAM_DUMMY_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u32 & 1) << 4);
        self.w
    }
}
#[doc = "Field `CACHE_SRAM_USR_RCMD` reader - For SPI0 In the spi sram mode cache read sram for user define command."]
pub struct CACHE_SRAM_USR_RCMD_R(crate::FieldReader<bool, bool>);
impl CACHE_SRAM_USR_RCMD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CACHE_SRAM_USR_RCMD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CACHE_SRAM_USR_RCMD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CACHE_SRAM_USR_RCMD` writer - For SPI0 In the spi sram mode cache read sram for user define command."]
pub struct CACHE_SRAM_USR_RCMD_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHE_SRAM_USR_RCMD_W<'a> {
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
#[doc = "Field `SRAM_BYTES_LEN` reader - For SPI0 In the sram mode it is the byte length of spi read sram data."]
pub struct SRAM_BYTES_LEN_R(crate::FieldReader<u8, u8>);
impl SRAM_BYTES_LEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SRAM_BYTES_LEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM_BYTES_LEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM_BYTES_LEN` writer - For SPI0 In the sram mode it is the byte length of spi read sram data."]
pub struct SRAM_BYTES_LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_BYTES_LEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 6)) | ((value as u32 & 0xff) << 6);
        self.w
    }
}
#[doc = "Field `SRAM_DUMMY_CYCLELEN` reader - For SPI0 In the sram mode it is the length in bits of address phase. The register value shall be (bit_num-1)."]
pub struct SRAM_DUMMY_CYCLELEN_R(crate::FieldReader<u8, u8>);
impl SRAM_DUMMY_CYCLELEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SRAM_DUMMY_CYCLELEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM_DUMMY_CYCLELEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM_DUMMY_CYCLELEN` writer - For SPI0 In the sram mode it is the length in bits of address phase. The register value shall be (bit_num-1)."]
pub struct SRAM_DUMMY_CYCLELEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_DUMMY_CYCLELEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 14)) | ((value as u32 & 0xff) << 14);
        self.w
    }
}
#[doc = "Field `SRAM_ADDR_BITLEN` reader - For SPI0 In the sram mode it is the length in bits of address phase. The register value shall be (bit_num-1)."]
pub struct SRAM_ADDR_BITLEN_R(crate::FieldReader<u8, u8>);
impl SRAM_ADDR_BITLEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SRAM_ADDR_BITLEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM_ADDR_BITLEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM_ADDR_BITLEN` writer - For SPI0 In the sram mode it is the length in bits of address phase. The register value shall be (bit_num-1)."]
pub struct SRAM_ADDR_BITLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_ADDR_BITLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 22)) | ((value as u32 & 0x3f) << 22);
        self.w
    }
}
#[doc = "Field `CACHE_SRAM_USR_WCMD` reader - For SPI0 In the spi sram mode cache write sram for user define command"]
pub struct CACHE_SRAM_USR_WCMD_R(crate::FieldReader<bool, bool>);
impl CACHE_SRAM_USR_WCMD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CACHE_SRAM_USR_WCMD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CACHE_SRAM_USR_WCMD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CACHE_SRAM_USR_WCMD` writer - For SPI0 In the spi sram mode cache write sram for user define command"]
pub struct CACHE_SRAM_USR_WCMD_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHE_SRAM_USR_WCMD_W<'a> {
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
impl R {
    #[doc = "Bit 1 - For SPI0 In the spi sram mode spi dual I/O mode enable 1: enable 0:disable"]
    #[inline(always)]
    pub fn usr_sram_dio(&self) -> USR_SRAM_DIO_R {
        USR_SRAM_DIO_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - For SPI0 In the spi sram mode spi quad I/O mode enable 1: enable 0:disable"]
    #[inline(always)]
    pub fn usr_sram_qio(&self) -> USR_SRAM_QIO_R {
        USR_SRAM_QIO_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - For SPI0 In the spi sram mode it is the enable bit of dummy phase for write operations."]
    #[inline(always)]
    pub fn usr_wr_sram_dummy(&self) -> USR_WR_SRAM_DUMMY_R {
        USR_WR_SRAM_DUMMY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - For SPI0 In the spi sram mode it is the enable bit of dummy phase for read operations."]
    #[inline(always)]
    pub fn usr_rd_sram_dummy(&self) -> USR_RD_SRAM_DUMMY_R {
        USR_RD_SRAM_DUMMY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - For SPI0 In the spi sram mode cache read sram for user define command."]
    #[inline(always)]
    pub fn cache_sram_usr_rcmd(&self) -> CACHE_SRAM_USR_RCMD_R {
        CACHE_SRAM_USR_RCMD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:13 - For SPI0 In the sram mode it is the byte length of spi read sram data."]
    #[inline(always)]
    pub fn sram_bytes_len(&self) -> SRAM_BYTES_LEN_R {
        SRAM_BYTES_LEN_R::new(((self.bits >> 6) & 0xff) as u8)
    }
    #[doc = "Bits 14:21 - For SPI0 In the sram mode it is the length in bits of address phase. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn sram_dummy_cyclelen(&self) -> SRAM_DUMMY_CYCLELEN_R {
        SRAM_DUMMY_CYCLELEN_R::new(((self.bits >> 14) & 0xff) as u8)
    }
    #[doc = "Bits 22:27 - For SPI0 In the sram mode it is the length in bits of address phase. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn sram_addr_bitlen(&self) -> SRAM_ADDR_BITLEN_R {
        SRAM_ADDR_BITLEN_R::new(((self.bits >> 22) & 0x3f) as u8)
    }
    #[doc = "Bit 28 - For SPI0 In the spi sram mode cache write sram for user define command"]
    #[inline(always)]
    pub fn cache_sram_usr_wcmd(&self) -> CACHE_SRAM_USR_WCMD_R {
        CACHE_SRAM_USR_WCMD_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - For SPI0 In the spi sram mode spi dual I/O mode enable 1: enable 0:disable"]
    #[inline(always)]
    pub fn usr_sram_dio(&mut self) -> USR_SRAM_DIO_W {
        USR_SRAM_DIO_W { w: self }
    }
    #[doc = "Bit 2 - For SPI0 In the spi sram mode spi quad I/O mode enable 1: enable 0:disable"]
    #[inline(always)]
    pub fn usr_sram_qio(&mut self) -> USR_SRAM_QIO_W {
        USR_SRAM_QIO_W { w: self }
    }
    #[doc = "Bit 3 - For SPI0 In the spi sram mode it is the enable bit of dummy phase for write operations."]
    #[inline(always)]
    pub fn usr_wr_sram_dummy(&mut self) -> USR_WR_SRAM_DUMMY_W {
        USR_WR_SRAM_DUMMY_W { w: self }
    }
    #[doc = "Bit 4 - For SPI0 In the spi sram mode it is the enable bit of dummy phase for read operations."]
    #[inline(always)]
    pub fn usr_rd_sram_dummy(&mut self) -> USR_RD_SRAM_DUMMY_W {
        USR_RD_SRAM_DUMMY_W { w: self }
    }
    #[doc = "Bit 5 - For SPI0 In the spi sram mode cache read sram for user define command."]
    #[inline(always)]
    pub fn cache_sram_usr_rcmd(&mut self) -> CACHE_SRAM_USR_RCMD_W {
        CACHE_SRAM_USR_RCMD_W { w: self }
    }
    #[doc = "Bits 6:13 - For SPI0 In the sram mode it is the byte length of spi read sram data."]
    #[inline(always)]
    pub fn sram_bytes_len(&mut self) -> SRAM_BYTES_LEN_W {
        SRAM_BYTES_LEN_W { w: self }
    }
    #[doc = "Bits 14:21 - For SPI0 In the sram mode it is the length in bits of address phase. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn sram_dummy_cyclelen(&mut self) -> SRAM_DUMMY_CYCLELEN_W {
        SRAM_DUMMY_CYCLELEN_W { w: self }
    }
    #[doc = "Bits 22:27 - For SPI0 In the sram mode it is the length in bits of address phase. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn sram_addr_bitlen(&mut self) -> SRAM_ADDR_BITLEN_W {
        SRAM_ADDR_BITLEN_W { w: self }
    }
    #[doc = "Bit 28 - For SPI0 In the spi sram mode cache write sram for user define command"]
    #[inline(always)]
    pub fn cache_sram_usr_wcmd(&mut self) -> CACHE_SRAM_USR_WCMD_W {
        CACHE_SRAM_USR_WCMD_W { w: self }
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
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cache_sctrl]
(index.html) module"]
pub struct CACHE_SCTRL_SPEC;
impl crate::RegisterSpec for CACHE_SCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cache_sctrl::R]
(R) reader structure"]
impl crate::Readable for CACHE_SCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cache_sctrl::W]
(W) writer structure"]
impl crate::Writable for CACHE_SCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CACHE_SCTRL to value 0x15c0_4830"]
impl crate::Resettable for CACHE_SCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x15c0_4830
    }
}