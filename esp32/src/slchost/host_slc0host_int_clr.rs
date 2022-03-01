#[doc = "Register `HOST_SLC0HOST_INT_CLR` writer"]
pub struct W(crate::W<HOST_SLC0HOST_INT_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HOST_SLC0HOST_INT_CLR_SPEC>;
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
impl From<crate::W<HOST_SLC0HOST_INT_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HOST_SLC0HOST_INT_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HOST_SLC0_TOHOST_BIT0_INT_CLR` writer - "]
pub struct HOST_SLC0_TOHOST_BIT0_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC0_TOHOST_BIT0_INT_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `HOST_SLC0_TOHOST_BIT1_INT_CLR` writer - "]
pub struct HOST_SLC0_TOHOST_BIT1_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC0_TOHOST_BIT1_INT_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `HOST_SLC0_TOHOST_BIT2_INT_CLR` writer - "]
pub struct HOST_SLC0_TOHOST_BIT2_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC0_TOHOST_BIT2_INT_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `HOST_SLC0_TOHOST_BIT3_INT_CLR` writer - "]
pub struct HOST_SLC0_TOHOST_BIT3_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC0_TOHOST_BIT3_INT_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `HOST_SLC0_TOHOST_BIT4_INT_CLR` writer - "]
pub struct HOST_SLC0_TOHOST_BIT4_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC0_TOHOST_BIT4_INT_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `HOST_SLC0_TOHOST_BIT5_INT_CLR` writer - "]
pub struct HOST_SLC0_TOHOST_BIT5_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC0_TOHOST_BIT5_INT_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `HOST_SLC0_TOHOST_BIT6_INT_CLR` writer - "]
pub struct HOST_SLC0_TOHOST_BIT6_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC0_TOHOST_BIT6_INT_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `HOST_SLC0_TOHOST_BIT7_INT_CLR` writer - "]
pub struct HOST_SLC0_TOHOST_BIT7_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC0_TOHOST_BIT7_INT_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `HOST_SLC0_TOKEN0_1TO0_INT_CLR` writer - "]
pub struct HOST_SLC0_TOKEN0_1TO0_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC0_TOKEN0_1TO0_INT_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `HOST_SLC0_TOKEN1_1TO0_INT_CLR` writer - "]
pub struct HOST_SLC0_TOKEN1_1TO0_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC0_TOKEN1_1TO0_INT_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `HOST_SLC0_TOKEN0_0TO1_INT_CLR` writer - "]
pub struct HOST_SLC0_TOKEN0_0TO1_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC0_TOKEN0_0TO1_INT_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `HOST_SLC0_TOKEN1_0TO1_INT_CLR` writer - "]
pub struct HOST_SLC0_TOKEN1_0TO1_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC0_TOKEN1_0TO1_INT_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `HOST_SLC0HOST_RX_SOF_INT_CLR` writer - "]
pub struct HOST_SLC0HOST_RX_SOF_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC0HOST_RX_SOF_INT_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `HOST_SLC0HOST_RX_EOF_INT_CLR` writer - "]
pub struct HOST_SLC0HOST_RX_EOF_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC0HOST_RX_EOF_INT_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `HOST_SLC0HOST_RX_START_INT_CLR` writer - "]
pub struct HOST_SLC0HOST_RX_START_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC0HOST_RX_START_INT_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `HOST_SLC0HOST_TX_START_INT_CLR` writer - "]
pub struct HOST_SLC0HOST_TX_START_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC0HOST_TX_START_INT_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `HOST_SLC0_RX_UDF_INT_CLR` writer - "]
pub struct HOST_SLC0_RX_UDF_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC0_RX_UDF_INT_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `HOST_SLC0_TX_OVF_INT_CLR` writer - "]
pub struct HOST_SLC0_TX_OVF_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC0_TX_OVF_INT_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `HOST_SLC0_RX_PF_VALID_INT_CLR` writer - "]
pub struct HOST_SLC0_RX_PF_VALID_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC0_RX_PF_VALID_INT_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `HOST_SLC0_EXT_BIT0_INT_CLR` writer - "]
pub struct HOST_SLC0_EXT_BIT0_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC0_EXT_BIT0_INT_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `HOST_SLC0_EXT_BIT1_INT_CLR` writer - "]
pub struct HOST_SLC0_EXT_BIT1_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC0_EXT_BIT1_INT_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `HOST_SLC0_EXT_BIT2_INT_CLR` writer - "]
pub struct HOST_SLC0_EXT_BIT2_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC0_EXT_BIT2_INT_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `HOST_SLC0_EXT_BIT3_INT_CLR` writer - "]
pub struct HOST_SLC0_EXT_BIT3_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC0_EXT_BIT3_INT_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `HOST_SLC0_RX_NEW_PACKET_INT_CLR` writer - "]
pub struct HOST_SLC0_RX_NEW_PACKET_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC0_RX_NEW_PACKET_INT_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `HOST_SLC0_HOST_RD_RETRY_INT_CLR` writer - "]
pub struct HOST_SLC0_HOST_RD_RETRY_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC0_HOST_RD_RETRY_INT_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `HOST_GPIO_SDIO_INT_CLR` writer - "]
pub struct HOST_GPIO_SDIO_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_GPIO_SDIO_INT_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn host_slc0_tohost_bit0_int_clr(&mut self) -> HOST_SLC0_TOHOST_BIT0_INT_CLR_W {
        HOST_SLC0_TOHOST_BIT0_INT_CLR_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn host_slc0_tohost_bit1_int_clr(&mut self) -> HOST_SLC0_TOHOST_BIT1_INT_CLR_W {
        HOST_SLC0_TOHOST_BIT1_INT_CLR_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn host_slc0_tohost_bit2_int_clr(&mut self) -> HOST_SLC0_TOHOST_BIT2_INT_CLR_W {
        HOST_SLC0_TOHOST_BIT2_INT_CLR_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn host_slc0_tohost_bit3_int_clr(&mut self) -> HOST_SLC0_TOHOST_BIT3_INT_CLR_W {
        HOST_SLC0_TOHOST_BIT3_INT_CLR_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn host_slc0_tohost_bit4_int_clr(&mut self) -> HOST_SLC0_TOHOST_BIT4_INT_CLR_W {
        HOST_SLC0_TOHOST_BIT4_INT_CLR_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn host_slc0_tohost_bit5_int_clr(&mut self) -> HOST_SLC0_TOHOST_BIT5_INT_CLR_W {
        HOST_SLC0_TOHOST_BIT5_INT_CLR_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn host_slc0_tohost_bit6_int_clr(&mut self) -> HOST_SLC0_TOHOST_BIT6_INT_CLR_W {
        HOST_SLC0_TOHOST_BIT6_INT_CLR_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn host_slc0_tohost_bit7_int_clr(&mut self) -> HOST_SLC0_TOHOST_BIT7_INT_CLR_W {
        HOST_SLC0_TOHOST_BIT7_INT_CLR_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn host_slc0_token0_1to0_int_clr(&mut self) -> HOST_SLC0_TOKEN0_1TO0_INT_CLR_W {
        HOST_SLC0_TOKEN0_1TO0_INT_CLR_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn host_slc0_token1_1to0_int_clr(&mut self) -> HOST_SLC0_TOKEN1_1TO0_INT_CLR_W {
        HOST_SLC0_TOKEN1_1TO0_INT_CLR_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn host_slc0_token0_0to1_int_clr(&mut self) -> HOST_SLC0_TOKEN0_0TO1_INT_CLR_W {
        HOST_SLC0_TOKEN0_0TO1_INT_CLR_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn host_slc0_token1_0to1_int_clr(&mut self) -> HOST_SLC0_TOKEN1_0TO1_INT_CLR_W {
        HOST_SLC0_TOKEN1_0TO1_INT_CLR_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn host_slc0host_rx_sof_int_clr(&mut self) -> HOST_SLC0HOST_RX_SOF_INT_CLR_W {
        HOST_SLC0HOST_RX_SOF_INT_CLR_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn host_slc0host_rx_eof_int_clr(&mut self) -> HOST_SLC0HOST_RX_EOF_INT_CLR_W {
        HOST_SLC0HOST_RX_EOF_INT_CLR_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn host_slc0host_rx_start_int_clr(&mut self) -> HOST_SLC0HOST_RX_START_INT_CLR_W {
        HOST_SLC0HOST_RX_START_INT_CLR_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn host_slc0host_tx_start_int_clr(&mut self) -> HOST_SLC0HOST_TX_START_INT_CLR_W {
        HOST_SLC0HOST_TX_START_INT_CLR_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn host_slc0_rx_udf_int_clr(&mut self) -> HOST_SLC0_RX_UDF_INT_CLR_W {
        HOST_SLC0_RX_UDF_INT_CLR_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn host_slc0_tx_ovf_int_clr(&mut self) -> HOST_SLC0_TX_OVF_INT_CLR_W {
        HOST_SLC0_TX_OVF_INT_CLR_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn host_slc0_rx_pf_valid_int_clr(&mut self) -> HOST_SLC0_RX_PF_VALID_INT_CLR_W {
        HOST_SLC0_RX_PF_VALID_INT_CLR_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn host_slc0_ext_bit0_int_clr(&mut self) -> HOST_SLC0_EXT_BIT0_INT_CLR_W {
        HOST_SLC0_EXT_BIT0_INT_CLR_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn host_slc0_ext_bit1_int_clr(&mut self) -> HOST_SLC0_EXT_BIT1_INT_CLR_W {
        HOST_SLC0_EXT_BIT1_INT_CLR_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn host_slc0_ext_bit2_int_clr(&mut self) -> HOST_SLC0_EXT_BIT2_INT_CLR_W {
        HOST_SLC0_EXT_BIT2_INT_CLR_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn host_slc0_ext_bit3_int_clr(&mut self) -> HOST_SLC0_EXT_BIT3_INT_CLR_W {
        HOST_SLC0_EXT_BIT3_INT_CLR_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn host_slc0_rx_new_packet_int_clr(&mut self) -> HOST_SLC0_RX_NEW_PACKET_INT_CLR_W {
        HOST_SLC0_RX_NEW_PACKET_INT_CLR_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn host_slc0_host_rd_retry_int_clr(&mut self) -> HOST_SLC0_HOST_RD_RETRY_INT_CLR_W {
        HOST_SLC0_HOST_RD_RETRY_INT_CLR_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn host_gpio_sdio_int_clr(&mut self) -> HOST_GPIO_SDIO_INT_CLR_W {
        HOST_GPIO_SDIO_INT_CLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_slc0host_int_clr]
(index.html) module"]
pub struct HOST_SLC0HOST_INT_CLR_SPEC;
impl crate::RegisterSpec for HOST_SLC0HOST_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [host_slc0host_int_clr::W]
(W) writer structure"]
impl crate::Writable for HOST_SLC0HOST_INT_CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HOST_SLC0HOST_INT_CLR to value 0"]
impl crate::Resettable for HOST_SLC0HOST_INT_CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}