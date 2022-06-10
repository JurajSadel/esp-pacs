#[doc = "Register `UPDATE` reader"]
pub struct R(crate::R<UPDATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UPDATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UPDATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UPDATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UPDATE` writer"]
pub struct W(crate::W<UPDATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UPDATE_SPEC>;
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
impl From<crate::W<UPDATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UPDATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER_VALUE_VALID` reader - Check if it is valid to read out timer value from registers. 0: Not ready to read timer value from registers; 1: Ready to read timer value from registers"]
pub type TIMER_VALUE_VALID_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_UPDATE` writer - Update system timer value to registers."]
pub type TIMER_UPDATE_W<'a> = crate::BitWriter<'a, u32, UPDATE_SPEC, bool, 31>;
impl R {
    #[doc = "Bit 30 - Check if it is valid to read out timer value from registers. 0: Not ready to read timer value from registers; 1: Ready to read timer value from registers"]
    #[inline(always)]
    pub fn timer_value_valid(&self) -> TIMER_VALUE_VALID_R {
        TIMER_VALUE_VALID_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Update system timer value to registers."]
    #[inline(always)]
    pub fn timer_update(&mut self) -> TIMER_UPDATE_W {
        TIMER_UPDATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Read out system timer value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [update](index.html) module"]
pub struct UPDATE_SPEC;
impl crate::RegisterSpec for UPDATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [update::R](R) reader structure"]
impl crate::Readable for UPDATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [update::W](W) writer structure"]
impl crate::Writable for UPDATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UPDATE to value 0"]
impl crate::Resettable for UPDATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
