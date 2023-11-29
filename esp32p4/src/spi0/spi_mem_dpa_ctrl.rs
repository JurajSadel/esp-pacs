#[doc = "Register `SPI_MEM_DPA_CTRL` reader"]
pub type R = crate::R<SPI_MEM_DPA_CTRL_SPEC>;
#[doc = "Register `SPI_MEM_DPA_CTRL` writer"]
pub type W = crate::W<SPI_MEM_DPA_CTRL_SPEC>;
#[doc = "Field `SPI_CRYPT_SECURITY_LEVEL` reader - Set the security level of spi mem cryption. 0: Shut off cryption DPA funtion. 1-7: The bigger the number is, the more secure the cryption is. (Note that the performance of cryption will decrease together with this number increasing)"]
pub type SPI_CRYPT_SECURITY_LEVEL_R = crate::FieldReader;
#[doc = "Field `SPI_CRYPT_SECURITY_LEVEL` writer - Set the security level of spi mem cryption. 0: Shut off cryption DPA funtion. 1-7: The bigger the number is, the more secure the cryption is. (Note that the performance of cryption will decrease together with this number increasing)"]
pub type SPI_CRYPT_SECURITY_LEVEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SPI_CRYPT_CALC_D_DPA_EN` reader - Only available when SPI_CRYPT_SECURITY_LEVEL is not 0. 1: Enable DPA in the calculation that using key 1 or key 2. 0: Enable DPA only in the calculation that using key 1."]
pub type SPI_CRYPT_CALC_D_DPA_EN_R = crate::BitReader;
#[doc = "Field `SPI_CRYPT_CALC_D_DPA_EN` writer - Only available when SPI_CRYPT_SECURITY_LEVEL is not 0. 1: Enable DPA in the calculation that using key 1 or key 2. 0: Enable DPA only in the calculation that using key 1."]
pub type SPI_CRYPT_CALC_D_DPA_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_CRYPT_DPA_SELECT_REGISTER` reader - 1: MSPI XTS DPA clock gate is controlled by SPI_CRYPT_CALC_D_DPA_EN and SPI_CRYPT_SECURITY_LEVEL. 0: Controlled by efuse bits."]
pub type SPI_CRYPT_DPA_SELECT_REGISTER_R = crate::BitReader;
#[doc = "Field `SPI_CRYPT_DPA_SELECT_REGISTER` writer - 1: MSPI XTS DPA clock gate is controlled by SPI_CRYPT_CALC_D_DPA_EN and SPI_CRYPT_SECURITY_LEVEL. 0: Controlled by efuse bits."]
pub type SPI_CRYPT_DPA_SELECT_REGISTER_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Set the security level of spi mem cryption. 0: Shut off cryption DPA funtion. 1-7: The bigger the number is, the more secure the cryption is. (Note that the performance of cryption will decrease together with this number increasing)"]
    #[inline(always)]
    pub fn spi_crypt_security_level(&self) -> SPI_CRYPT_SECURITY_LEVEL_R {
        SPI_CRYPT_SECURITY_LEVEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Only available when SPI_CRYPT_SECURITY_LEVEL is not 0. 1: Enable DPA in the calculation that using key 1 or key 2. 0: Enable DPA only in the calculation that using key 1."]
    #[inline(always)]
    pub fn spi_crypt_calc_d_dpa_en(&self) -> SPI_CRYPT_CALC_D_DPA_EN_R {
        SPI_CRYPT_CALC_D_DPA_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1: MSPI XTS DPA clock gate is controlled by SPI_CRYPT_CALC_D_DPA_EN and SPI_CRYPT_SECURITY_LEVEL. 0: Controlled by efuse bits."]
    #[inline(always)]
    pub fn spi_crypt_dpa_select_register(&self) -> SPI_CRYPT_DPA_SELECT_REGISTER_R {
        SPI_CRYPT_DPA_SELECT_REGISTER_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_DPA_CTRL")
            .field(
                "spi_crypt_security_level",
                &format_args!("{}", self.spi_crypt_security_level().bits()),
            )
            .field(
                "spi_crypt_calc_d_dpa_en",
                &format_args!("{}", self.spi_crypt_calc_d_dpa_en().bit()),
            )
            .field(
                "spi_crypt_dpa_select_register",
                &format_args!("{}", self.spi_crypt_dpa_select_register().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_MEM_DPA_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:2 - Set the security level of spi mem cryption. 0: Shut off cryption DPA funtion. 1-7: The bigger the number is, the more secure the cryption is. (Note that the performance of cryption will decrease together with this number increasing)"]
    #[inline(always)]
    #[must_use]
    pub fn spi_crypt_security_level(
        &mut self,
    ) -> SPI_CRYPT_SECURITY_LEVEL_W<SPI_MEM_DPA_CTRL_SPEC> {
        SPI_CRYPT_SECURITY_LEVEL_W::new(self, 0)
    }
    #[doc = "Bit 3 - Only available when SPI_CRYPT_SECURITY_LEVEL is not 0. 1: Enable DPA in the calculation that using key 1 or key 2. 0: Enable DPA only in the calculation that using key 1."]
    #[inline(always)]
    #[must_use]
    pub fn spi_crypt_calc_d_dpa_en(&mut self) -> SPI_CRYPT_CALC_D_DPA_EN_W<SPI_MEM_DPA_CTRL_SPEC> {
        SPI_CRYPT_CALC_D_DPA_EN_W::new(self, 3)
    }
    #[doc = "Bit 4 - 1: MSPI XTS DPA clock gate is controlled by SPI_CRYPT_CALC_D_DPA_EN and SPI_CRYPT_SECURITY_LEVEL. 0: Controlled by efuse bits."]
    #[inline(always)]
    #[must_use]
    pub fn spi_crypt_dpa_select_register(
        &mut self,
    ) -> SPI_CRYPT_DPA_SELECT_REGISTER_W<SPI_MEM_DPA_CTRL_SPEC> {
        SPI_CRYPT_DPA_SELECT_REGISTER_W::new(self, 4)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SPI memory cryption DPA register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_dpa_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_dpa_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_MEM_DPA_CTRL_SPEC;
impl crate::RegisterSpec for SPI_MEM_DPA_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_mem_dpa_ctrl::R`](R) reader structure"]
impl crate::Readable for SPI_MEM_DPA_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_mem_dpa_ctrl::W`](W) writer structure"]
impl crate::Writable for SPI_MEM_DPA_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_MEM_DPA_CTRL to value 0x0f"]
impl crate::Resettable for SPI_MEM_DPA_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f;
}