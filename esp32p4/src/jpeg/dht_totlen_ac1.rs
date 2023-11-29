#[doc = "Register `DHT_TOTLEN_AC1` reader"]
pub type R = crate::R<DHT_TOTLEN_AC1_SPEC>;
#[doc = "Field `DHT_TOTLEN_AC1` reader - write the numbers of 1~n codeword length sum from 1~16 of ac1 table"]
pub type DHT_TOTLEN_AC1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - write the numbers of 1~n codeword length sum from 1~16 of ac1 table"]
    #[inline(always)]
    pub fn dht_totlen_ac1(&self) -> DHT_TOTLEN_AC1_R {
        DHT_TOTLEN_AC1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHT_TOTLEN_AC1")
            .field(
                "dht_totlen_ac1",
                &format_args!("{}", self.dht_totlen_ac1().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DHT_TOTLEN_AC1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Trace and Debug registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dht_totlen_ac1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DHT_TOTLEN_AC1_SPEC;
impl crate::RegisterSpec for DHT_TOTLEN_AC1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dht_totlen_ac1::R`](R) reader structure"]
impl crate::Readable for DHT_TOTLEN_AC1_SPEC {}
#[doc = "`reset()` method sets DHT_TOTLEN_AC1 to value 0"]
impl crate::Resettable for DHT_TOTLEN_AC1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}