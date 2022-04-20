#[doc = "Register `SAR_COCPU_INT_ST` reader"]
pub struct R(crate::R<SAR_COCPU_INT_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_COCPU_INT_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_COCPU_INT_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_COCPU_INT_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COCPU_TOUCH_DONE_INT_ST` reader - TOUCH_DONE_INT interrupt status bit"]
pub struct COCPU_TOUCH_DONE_INT_ST_R(crate::FieldReader<bool, bool>);
impl COCPU_TOUCH_DONE_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COCPU_TOUCH_DONE_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COCPU_TOUCH_DONE_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COCPU_TOUCH_INACTIVE_INT_ST` reader - TOUCH_INACTIVE_INT interrupt status bit"]
pub struct COCPU_TOUCH_INACTIVE_INT_ST_R(crate::FieldReader<bool, bool>);
impl COCPU_TOUCH_INACTIVE_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COCPU_TOUCH_INACTIVE_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COCPU_TOUCH_INACTIVE_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COCPU_TOUCH_ACTIVE_INT_ST` reader - TOUCH_ACTIVE_INT interrupt status bit"]
pub struct COCPU_TOUCH_ACTIVE_INT_ST_R(crate::FieldReader<bool, bool>);
impl COCPU_TOUCH_ACTIVE_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COCPU_TOUCH_ACTIVE_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COCPU_TOUCH_ACTIVE_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COCPU_SARADC1_INT_ST` reader - SARADC1_DONE_INT interrupt status bit"]
pub struct COCPU_SARADC1_INT_ST_R(crate::FieldReader<bool, bool>);
impl COCPU_SARADC1_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COCPU_SARADC1_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COCPU_SARADC1_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COCPU_SARADC2_INT_ST` reader - SARADC2_DONE_INT interrupt status bit"]
pub struct COCPU_SARADC2_INT_ST_R(crate::FieldReader<bool, bool>);
impl COCPU_SARADC2_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COCPU_SARADC2_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COCPU_SARADC2_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COCPU_TSENS_INT_ST` reader - TSENS_DONE_INT interrupt status bit"]
pub struct COCPU_TSENS_INT_ST_R(crate::FieldReader<bool, bool>);
impl COCPU_TSENS_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COCPU_TSENS_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COCPU_TSENS_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COCPU_START_INT_ST` reader - RISCV_START_INT interrupt status bit"]
pub struct COCPU_START_INT_ST_R(crate::FieldReader<bool, bool>);
impl COCPU_START_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COCPU_START_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COCPU_START_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COCPU_SW_INT_ST` reader - SW_INT interrupt status bit"]
pub struct COCPU_SW_INT_ST_R(crate::FieldReader<bool, bool>);
impl COCPU_SW_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COCPU_SW_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COCPU_SW_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COCPU_SWD_INT_ST` reader - SWD_INT interrupt status bit"]
pub struct COCPU_SWD_INT_ST_R(crate::FieldReader<bool, bool>);
impl COCPU_SWD_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COCPU_SWD_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COCPU_SWD_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - TOUCH_DONE_INT interrupt status bit"]
    #[inline(always)]
    pub fn cocpu_touch_done_int_st(&self) -> COCPU_TOUCH_DONE_INT_ST_R {
        COCPU_TOUCH_DONE_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TOUCH_INACTIVE_INT interrupt status bit"]
    #[inline(always)]
    pub fn cocpu_touch_inactive_int_st(&self) -> COCPU_TOUCH_INACTIVE_INT_ST_R {
        COCPU_TOUCH_INACTIVE_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TOUCH_ACTIVE_INT interrupt status bit"]
    #[inline(always)]
    pub fn cocpu_touch_active_int_st(&self) -> COCPU_TOUCH_ACTIVE_INT_ST_R {
        COCPU_TOUCH_ACTIVE_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SARADC1_DONE_INT interrupt status bit"]
    #[inline(always)]
    pub fn cocpu_saradc1_int_st(&self) -> COCPU_SARADC1_INT_ST_R {
        COCPU_SARADC1_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SARADC2_DONE_INT interrupt status bit"]
    #[inline(always)]
    pub fn cocpu_saradc2_int_st(&self) -> COCPU_SARADC2_INT_ST_R {
        COCPU_SARADC2_INT_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TSENS_DONE_INT interrupt status bit"]
    #[inline(always)]
    pub fn cocpu_tsens_int_st(&self) -> COCPU_TSENS_INT_ST_R {
        COCPU_TSENS_INT_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RISCV_START_INT interrupt status bit"]
    #[inline(always)]
    pub fn cocpu_start_int_st(&self) -> COCPU_START_INT_ST_R {
        COCPU_START_INT_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SW_INT interrupt status bit"]
    #[inline(always)]
    pub fn cocpu_sw_int_st(&self) -> COCPU_SW_INT_ST_R {
        COCPU_SW_INT_ST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SWD_INT interrupt status bit"]
    #[inline(always)]
    pub fn cocpu_swd_int_st(&self) -> COCPU_SWD_INT_ST_R {
        COCPU_SWD_INT_ST_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "Interrupt status bit of ULP-RISCV\n\nThis register you can [`read`]
(crate::generic::Reg::read). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_cocpu_int_st]
(index.html) module"]
pub struct SAR_COCPU_INT_ST_SPEC;
impl crate::RegisterSpec for SAR_COCPU_INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_cocpu_int_st::R]
(R) reader structure"]
impl crate::Readable for SAR_COCPU_INT_ST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SAR_COCPU_INT_ST to value 0"]
impl crate::Resettable for SAR_COCPU_INT_ST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}