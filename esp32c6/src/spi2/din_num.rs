#[doc = "Register `DIN_NUM` reader"]
pub struct R(crate::R<DIN_NUM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIN_NUM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIN_NUM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIN_NUM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIN_NUM` writer"]
pub struct W(crate::W<DIN_NUM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIN_NUM_SPEC>;
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
impl From<crate::W<DIN_NUM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIN_NUM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIN0_NUM` reader - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type DIN0_NUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIN0_NUM` writer - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type DIN0_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DIN_NUM_SPEC, u8, u8, 2, O>;
#[doc = "Field `DIN1_NUM` reader - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type DIN1_NUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIN1_NUM` writer - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type DIN1_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DIN_NUM_SPEC, u8, u8, 2, O>;
#[doc = "Field `DIN2_NUM` reader - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type DIN2_NUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIN2_NUM` writer - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type DIN2_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DIN_NUM_SPEC, u8, u8, 2, O>;
#[doc = "Field `DIN3_NUM` reader - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type DIN3_NUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIN3_NUM` writer - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type DIN3_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DIN_NUM_SPEC, u8, u8, 2, O>;
#[doc = "Field `DIN4_NUM` reader - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type DIN4_NUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIN5_NUM` reader - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type DIN5_NUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIN6_NUM` reader - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type DIN6_NUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIN7_NUM` reader - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type DIN7_NUM_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:1 - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn din0_num(&self) -> DIN0_NUM_R {
        DIN0_NUM_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn din1_num(&self) -> DIN1_NUM_R {
        DIN1_NUM_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn din2_num(&self) -> DIN2_NUM_R {
        DIN2_NUM_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn din3_num(&self) -> DIN3_NUM_R {
        DIN3_NUM_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn din4_num(&self) -> DIN4_NUM_R {
        DIN4_NUM_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn din5_num(&self) -> DIN5_NUM_R {
        DIN5_NUM_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn din6_num(&self) -> DIN6_NUM_R {
        DIN6_NUM_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn din7_num(&self) -> DIN7_NUM_R {
        DIN7_NUM_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn din0_num(&mut self) -> DIN0_NUM_W<0> {
        DIN0_NUM_W::new(self)
    }
    #[doc = "Bits 2:3 - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn din1_num(&mut self) -> DIN1_NUM_W<2> {
        DIN1_NUM_W::new(self)
    }
    #[doc = "Bits 4:5 - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn din2_num(&mut self) -> DIN2_NUM_W<4> {
        DIN2_NUM_W::new(self)
    }
    #[doc = "Bits 6:7 - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn din3_num(&mut self) -> DIN3_NUM_W<6> {
        DIN3_NUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI input delay number configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [din_num](index.html) module"]
pub struct DIN_NUM_SPEC;
impl crate::RegisterSpec for DIN_NUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [din_num::R](R) reader structure"]
impl crate::Readable for DIN_NUM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [din_num::W](W) writer structure"]
impl crate::Writable for DIN_NUM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIN_NUM to value 0"]
impl crate::Resettable for DIN_NUM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}