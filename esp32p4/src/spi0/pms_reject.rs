#[doc = "Register `PMS_REJECT` reader"]
pub type R = crate::R<PMS_REJECT_SPEC>;
#[doc = "Register `PMS_REJECT` writer"]
pub type W = crate::W<PMS_REJECT_SPEC>;
#[doc = "Field `REJECT_ADDR` reader - This bits show the first SPI1 access error address. It is cleared by when SPI_MEM_PMS_REJECT_INT_CLR bit is set."]
pub type REJECT_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `PM_EN` reader - Set this bit to enable SPI0/1 transfer permission control function."]
pub type PM_EN_R = crate::BitReader;
#[doc = "Field `PM_EN` writer - Set this bit to enable SPI0/1 transfer permission control function."]
pub type PM_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMS_LD` reader - 1: SPI1 write access error. 0: No write access error. It is cleared by when SPI_MEM_PMS_REJECT_INT_CLR bit is set."]
pub type PMS_LD_R = crate::BitReader;
#[doc = "Field `PMS_ST` reader - 1: SPI1 read access error. 0: No read access error. It is cleared by when SPI_MEM_PMS_REJECT_INT_CLR bit is set."]
pub type PMS_ST_R = crate::BitReader;
#[doc = "Field `PMS_MULTI_HIT` reader - 1: SPI1 access is rejected because of address miss. 0: No address miss error. It is cleared by when SPI_MEM_PMS_REJECT_INT_CLR bit is set."]
pub type PMS_MULTI_HIT_R = crate::BitReader;
#[doc = "Field `PMS_IVD` reader - 1: SPI1 access is rejected because of address multi-hit. 0: No address multi-hit error. It is cleared by when SPI_MEM_PMS_REJECT_INT_CLR bit is set."]
pub type PMS_IVD_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:26 - This bits show the first SPI1 access error address. It is cleared by when SPI_MEM_PMS_REJECT_INT_CLR bit is set."]
    #[inline(always)]
    pub fn reject_addr(&self) -> REJECT_ADDR_R {
        REJECT_ADDR_R::new(self.bits & 0x07ff_ffff)
    }
    #[doc = "Bit 27 - Set this bit to enable SPI0/1 transfer permission control function."]
    #[inline(always)]
    pub fn pm_en(&self) -> PM_EN_R {
        PM_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 1: SPI1 write access error. 0: No write access error. It is cleared by when SPI_MEM_PMS_REJECT_INT_CLR bit is set."]
    #[inline(always)]
    pub fn pms_ld(&self) -> PMS_LD_R {
        PMS_LD_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 1: SPI1 read access error. 0: No read access error. It is cleared by when SPI_MEM_PMS_REJECT_INT_CLR bit is set."]
    #[inline(always)]
    pub fn pms_st(&self) -> PMS_ST_R {
        PMS_ST_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 1: SPI1 access is rejected because of address miss. 0: No address miss error. It is cleared by when SPI_MEM_PMS_REJECT_INT_CLR bit is set."]
    #[inline(always)]
    pub fn pms_multi_hit(&self) -> PMS_MULTI_HIT_R {
        PMS_MULTI_HIT_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 1: SPI1 access is rejected because of address multi-hit. 0: No address multi-hit error. It is cleared by when SPI_MEM_PMS_REJECT_INT_CLR bit is set."]
    #[inline(always)]
    pub fn pms_ivd(&self) -> PMS_IVD_R {
        PMS_IVD_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PMS_REJECT")
            .field(
                "reject_addr",
                &format_args!("{}", self.reject_addr().bits()),
            )
            .field("pm_en", &format_args!("{}", self.pm_en().bit()))
            .field("pms_ld", &format_args!("{}", self.pms_ld().bit()))
            .field("pms_st", &format_args!("{}", self.pms_st().bit()))
            .field(
                "pms_multi_hit",
                &format_args!("{}", self.pms_multi_hit().bit()),
            )
            .field("pms_ivd", &format_args!("{}", self.pms_ivd().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PMS_REJECT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 27 - Set this bit to enable SPI0/1 transfer permission control function."]
    #[inline(always)]
    #[must_use]
    pub fn pm_en(&mut self) -> PM_EN_W<PMS_REJECT_SPEC> {
        PM_EN_W::new(self, 27)
    }
}
#[doc = "SPI1 access reject register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pms_reject::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pms_reject::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMS_REJECT_SPEC;
impl crate::RegisterSpec for PMS_REJECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pms_reject::R`](R) reader structure"]
impl crate::Readable for PMS_REJECT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pms_reject::W`](W) writer structure"]
impl crate::Writable for PMS_REJECT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMS_REJECT to value 0"]
impl crate::Resettable for PMS_REJECT_SPEC {
    const RESET_VALUE: u32 = 0;
}