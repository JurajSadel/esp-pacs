#[doc = "Register `ESCAPE_CONF` reader"]
pub struct R(crate::R<ESCAPE_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ESCAPE_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ESCAPE_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ESCAPE_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ESCAPE_CONF` writer"]
pub struct W(crate::W<ESCAPE_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ESCAPE_CONF_SPEC>;
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
impl From<crate::W<ESCAPE_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ESCAPE_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_C0_ESC_EN` reader - a"]
pub type TX_C0_ESC_EN_R = crate::BitReader<bool>;
#[doc = "Field `TX_C0_ESC_EN` writer - a"]
pub type TX_C0_ESC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ESCAPE_CONF_SPEC, bool, O>;
#[doc = "Field `TX_DB_ESC_EN` reader - a"]
pub type TX_DB_ESC_EN_R = crate::BitReader<bool>;
#[doc = "Field `TX_DB_ESC_EN` writer - a"]
pub type TX_DB_ESC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ESCAPE_CONF_SPEC, bool, O>;
#[doc = "Field `TX_11_ESC_EN` reader - a"]
pub type TX_11_ESC_EN_R = crate::BitReader<bool>;
#[doc = "Field `TX_11_ESC_EN` writer - a"]
pub type TX_11_ESC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ESCAPE_CONF_SPEC, bool, O>;
#[doc = "Field `TX_13_ESC_EN` reader - a"]
pub type TX_13_ESC_EN_R = crate::BitReader<bool>;
#[doc = "Field `TX_13_ESC_EN` writer - a"]
pub type TX_13_ESC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ESCAPE_CONF_SPEC, bool, O>;
#[doc = "Field `RX_C0_ESC_EN` reader - a"]
pub type RX_C0_ESC_EN_R = crate::BitReader<bool>;
#[doc = "Field `RX_C0_ESC_EN` writer - a"]
pub type RX_C0_ESC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ESCAPE_CONF_SPEC, bool, O>;
#[doc = "Field `RX_DB_ESC_EN` reader - a"]
pub type RX_DB_ESC_EN_R = crate::BitReader<bool>;
#[doc = "Field `RX_DB_ESC_EN` writer - a"]
pub type RX_DB_ESC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ESCAPE_CONF_SPEC, bool, O>;
#[doc = "Field `RX_11_ESC_EN` reader - a"]
pub type RX_11_ESC_EN_R = crate::BitReader<bool>;
#[doc = "Field `RX_11_ESC_EN` writer - a"]
pub type RX_11_ESC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ESCAPE_CONF_SPEC, bool, O>;
#[doc = "Field `RX_13_ESC_EN` reader - a"]
pub type RX_13_ESC_EN_R = crate::BitReader<bool>;
#[doc = "Field `RX_13_ESC_EN` writer - a"]
pub type RX_13_ESC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ESCAPE_CONF_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - a"]
    #[inline(always)]
    pub fn tx_c0_esc_en(&self) -> TX_C0_ESC_EN_R {
        TX_C0_ESC_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - a"]
    #[inline(always)]
    pub fn tx_db_esc_en(&self) -> TX_DB_ESC_EN_R {
        TX_DB_ESC_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - a"]
    #[inline(always)]
    pub fn tx_11_esc_en(&self) -> TX_11_ESC_EN_R {
        TX_11_ESC_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - a"]
    #[inline(always)]
    pub fn tx_13_esc_en(&self) -> TX_13_ESC_EN_R {
        TX_13_ESC_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - a"]
    #[inline(always)]
    pub fn rx_c0_esc_en(&self) -> RX_C0_ESC_EN_R {
        RX_C0_ESC_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - a"]
    #[inline(always)]
    pub fn rx_db_esc_en(&self) -> RX_DB_ESC_EN_R {
        RX_DB_ESC_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - a"]
    #[inline(always)]
    pub fn rx_11_esc_en(&self) -> RX_11_ESC_EN_R {
        RX_11_ESC_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - a"]
    #[inline(always)]
    pub fn rx_13_esc_en(&self) -> RX_13_ESC_EN_R {
        RX_13_ESC_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - a"]
    #[inline(always)]
    pub fn tx_c0_esc_en(&mut self) -> TX_C0_ESC_EN_W<0> {
        TX_C0_ESC_EN_W::new(self)
    }
    #[doc = "Bit 1 - a"]
    #[inline(always)]
    pub fn tx_db_esc_en(&mut self) -> TX_DB_ESC_EN_W<1> {
        TX_DB_ESC_EN_W::new(self)
    }
    #[doc = "Bit 2 - a"]
    #[inline(always)]
    pub fn tx_11_esc_en(&mut self) -> TX_11_ESC_EN_W<2> {
        TX_11_ESC_EN_W::new(self)
    }
    #[doc = "Bit 3 - a"]
    #[inline(always)]
    pub fn tx_13_esc_en(&mut self) -> TX_13_ESC_EN_W<3> {
        TX_13_ESC_EN_W::new(self)
    }
    #[doc = "Bit 4 - a"]
    #[inline(always)]
    pub fn rx_c0_esc_en(&mut self) -> RX_C0_ESC_EN_W<4> {
        RX_C0_ESC_EN_W::new(self)
    }
    #[doc = "Bit 5 - a"]
    #[inline(always)]
    pub fn rx_db_esc_en(&mut self) -> RX_DB_ESC_EN_W<5> {
        RX_DB_ESC_EN_W::new(self)
    }
    #[doc = "Bit 6 - a"]
    #[inline(always)]
    pub fn rx_11_esc_en(&mut self) -> RX_11_ESC_EN_W<6> {
        RX_11_ESC_EN_W::new(self)
    }
    #[doc = "Bit 7 - a"]
    #[inline(always)]
    pub fn rx_13_esc_en(&mut self) -> RX_13_ESC_EN_W<7> {
        RX_13_ESC_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "a\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [escape_conf](index.html) module"]
pub struct ESCAPE_CONF_SPEC;
impl crate::RegisterSpec for ESCAPE_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [escape_conf::R](R) reader structure"]
impl crate::Readable for ESCAPE_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [escape_conf::W](W) writer structure"]
impl crate::Writable for ESCAPE_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ESCAPE_CONF to value 0x33"]
impl crate::Resettable for ESCAPE_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x33
    }
}