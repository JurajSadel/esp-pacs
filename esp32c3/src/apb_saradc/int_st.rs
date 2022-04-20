#[doc = "Register `INT_ST` reader"]
pub struct R(crate::R<INT_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `APB_SARADC_THRES1_LOW_INT_ST` reader - saradc thres1 low interrupt state"]
pub struct APB_SARADC_THRES1_LOW_INT_ST_R(crate::FieldReader<bool, bool>);
impl APB_SARADC_THRES1_LOW_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APB_SARADC_THRES1_LOW_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB_SARADC_THRES1_LOW_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APB_SARADC_THRES0_LOW_INT_ST` reader - saradc thres0 low interrupt state"]
pub struct APB_SARADC_THRES0_LOW_INT_ST_R(crate::FieldReader<bool, bool>);
impl APB_SARADC_THRES0_LOW_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APB_SARADC_THRES0_LOW_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB_SARADC_THRES0_LOW_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APB_SARADC_THRES1_HIGH_INT_ST` reader - saradc thres1 high interrupt state"]
pub struct APB_SARADC_THRES1_HIGH_INT_ST_R(crate::FieldReader<bool, bool>);
impl APB_SARADC_THRES1_HIGH_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APB_SARADC_THRES1_HIGH_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB_SARADC_THRES1_HIGH_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APB_SARADC_THRES0_HIGH_INT_ST` reader - saradc thres0 high interrupt state"]
pub struct APB_SARADC_THRES0_HIGH_INT_ST_R(crate::FieldReader<bool, bool>);
impl APB_SARADC_THRES0_HIGH_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APB_SARADC_THRES0_HIGH_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB_SARADC_THRES0_HIGH_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APB_SARADC2_DONE_INT_ST` reader - saradc2 done interrupt state"]
pub struct APB_SARADC2_DONE_INT_ST_R(crate::FieldReader<bool, bool>);
impl APB_SARADC2_DONE_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APB_SARADC2_DONE_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB_SARADC2_DONE_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APB_SARADC1_DONE_INT_ST` reader - saradc1 done interrupt state"]
pub struct APB_SARADC1_DONE_INT_ST_R(crate::FieldReader<bool, bool>);
impl APB_SARADC1_DONE_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APB_SARADC1_DONE_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB_SARADC1_DONE_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 26 - saradc thres1 low interrupt state"]
    #[inline(always)]
    pub fn apb_saradc_thres1_low_int_st(&self) -> APB_SARADC_THRES1_LOW_INT_ST_R {
        APB_SARADC_THRES1_LOW_INT_ST_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - saradc thres0 low interrupt state"]
    #[inline(always)]
    pub fn apb_saradc_thres0_low_int_st(&self) -> APB_SARADC_THRES0_LOW_INT_ST_R {
        APB_SARADC_THRES0_LOW_INT_ST_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - saradc thres1 high interrupt state"]
    #[inline(always)]
    pub fn apb_saradc_thres1_high_int_st(&self) -> APB_SARADC_THRES1_HIGH_INT_ST_R {
        APB_SARADC_THRES1_HIGH_INT_ST_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - saradc thres0 high interrupt state"]
    #[inline(always)]
    pub fn apb_saradc_thres0_high_int_st(&self) -> APB_SARADC_THRES0_HIGH_INT_ST_R {
        APB_SARADC_THRES0_HIGH_INT_ST_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - saradc2 done interrupt state"]
    #[inline(always)]
    pub fn apb_saradc2_done_int_st(&self) -> APB_SARADC2_DONE_INT_ST_R {
        APB_SARADC2_DONE_INT_ST_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - saradc1 done interrupt state"]
    #[inline(always)]
    pub fn apb_saradc1_done_int_st(&self) -> APB_SARADC1_DONE_INT_ST_R {
        APB_SARADC1_DONE_INT_ST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "digital saradc int register\n\nThis register you can [`read`]
(crate::generic::Reg::read). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_st]
(index.html) module"]
pub struct INT_ST_SPEC;
impl crate::RegisterSpec for INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_st::R]
(R) reader structure"]
impl crate::Readable for INT_ST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for INT_ST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}