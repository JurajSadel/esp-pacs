#[doc = "Register `RXD_CNT` reader"]
pub struct R(crate::R<RXD_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXD_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXD_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXD_CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXD_EDGE_CNT` reader - This register stores the count of RXD edge change. It is used in baud rate detection. As baud rate registers UART_REG_LOWPULSE_MIN_CNT, UART_REG_HIGHPULSE_MIN_CNT, UART_REG_POSEDGE_MIN_CNT, and UART_REG_NEGEDGE_MIN_CNT always record the minimal value, UART_REG_RXD_EDGE_CNT indicates the statistic number of RXD edge to find out the minimal value for these baud rate registers."]
pub struct RXD_EDGE_CNT_R(crate::FieldReader<u16, u16>);
impl RXD_EDGE_CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        RXD_EDGE_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXD_EDGE_CNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:9 - This register stores the count of RXD edge change. It is used in baud rate detection. As baud rate registers UART_REG_LOWPULSE_MIN_CNT, UART_REG_HIGHPULSE_MIN_CNT, UART_REG_POSEDGE_MIN_CNT, and UART_REG_NEGEDGE_MIN_CNT always record the minimal value, UART_REG_RXD_EDGE_CNT indicates the statistic number of RXD edge to find out the minimal value for these baud rate registers."]
    #[inline(always)]
    pub fn rxd_edge_cnt(&self) -> RXD_EDGE_CNT_R {
        RXD_EDGE_CNT_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "Autobaud edge change count register\n\nThis register you can [`read`]
(crate::generic::Reg::read). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxd_cnt]
(index.html) module"]
pub struct RXD_CNT_SPEC;
impl crate::RegisterSpec for RXD_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxd_cnt::R]
(R) reader structure"]
impl crate::Readable for RXD_CNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXD_CNT to value 0"]
impl crate::Resettable for RXD_CNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}