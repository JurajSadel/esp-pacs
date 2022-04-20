#[doc = "Register `CORE_0_DRAM0_EXCEPTION_MONITOR_5` reader"]
pub struct R(crate::R<CORE_0_DRAM0_EXCEPTION_MONITOR_5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_0_DRAM0_EXCEPTION_MONITOR_5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_0_DRAM0_EXCEPTION_MONITOR_5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_0_DRAM0_EXCEPTION_MONITOR_5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CORE_0_DRAM0_RECORDING_PC_1` reader - The second dram0's PC status when trigger DRAM busy interrupt"]
pub struct CORE_0_DRAM0_RECORDING_PC_1_R(crate::FieldReader<u32, u32>);
impl CORE_0_DRAM0_RECORDING_PC_1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CORE_0_DRAM0_RECORDING_PC_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_DRAM0_RECORDING_PC_1_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - The second dram0's PC status when trigger DRAM busy interrupt"]
    #[inline(always)]
    pub fn core_0_dram0_recording_pc_1(&self) -> CORE_0_DRAM0_RECORDING_PC_1_R {
        CORE_0_DRAM0_RECORDING_PC_1_R::new(self.bits)
    }
}
#[doc = "core0 bus busy configuration regsiter\n\nThis register you can [`read`]
(crate::generic::Reg::read). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_0_dram0_exception_monitor_5]
(index.html) module"]
pub struct CORE_0_DRAM0_EXCEPTION_MONITOR_5_SPEC;
impl crate::RegisterSpec for CORE_0_DRAM0_EXCEPTION_MONITOR_5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_0_dram0_exception_monitor_5::R]
(R) reader structure"]
impl crate::Readable for CORE_0_DRAM0_EXCEPTION_MONITOR_5_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CORE_0_DRAM0_EXCEPTION_MONITOR_5 to value 0xffff_ffff"]
impl crate::Resettable for CORE_0_DRAM0_EXCEPTION_MONITOR_5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}