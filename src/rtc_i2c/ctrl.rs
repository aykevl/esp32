#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_I2C_RX_LSB_FIRST`"]
pub type RTC_I2C_RX_LSB_FIRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_I2C_RX_LSB_FIRST`"]
pub struct RTC_I2C_RX_LSB_FIRST_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_I2C_RX_LSB_FIRST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `RTC_I2C_TX_LSB_FIRST`"]
pub type RTC_I2C_TX_LSB_FIRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_I2C_TX_LSB_FIRST`"]
pub struct RTC_I2C_TX_LSB_FIRST_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_I2C_TX_LSB_FIRST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `RTC_I2C_TRANS_START`"]
pub type RTC_I2C_TRANS_START_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_I2C_TRANS_START`"]
pub struct RTC_I2C_TRANS_START_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_I2C_TRANS_START_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `RTC_I2C_MS_MODE`"]
pub type RTC_I2C_MS_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_I2C_MS_MODE`"]
pub struct RTC_I2C_MS_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_I2C_MS_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `RTC_I2C_SCL_FORCE_OUT`"]
pub type RTC_I2C_SCL_FORCE_OUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_I2C_SCL_FORCE_OUT`"]
pub struct RTC_I2C_SCL_FORCE_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_I2C_SCL_FORCE_OUT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `RTC_I2C_SDA_FORCE_OUT`"]
pub type RTC_I2C_SDA_FORCE_OUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_I2C_SDA_FORCE_OUT`"]
pub struct RTC_I2C_SDA_FORCE_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_I2C_SDA_FORCE_OUT_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 7 - Receive LSB first"]
    #[inline(always)]
    pub fn rtc_i2c_rx_lsb_first(&self) -> RTC_I2C_RX_LSB_FIRST_R {
        RTC_I2C_RX_LSB_FIRST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Send LSB first"]
    #[inline(always)]
    pub fn rtc_i2c_tx_lsb_first(&self) -> RTC_I2C_TX_LSB_FIRST_R {
        RTC_I2C_TX_LSB_FIRST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Force to generate start condition"]
    #[inline(always)]
    pub fn rtc_i2c_trans_start(&self) -> RTC_I2C_TRANS_START_R {
        RTC_I2C_TRANS_START_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Master (1) or slave (0)"]
    #[inline(always)]
    pub fn rtc_i2c_ms_mode(&self) -> RTC_I2C_MS_MODE_R {
        RTC_I2C_MS_MODE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 1 - SCL is push-pull (1) or open-drain (0)"]
    #[inline(always)]
    pub fn rtc_i2c_scl_force_out(&self) -> RTC_I2C_SCL_FORCE_OUT_R {
        RTC_I2C_SCL_FORCE_OUT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - SDA is push-pull (1) or open-drain (0)"]
    #[inline(always)]
    pub fn rtc_i2c_sda_force_out(&self) -> RTC_I2C_SDA_FORCE_OUT_R {
        RTC_I2C_SDA_FORCE_OUT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Receive LSB first"]
    #[inline(always)]
    pub fn rtc_i2c_rx_lsb_first(&mut self) -> RTC_I2C_RX_LSB_FIRST_W {
        RTC_I2C_RX_LSB_FIRST_W { w: self }
    }
    #[doc = "Bit 6 - Send LSB first"]
    #[inline(always)]
    pub fn rtc_i2c_tx_lsb_first(&mut self) -> RTC_I2C_TX_LSB_FIRST_W {
        RTC_I2C_TX_LSB_FIRST_W { w: self }
    }
    #[doc = "Bit 5 - Force to generate start condition"]
    #[inline(always)]
    pub fn rtc_i2c_trans_start(&mut self) -> RTC_I2C_TRANS_START_W {
        RTC_I2C_TRANS_START_W { w: self }
    }
    #[doc = "Bit 4 - Master (1) or slave (0)"]
    #[inline(always)]
    pub fn rtc_i2c_ms_mode(&mut self) -> RTC_I2C_MS_MODE_W {
        RTC_I2C_MS_MODE_W { w: self }
    }
    #[doc = "Bit 1 - SCL is push-pull (1) or open-drain (0)"]
    #[inline(always)]
    pub fn rtc_i2c_scl_force_out(&mut self) -> RTC_I2C_SCL_FORCE_OUT_W {
        RTC_I2C_SCL_FORCE_OUT_W { w: self }
    }
    #[doc = "Bit 0 - SDA is push-pull (1) or open-drain (0)"]
    #[inline(always)]
    pub fn rtc_i2c_sda_force_out(&mut self) -> RTC_I2C_SDA_FORCE_OUT_W {
        RTC_I2C_SDA_FORCE_OUT_W { w: self }
    }
}
