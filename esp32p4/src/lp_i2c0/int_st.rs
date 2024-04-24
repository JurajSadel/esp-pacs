#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<INT_ST_SPEC>;
#[doc = "Field `RXFIFO_WM` reader - The masked interrupt status status of I2C_RXFIFO_WM_INT interrupt."]
pub type RXFIFO_WM_R = crate::BitReader;
#[doc = "Field `TXFIFO_WM` reader - The masked interrupt status status of I2C_TXFIFO_WM_INT interrupt."]
pub type TXFIFO_WM_R = crate::BitReader;
#[doc = "Field `RXFIFO_OVF` reader - The masked interrupt status status of I2C_RXFIFO_OVF_INT interrupt."]
pub type RXFIFO_OVF_R = crate::BitReader;
#[doc = "Field `END_DETECT` reader - The masked interrupt status status of the I2C_END_DETECT_INT interrupt."]
pub type END_DETECT_R = crate::BitReader;
#[doc = "Field `BYTE_TRANS_DONE` reader - The masked interrupt status status of the I2C_END_DETECT_INT interrupt."]
pub type BYTE_TRANS_DONE_R = crate::BitReader;
#[doc = "Field `ARBITRATION_LOST` reader - The masked interrupt status status of the I2C_ARBITRATION_LOST_INT interrupt."]
pub type ARBITRATION_LOST_R = crate::BitReader;
#[doc = "Field `MST_TXFIFO_UDF` reader - The masked interrupt status status of I2C_TRANS_COMPLETE_INT interrupt."]
pub type MST_TXFIFO_UDF_R = crate::BitReader;
#[doc = "Field `TRANS_COMPLETE` reader - The masked interrupt status status of the I2C_TRANS_COMPLETE_INT interrupt."]
pub type TRANS_COMPLETE_R = crate::BitReader;
#[doc = "Field `TIME_OUT` reader - The masked interrupt status status of the I2C_TIME_OUT_INT interrupt."]
pub type TIME_OUT_R = crate::BitReader;
#[doc = "Field `TRANS_START` reader - The masked interrupt status status of the I2C_TRANS_START_INT interrupt."]
pub type TRANS_START_R = crate::BitReader;
#[doc = "Field `NACK` reader - The masked interrupt status status of I2C_SLAVE_STRETCH_INT interrupt."]
pub type NACK_R = crate::BitReader;
#[doc = "Field `TXFIFO_OVF` reader - The masked interrupt status status of I2C_TXFIFO_OVF_INT interrupt."]
pub type TXFIFO_OVF_R = crate::BitReader;
#[doc = "Field `RXFIFO_UDF` reader - The masked interrupt status status of I2C_RXFIFO_UDF_INT interrupt."]
pub type RXFIFO_UDF_R = crate::BitReader;
#[doc = "Field `SCL_ST_TO` reader - The masked interrupt status status of I2C_SCL_ST_TO_INT interrupt."]
pub type SCL_ST_TO_R = crate::BitReader;
#[doc = "Field `SCL_MAIN_ST_TO` reader - The masked interrupt status status of I2C_SCL_MAIN_ST_TO_INT interrupt."]
pub type SCL_MAIN_ST_TO_R = crate::BitReader;
#[doc = "Field `DET_START` reader - The masked interrupt status status of I2C_DET_START_INT interrupt."]
pub type DET_START_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The masked interrupt status status of I2C_RXFIFO_WM_INT interrupt."]
    #[inline(always)]
    pub fn rxfifo_wm(&self) -> RXFIFO_WM_R {
        RXFIFO_WM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The masked interrupt status status of I2C_TXFIFO_WM_INT interrupt."]
    #[inline(always)]
    pub fn txfifo_wm(&self) -> TXFIFO_WM_R {
        TXFIFO_WM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The masked interrupt status status of I2C_RXFIFO_OVF_INT interrupt."]
    #[inline(always)]
    pub fn rxfifo_ovf(&self) -> RXFIFO_OVF_R {
        RXFIFO_OVF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The masked interrupt status status of the I2C_END_DETECT_INT interrupt."]
    #[inline(always)]
    pub fn end_detect(&self) -> END_DETECT_R {
        END_DETECT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The masked interrupt status status of the I2C_END_DETECT_INT interrupt."]
    #[inline(always)]
    pub fn byte_trans_done(&self) -> BYTE_TRANS_DONE_R {
        BYTE_TRANS_DONE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The masked interrupt status status of the I2C_ARBITRATION_LOST_INT interrupt."]
    #[inline(always)]
    pub fn arbitration_lost(&self) -> ARBITRATION_LOST_R {
        ARBITRATION_LOST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The masked interrupt status status of I2C_TRANS_COMPLETE_INT interrupt."]
    #[inline(always)]
    pub fn mst_txfifo_udf(&self) -> MST_TXFIFO_UDF_R {
        MST_TXFIFO_UDF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The masked interrupt status status of the I2C_TRANS_COMPLETE_INT interrupt."]
    #[inline(always)]
    pub fn trans_complete(&self) -> TRANS_COMPLETE_R {
        TRANS_COMPLETE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The masked interrupt status status of the I2C_TIME_OUT_INT interrupt."]
    #[inline(always)]
    pub fn time_out(&self) -> TIME_OUT_R {
        TIME_OUT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The masked interrupt status status of the I2C_TRANS_START_INT interrupt."]
    #[inline(always)]
    pub fn trans_start(&self) -> TRANS_START_R {
        TRANS_START_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The masked interrupt status status of I2C_SLAVE_STRETCH_INT interrupt."]
    #[inline(always)]
    pub fn nack(&self) -> NACK_R {
        NACK_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The masked interrupt status status of I2C_TXFIFO_OVF_INT interrupt."]
    #[inline(always)]
    pub fn txfifo_ovf(&self) -> TXFIFO_OVF_R {
        TXFIFO_OVF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The masked interrupt status status of I2C_RXFIFO_UDF_INT interrupt."]
    #[inline(always)]
    pub fn rxfifo_udf(&self) -> RXFIFO_UDF_R {
        RXFIFO_UDF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The masked interrupt status status of I2C_SCL_ST_TO_INT interrupt."]
    #[inline(always)]
    pub fn scl_st_to(&self) -> SCL_ST_TO_R {
        SCL_ST_TO_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The masked interrupt status status of I2C_SCL_MAIN_ST_TO_INT interrupt."]
    #[inline(always)]
    pub fn scl_main_st_to(&self) -> SCL_MAIN_ST_TO_R {
        SCL_MAIN_ST_TO_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The masked interrupt status status of I2C_DET_START_INT interrupt."]
    #[inline(always)]
    pub fn det_start(&self) -> DET_START_R {
        DET_START_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field("rxfifo_wm", &format_args!("{}", self.rxfifo_wm().bit()))
            .field("txfifo_wm", &format_args!("{}", self.txfifo_wm().bit()))
            .field("rxfifo_ovf", &format_args!("{}", self.rxfifo_ovf().bit()))
            .field("end_detect", &format_args!("{}", self.end_detect().bit()))
            .field(
                "byte_trans_done",
                &format_args!("{}", self.byte_trans_done().bit()),
            )
            .field(
                "arbitration_lost",
                &format_args!("{}", self.arbitration_lost().bit()),
            )
            .field(
                "mst_txfifo_udf",
                &format_args!("{}", self.mst_txfifo_udf().bit()),
            )
            .field(
                "trans_complete",
                &format_args!("{}", self.trans_complete().bit()),
            )
            .field("time_out", &format_args!("{}", self.time_out().bit()))
            .field("trans_start", &format_args!("{}", self.trans_start().bit()))
            .field("nack", &format_args!("{}", self.nack().bit()))
            .field("txfifo_ovf", &format_args!("{}", self.txfifo_ovf().bit()))
            .field("rxfifo_udf", &format_args!("{}", self.rxfifo_udf().bit()))
            .field("scl_st_to", &format_args!("{}", self.scl_st_to().bit()))
            .field(
                "scl_main_st_to",
                &format_args!("{}", self.scl_main_st_to().bit()),
            )
            .field("det_start", &format_args!("{}", self.det_start().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Status of captured I2C communication events\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ST_SPEC;
impl crate::RegisterSpec for INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st::R`](R) reader structure"]
impl crate::Readable for INT_ST_SPEC {}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for INT_ST_SPEC {
    const RESET_VALUE: u32 = 0;
}