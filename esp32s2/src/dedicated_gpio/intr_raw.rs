#[doc = "Register `INTR_RAW` reader"]
pub struct R(crate::R<INTR_RAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_RAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_RAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_RAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `GPIO0` reader - This interrupt raw bit turns to high level when dedicated GPIO0 has level/edge change configured by DEDIC_GPIO_INTR_RCGN_REG."]
pub struct GPIO0_R(crate::FieldReader<bool, bool>);
impl GPIO0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO1` reader - This interrupt raw bit turns to high level when dedicated GPIO1 has level/edge change configured by DEDIC_GPIO_INTR_RCGN_REG."]
pub struct GPIO1_R(crate::FieldReader<bool, bool>);
impl GPIO1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO2` reader - This interrupt raw bit turns to high level when dedicated GPIO2 has level/edge change configured by DEDIC_GPIO_INTR_RCGN_REG."]
pub struct GPIO2_R(crate::FieldReader<bool, bool>);
impl GPIO2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO3` reader - This interrupt raw bit turns to high level when dedicated GPIO3 has level/edge change configured by DEDIC_GPIO_INTR_RCGN_REG."]
pub struct GPIO3_R(crate::FieldReader<bool, bool>);
impl GPIO3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO4` reader - This interrupt raw bit turns to high level when dedicated GPIO4 has level/edge change configured by DEDIC_GPIO_INTR_RCGN_REG."]
pub struct GPIO4_R(crate::FieldReader<bool, bool>);
impl GPIO4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO5` reader - This interrupt raw bit turns to high level when dedicated GPIO5 has level/edge change configured by DEDIC_GPIO_INTR_RCGN_REG."]
pub struct GPIO5_R(crate::FieldReader<bool, bool>);
impl GPIO5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO6` reader - This interrupt raw bit turns to high level when dedicated GPIO6 has level/edge change configured by DEDIC_GPIO_INTR_RCGN_REG."]
pub struct GPIO6_R(crate::FieldReader<bool, bool>);
impl GPIO6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO7` reader - This interrupt raw bit turns to high level when dedicated GPIO7 has level/edge change configured by DEDIC_GPIO_INTR_RCGN_REG."]
pub struct GPIO7_R(crate::FieldReader<bool, bool>);
impl GPIO7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - This interrupt raw bit turns to high level when dedicated GPIO0 has level/edge change configured by DEDIC_GPIO_INTR_RCGN_REG."]
    #[inline(always)]
    pub fn gpio0(&self) -> GPIO0_R {
        GPIO0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - This interrupt raw bit turns to high level when dedicated GPIO1 has level/edge change configured by DEDIC_GPIO_INTR_RCGN_REG."]
    #[inline(always)]
    pub fn gpio1(&self) -> GPIO1_R {
        GPIO1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - This interrupt raw bit turns to high level when dedicated GPIO2 has level/edge change configured by DEDIC_GPIO_INTR_RCGN_REG."]
    #[inline(always)]
    pub fn gpio2(&self) -> GPIO2_R {
        GPIO2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - This interrupt raw bit turns to high level when dedicated GPIO3 has level/edge change configured by DEDIC_GPIO_INTR_RCGN_REG."]
    #[inline(always)]
    pub fn gpio3(&self) -> GPIO3_R {
        GPIO3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - This interrupt raw bit turns to high level when dedicated GPIO4 has level/edge change configured by DEDIC_GPIO_INTR_RCGN_REG."]
    #[inline(always)]
    pub fn gpio4(&self) -> GPIO4_R {
        GPIO4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - This interrupt raw bit turns to high level when dedicated GPIO5 has level/edge change configured by DEDIC_GPIO_INTR_RCGN_REG."]
    #[inline(always)]
    pub fn gpio5(&self) -> GPIO5_R {
        GPIO5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - This interrupt raw bit turns to high level when dedicated GPIO6 has level/edge change configured by DEDIC_GPIO_INTR_RCGN_REG."]
    #[inline(always)]
    pub fn gpio6(&self) -> GPIO6_R {
        GPIO6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - This interrupt raw bit turns to high level when dedicated GPIO7 has level/edge change configured by DEDIC_GPIO_INTR_RCGN_REG."]
    #[inline(always)]
    pub fn gpio7(&self) -> GPIO7_R {
        GPIO7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
#[doc = "Raw interrupt status\n\nThis register you can [`read`]
(crate::generic::Reg::read). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_raw]
(index.html) module"]
pub struct INTR_RAW_SPEC;
impl crate::RegisterSpec for INTR_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_raw::R]
(R) reader structure"]
impl crate::Readable for INTR_RAW_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTR_RAW to value 0"]
impl crate::Resettable for INTR_RAW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}