#[doc = "Register `CORE_0_INTERRUPT_RLS` reader"]
pub struct R(crate::R<CORE_0_INTERRUPT_RLS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_0_INTERRUPT_RLS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_0_INTERRUPT_RLS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_0_INTERRUPT_RLS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORE_0_INTERRUPT_RLS` writer"]
pub struct W(crate::W<CORE_0_INTERRUPT_RLS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_0_INTERRUPT_RLS_SPEC>;
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
impl From<crate::W<CORE_0_INTERRUPT_RLS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_0_INTERRUPT_RLS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_0_AREA_DRAM0_0_RD_RLS` reader - Core0 dram0 area0 read monitor interrupt enable"]
pub struct CORE_0_AREA_DRAM0_0_RD_RLS_R(crate::FieldReader<bool, bool>);
impl CORE_0_AREA_DRAM0_0_RD_RLS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CORE_0_AREA_DRAM0_0_RD_RLS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_AREA_DRAM0_0_RD_RLS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_AREA_DRAM0_0_RD_RLS` writer - Core0 dram0 area0 read monitor interrupt enable"]
pub struct CORE_0_AREA_DRAM0_0_RD_RLS_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_AREA_DRAM0_0_RD_RLS_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `CORE_0_AREA_DRAM0_0_WR_RLS` reader - Core0 dram0 area0 write monitor interrupt enable"]
pub struct CORE_0_AREA_DRAM0_0_WR_RLS_R(crate::FieldReader<bool, bool>);
impl CORE_0_AREA_DRAM0_0_WR_RLS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CORE_0_AREA_DRAM0_0_WR_RLS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_AREA_DRAM0_0_WR_RLS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_AREA_DRAM0_0_WR_RLS` writer - Core0 dram0 area0 write monitor interrupt enable"]
pub struct CORE_0_AREA_DRAM0_0_WR_RLS_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_AREA_DRAM0_0_WR_RLS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `CORE_0_AREA_DRAM0_1_RD_RLS` reader - Core0 dram0 area1 read monitor interrupt enable"]
pub struct CORE_0_AREA_DRAM0_1_RD_RLS_R(crate::FieldReader<bool, bool>);
impl CORE_0_AREA_DRAM0_1_RD_RLS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CORE_0_AREA_DRAM0_1_RD_RLS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_AREA_DRAM0_1_RD_RLS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_AREA_DRAM0_1_RD_RLS` writer - Core0 dram0 area1 read monitor interrupt enable"]
pub struct CORE_0_AREA_DRAM0_1_RD_RLS_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_AREA_DRAM0_1_RD_RLS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `CORE_0_AREA_DRAM0_1_WR_RLS` reader - Core0 dram0 area1 write monitor interrupt enable"]
pub struct CORE_0_AREA_DRAM0_1_WR_RLS_R(crate::FieldReader<bool, bool>);
impl CORE_0_AREA_DRAM0_1_WR_RLS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CORE_0_AREA_DRAM0_1_WR_RLS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_AREA_DRAM0_1_WR_RLS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_AREA_DRAM0_1_WR_RLS` writer - Core0 dram0 area1 write monitor interrupt enable"]
pub struct CORE_0_AREA_DRAM0_1_WR_RLS_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_AREA_DRAM0_1_WR_RLS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `CORE_0_AREA_PIF_0_RD_RLS` reader - Core0 PIF area0 read monitor interrupt enable"]
pub struct CORE_0_AREA_PIF_0_RD_RLS_R(crate::FieldReader<bool, bool>);
impl CORE_0_AREA_PIF_0_RD_RLS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CORE_0_AREA_PIF_0_RD_RLS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_AREA_PIF_0_RD_RLS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_AREA_PIF_0_RD_RLS` writer - Core0 PIF area0 read monitor interrupt enable"]
pub struct CORE_0_AREA_PIF_0_RD_RLS_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_AREA_PIF_0_RD_RLS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `CORE_0_AREA_PIF_0_WR_RLS` reader - Core0 PIF area0 write monitor interrupt enable"]
pub struct CORE_0_AREA_PIF_0_WR_RLS_R(crate::FieldReader<bool, bool>);
impl CORE_0_AREA_PIF_0_WR_RLS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CORE_0_AREA_PIF_0_WR_RLS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_AREA_PIF_0_WR_RLS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_AREA_PIF_0_WR_RLS` writer - Core0 PIF area0 write monitor interrupt enable"]
pub struct CORE_0_AREA_PIF_0_WR_RLS_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_AREA_PIF_0_WR_RLS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `CORE_0_AREA_PIF_1_RD_RLS` reader - Core0 PIF area1 read monitor interrupt enable"]
pub struct CORE_0_AREA_PIF_1_RD_RLS_R(crate::FieldReader<bool, bool>);
impl CORE_0_AREA_PIF_1_RD_RLS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CORE_0_AREA_PIF_1_RD_RLS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_AREA_PIF_1_RD_RLS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_AREA_PIF_1_RD_RLS` writer - Core0 PIF area1 read monitor interrupt enable"]
pub struct CORE_0_AREA_PIF_1_RD_RLS_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_AREA_PIF_1_RD_RLS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `CORE_0_AREA_PIF_1_WR_RLS` reader - Core0 PIF area1 write monitor interrupt enable"]
pub struct CORE_0_AREA_PIF_1_WR_RLS_R(crate::FieldReader<bool, bool>);
impl CORE_0_AREA_PIF_1_WR_RLS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CORE_0_AREA_PIF_1_WR_RLS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_AREA_PIF_1_WR_RLS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_AREA_PIF_1_WR_RLS` writer - Core0 PIF area1 write monitor interrupt enable"]
pub struct CORE_0_AREA_PIF_1_WR_RLS_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_AREA_PIF_1_WR_RLS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `CORE_0_SP_SPILL_MIN_RLS` reader - Core0 stackpoint overflow monitor interrupt enable"]
pub struct CORE_0_SP_SPILL_MIN_RLS_R(crate::FieldReader<bool, bool>);
impl CORE_0_SP_SPILL_MIN_RLS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CORE_0_SP_SPILL_MIN_RLS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_SP_SPILL_MIN_RLS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_SP_SPILL_MIN_RLS` writer - Core0 stackpoint overflow monitor interrupt enable"]
pub struct CORE_0_SP_SPILL_MIN_RLS_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_SP_SPILL_MIN_RLS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `CORE_0_SP_SPILL_MAX_RLS` reader - Core0 stackpoint underflow monitor interrupt enable"]
pub struct CORE_0_SP_SPILL_MAX_RLS_R(crate::FieldReader<bool, bool>);
impl CORE_0_SP_SPILL_MAX_RLS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CORE_0_SP_SPILL_MAX_RLS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_SP_SPILL_MAX_RLS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_SP_SPILL_MAX_RLS` writer - Core0 stackpoint underflow monitor interrupt enable"]
pub struct CORE_0_SP_SPILL_MAX_RLS_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_SP_SPILL_MAX_RLS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `CORE_0_IRAM0_EXCEPTION_MONITOR_RLS` reader - IBUS busy monitor interrupt enable"]
pub struct CORE_0_IRAM0_EXCEPTION_MONITOR_RLS_R(crate::FieldReader<bool, bool>);
impl CORE_0_IRAM0_EXCEPTION_MONITOR_RLS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CORE_0_IRAM0_EXCEPTION_MONITOR_RLS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_IRAM0_EXCEPTION_MONITOR_RLS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_IRAM0_EXCEPTION_MONITOR_RLS` writer - IBUS busy monitor interrupt enable"]
pub struct CORE_0_IRAM0_EXCEPTION_MONITOR_RLS_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_IRAM0_EXCEPTION_MONITOR_RLS_W<'a> {
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
#[doc = "Field `CORE_0_DRAM0_EXCEPTION_MONITOR_RLS` reader - DBUS busy monitor interrupt enbale"]
pub struct CORE_0_DRAM0_EXCEPTION_MONITOR_RLS_R(crate::FieldReader<bool, bool>);
impl CORE_0_DRAM0_EXCEPTION_MONITOR_RLS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CORE_0_DRAM0_EXCEPTION_MONITOR_RLS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_DRAM0_EXCEPTION_MONITOR_RLS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_DRAM0_EXCEPTION_MONITOR_RLS` writer - DBUS busy monitor interrupt enbale"]
pub struct CORE_0_DRAM0_EXCEPTION_MONITOR_RLS_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_DRAM0_EXCEPTION_MONITOR_RLS_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Core0 dram0 area0 read monitor interrupt enable"]
    #[inline(always)]
    pub fn core_0_area_dram0_0_rd_rls(&self) -> CORE_0_AREA_DRAM0_0_RD_RLS_R {
        CORE_0_AREA_DRAM0_0_RD_RLS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Core0 dram0 area0 write monitor interrupt enable"]
    #[inline(always)]
    pub fn core_0_area_dram0_0_wr_rls(&self) -> CORE_0_AREA_DRAM0_0_WR_RLS_R {
        CORE_0_AREA_DRAM0_0_WR_RLS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Core0 dram0 area1 read monitor interrupt enable"]
    #[inline(always)]
    pub fn core_0_area_dram0_1_rd_rls(&self) -> CORE_0_AREA_DRAM0_1_RD_RLS_R {
        CORE_0_AREA_DRAM0_1_RD_RLS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Core0 dram0 area1 write monitor interrupt enable"]
    #[inline(always)]
    pub fn core_0_area_dram0_1_wr_rls(&self) -> CORE_0_AREA_DRAM0_1_WR_RLS_R {
        CORE_0_AREA_DRAM0_1_WR_RLS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Core0 PIF area0 read monitor interrupt enable"]
    #[inline(always)]
    pub fn core_0_area_pif_0_rd_rls(&self) -> CORE_0_AREA_PIF_0_RD_RLS_R {
        CORE_0_AREA_PIF_0_RD_RLS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Core0 PIF area0 write monitor interrupt enable"]
    #[inline(always)]
    pub fn core_0_area_pif_0_wr_rls(&self) -> CORE_0_AREA_PIF_0_WR_RLS_R {
        CORE_0_AREA_PIF_0_WR_RLS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Core0 PIF area1 read monitor interrupt enable"]
    #[inline(always)]
    pub fn core_0_area_pif_1_rd_rls(&self) -> CORE_0_AREA_PIF_1_RD_RLS_R {
        CORE_0_AREA_PIF_1_RD_RLS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Core0 PIF area1 write monitor interrupt enable"]
    #[inline(always)]
    pub fn core_0_area_pif_1_wr_rls(&self) -> CORE_0_AREA_PIF_1_WR_RLS_R {
        CORE_0_AREA_PIF_1_WR_RLS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Core0 stackpoint overflow monitor interrupt enable"]
    #[inline(always)]
    pub fn core_0_sp_spill_min_rls(&self) -> CORE_0_SP_SPILL_MIN_RLS_R {
        CORE_0_SP_SPILL_MIN_RLS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Core0 stackpoint underflow monitor interrupt enable"]
    #[inline(always)]
    pub fn core_0_sp_spill_max_rls(&self) -> CORE_0_SP_SPILL_MAX_RLS_R {
        CORE_0_SP_SPILL_MAX_RLS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - IBUS busy monitor interrupt enable"]
    #[inline(always)]
    pub fn core_0_iram0_exception_monitor_rls(&self) -> CORE_0_IRAM0_EXCEPTION_MONITOR_RLS_R {
        CORE_0_IRAM0_EXCEPTION_MONITOR_RLS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - DBUS busy monitor interrupt enbale"]
    #[inline(always)]
    pub fn core_0_dram0_exception_monitor_rls(&self) -> CORE_0_DRAM0_EXCEPTION_MONITOR_RLS_R {
        CORE_0_DRAM0_EXCEPTION_MONITOR_RLS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Core0 dram0 area0 read monitor interrupt enable"]
    #[inline(always)]
    pub fn core_0_area_dram0_0_rd_rls(&mut self) -> CORE_0_AREA_DRAM0_0_RD_RLS_W {
        CORE_0_AREA_DRAM0_0_RD_RLS_W { w: self }
    }
    #[doc = "Bit 1 - Core0 dram0 area0 write monitor interrupt enable"]
    #[inline(always)]
    pub fn core_0_area_dram0_0_wr_rls(&mut self) -> CORE_0_AREA_DRAM0_0_WR_RLS_W {
        CORE_0_AREA_DRAM0_0_WR_RLS_W { w: self }
    }
    #[doc = "Bit 2 - Core0 dram0 area1 read monitor interrupt enable"]
    #[inline(always)]
    pub fn core_0_area_dram0_1_rd_rls(&mut self) -> CORE_0_AREA_DRAM0_1_RD_RLS_W {
        CORE_0_AREA_DRAM0_1_RD_RLS_W { w: self }
    }
    #[doc = "Bit 3 - Core0 dram0 area1 write monitor interrupt enable"]
    #[inline(always)]
    pub fn core_0_area_dram0_1_wr_rls(&mut self) -> CORE_0_AREA_DRAM0_1_WR_RLS_W {
        CORE_0_AREA_DRAM0_1_WR_RLS_W { w: self }
    }
    #[doc = "Bit 4 - Core0 PIF area0 read monitor interrupt enable"]
    #[inline(always)]
    pub fn core_0_area_pif_0_rd_rls(&mut self) -> CORE_0_AREA_PIF_0_RD_RLS_W {
        CORE_0_AREA_PIF_0_RD_RLS_W { w: self }
    }
    #[doc = "Bit 5 - Core0 PIF area0 write monitor interrupt enable"]
    #[inline(always)]
    pub fn core_0_area_pif_0_wr_rls(&mut self) -> CORE_0_AREA_PIF_0_WR_RLS_W {
        CORE_0_AREA_PIF_0_WR_RLS_W { w: self }
    }
    #[doc = "Bit 6 - Core0 PIF area1 read monitor interrupt enable"]
    #[inline(always)]
    pub fn core_0_area_pif_1_rd_rls(&mut self) -> CORE_0_AREA_PIF_1_RD_RLS_W {
        CORE_0_AREA_PIF_1_RD_RLS_W { w: self }
    }
    #[doc = "Bit 7 - Core0 PIF area1 write monitor interrupt enable"]
    #[inline(always)]
    pub fn core_0_area_pif_1_wr_rls(&mut self) -> CORE_0_AREA_PIF_1_WR_RLS_W {
        CORE_0_AREA_PIF_1_WR_RLS_W { w: self }
    }
    #[doc = "Bit 8 - Core0 stackpoint overflow monitor interrupt enable"]
    #[inline(always)]
    pub fn core_0_sp_spill_min_rls(&mut self) -> CORE_0_SP_SPILL_MIN_RLS_W {
        CORE_0_SP_SPILL_MIN_RLS_W { w: self }
    }
    #[doc = "Bit 9 - Core0 stackpoint underflow monitor interrupt enable"]
    #[inline(always)]
    pub fn core_0_sp_spill_max_rls(&mut self) -> CORE_0_SP_SPILL_MAX_RLS_W {
        CORE_0_SP_SPILL_MAX_RLS_W { w: self }
    }
    #[doc = "Bit 10 - IBUS busy monitor interrupt enable"]
    #[inline(always)]
    pub fn core_0_iram0_exception_monitor_rls(&mut self) -> CORE_0_IRAM0_EXCEPTION_MONITOR_RLS_W {
        CORE_0_IRAM0_EXCEPTION_MONITOR_RLS_W { w: self }
    }
    #[doc = "Bit 11 - DBUS busy monitor interrupt enbale"]
    #[inline(always)]
    pub fn core_0_dram0_exception_monitor_rls(&mut self) -> CORE_0_DRAM0_EXCEPTION_MONITOR_RLS_W {
        CORE_0_DRAM0_EXCEPTION_MONITOR_RLS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "core0 monitor interrupt enable register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_0_interrupt_rls]
(index.html) module"]
pub struct CORE_0_INTERRUPT_RLS_SPEC;
impl crate::RegisterSpec for CORE_0_INTERRUPT_RLS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_0_interrupt_rls::R]
(R) reader structure"]
impl crate::Readable for CORE_0_INTERRUPT_RLS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_0_interrupt_rls::W]
(W) writer structure"]
impl crate::Writable for CORE_0_INTERRUPT_RLS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CORE_0_INTERRUPT_RLS to value 0"]
impl crate::Resettable for CORE_0_INTERRUPT_RLS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}