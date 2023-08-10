#[doc = "Register `CH%sADDR` reader"]
pub struct R(crate::R<CHADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `APB_MEM_ADDR` reader - The ram relative address in channel0 by apb fifo access"]
pub type APB_MEM_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The ram relative address in channel0 by apb fifo access"]
    #[inline(always)]
    pub fn apb_mem_addr(&self) -> APB_MEM_ADDR_R {
        APB_MEM_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHADDR")
            .field(
                "apb_mem_addr",
                &format_args!("{}", self.apb_mem_addr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CHADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chaddr](index.html) module"]
pub struct CHADDR_SPEC;
impl crate::RegisterSpec for CHADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chaddr::R](R) reader structure"]
impl crate::Readable for CHADDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CH%sADDR to value 0"]
impl crate::Resettable for CHADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}