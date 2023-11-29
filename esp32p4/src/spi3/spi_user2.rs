#[doc = "Register `SPI_USER2` reader"]
pub type R = crate::R<SPI_USER2_SPEC>;
#[doc = "Register `SPI_USER2` writer"]
pub type W = crate::W<SPI_USER2_SPEC>;
#[doc = "Field `SPI_USR_COMMAND_VALUE` reader - The value of command. Can be configured in CONF state."]
pub type SPI_USR_COMMAND_VALUE_R = crate::FieldReader<u16>;
#[doc = "Field `SPI_USR_COMMAND_VALUE` writer - The value of command. Can be configured in CONF state."]
pub type SPI_USR_COMMAND_VALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SPI_MST_REMPTY_ERR_END_EN` reader - 1: SPI transfer is ended when SPI TX AFIFO read empty error is valid in GP-SPI master FD/HD-mode. 0: SPI transfer is not ended when SPI TX AFIFO read empty error is valid in GP-SPI master FD/HD-mode."]
pub type SPI_MST_REMPTY_ERR_END_EN_R = crate::BitReader;
#[doc = "Field `SPI_MST_REMPTY_ERR_END_EN` writer - 1: SPI transfer is ended when SPI TX AFIFO read empty error is valid in GP-SPI master FD/HD-mode. 0: SPI transfer is not ended when SPI TX AFIFO read empty error is valid in GP-SPI master FD/HD-mode."]
pub type SPI_MST_REMPTY_ERR_END_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_USR_COMMAND_BITLEN` reader - The length in bits of command phase. The register value shall be (bit_num-1). Can be configured in CONF state."]
pub type SPI_USR_COMMAND_BITLEN_R = crate::FieldReader;
#[doc = "Field `SPI_USR_COMMAND_BITLEN` writer - The length in bits of command phase. The register value shall be (bit_num-1). Can be configured in CONF state."]
pub type SPI_USR_COMMAND_BITLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:15 - The value of command. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_usr_command_value(&self) -> SPI_USR_COMMAND_VALUE_R {
        SPI_USR_COMMAND_VALUE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 27 - 1: SPI transfer is ended when SPI TX AFIFO read empty error is valid in GP-SPI master FD/HD-mode. 0: SPI transfer is not ended when SPI TX AFIFO read empty error is valid in GP-SPI master FD/HD-mode."]
    #[inline(always)]
    pub fn spi_mst_rempty_err_end_en(&self) -> SPI_MST_REMPTY_ERR_END_EN_R {
        SPI_MST_REMPTY_ERR_END_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:31 - The length in bits of command phase. The register value shall be (bit_num-1). Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_usr_command_bitlen(&self) -> SPI_USR_COMMAND_BITLEN_R {
        SPI_USR_COMMAND_BITLEN_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_USER2")
            .field(
                "spi_usr_command_value",
                &format_args!("{}", self.spi_usr_command_value().bits()),
            )
            .field(
                "spi_mst_rempty_err_end_en",
                &format_args!("{}", self.spi_mst_rempty_err_end_en().bit()),
            )
            .field(
                "spi_usr_command_bitlen",
                &format_args!("{}", self.spi_usr_command_bitlen().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_USER2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:15 - The value of command. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_usr_command_value(&mut self) -> SPI_USR_COMMAND_VALUE_W<SPI_USER2_SPEC> {
        SPI_USR_COMMAND_VALUE_W::new(self, 0)
    }
    #[doc = "Bit 27 - 1: SPI transfer is ended when SPI TX AFIFO read empty error is valid in GP-SPI master FD/HD-mode. 0: SPI transfer is not ended when SPI TX AFIFO read empty error is valid in GP-SPI master FD/HD-mode."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mst_rempty_err_end_en(&mut self) -> SPI_MST_REMPTY_ERR_END_EN_W<SPI_USER2_SPEC> {
        SPI_MST_REMPTY_ERR_END_EN_W::new(self, 27)
    }
    #[doc = "Bits 28:31 - The length in bits of command phase. The register value shall be (bit_num-1). Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_usr_command_bitlen(&mut self) -> SPI_USR_COMMAND_BITLEN_W<SPI_USER2_SPEC> {
        SPI_USR_COMMAND_BITLEN_W::new(self, 28)
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
#[doc = "SPI USER control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_user2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_user2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_USER2_SPEC;
impl crate::RegisterSpec for SPI_USER2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_user2::R`](R) reader structure"]
impl crate::Readable for SPI_USER2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_user2::W`](W) writer structure"]
impl crate::Writable for SPI_USER2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_USER2 to value 0x7800_0000"]
impl crate::Resettable for SPI_USER2_SPEC {
    const RESET_VALUE: Self::Ux = 0x7800_0000;
}