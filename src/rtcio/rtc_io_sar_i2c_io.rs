#[doc = "Reader of register RTC_IO_SAR_I2C_IO"]
pub type R = crate::R<u32, super::RTC_IO_SAR_I2C_IO>;
#[doc = "Writer for register RTC_IO_SAR_I2C_IO"]
pub type W = crate::W<u32, super::RTC_IO_SAR_I2C_IO>;
#[doc = "Register RTC_IO_SAR_I2C_IO `reset()`'s with value 0"]
impl crate::ResetValue for super::RTC_IO_SAR_I2C_IO {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_IO_SAR_I2C_SDA_SEL`"]
pub type RTC_IO_SAR_I2C_SDA_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_IO_SAR_I2C_SDA_SEL`"]
pub struct RTC_IO_SAR_I2C_SDA_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_SAR_I2C_SDA_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
#[doc = "Reader of field `RTC_IO_SAR_I2C_SCL_SEL`"]
pub type RTC_IO_SAR_I2C_SCL_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_IO_SAR_I2C_SCL_SEL`"]
pub struct RTC_IO_SAR_I2C_SCL_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_SAR_I2C_SCL_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Reader of field `RTC_IO_SAR_DEBUG_BIT_SEL`"]
pub type RTC_IO_SAR_DEBUG_BIT_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_IO_SAR_DEBUG_BIT_SEL`"]
pub struct RTC_IO_SAR_DEBUG_BIT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_SAR_DEBUG_BIT_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 23)) | (((value as u32) & 0x1f) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31 - \u{d2}0\u{d3} using TOUCH_PAD\\[1\\] as i2c sda \u{d2}1\u{d3} using TOUCH_PAD\\[3\\] as i2c sda"]
    #[inline(always)]
    pub fn rtc_io_sar_i2c_sda_sel(&self) -> RTC_IO_SAR_I2C_SDA_SEL_R {
        RTC_IO_SAR_I2C_SDA_SEL_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - \u{d2}0\u{d3} using TOUCH_PAD\\[0\\] as i2c clk \u{d2}1\u{d3} using TOUCH_PAD\\[2\\] as i2c clk"]
    #[inline(always)]
    pub fn rtc_io_sar_i2c_scl_sel(&self) -> RTC_IO_SAR_I2C_SCL_SEL_R {
        RTC_IO_SAR_I2C_SCL_SEL_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 23:27"]
    #[inline(always)]
    pub fn rtc_io_sar_debug_bit_sel(&self) -> RTC_IO_SAR_DEBUG_BIT_SEL_R {
        RTC_IO_SAR_DEBUG_BIT_SEL_R::new(((self.bits >> 23) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31 - \u{d2}0\u{d3} using TOUCH_PAD\\[1\\] as i2c sda \u{d2}1\u{d3} using TOUCH_PAD\\[3\\] as i2c sda"]
    #[inline(always)]
    pub fn rtc_io_sar_i2c_sda_sel(&mut self) -> RTC_IO_SAR_I2C_SDA_SEL_W {
        RTC_IO_SAR_I2C_SDA_SEL_W { w: self }
    }
    #[doc = "Bits 28:29 - \u{d2}0\u{d3} using TOUCH_PAD\\[0\\] as i2c clk \u{d2}1\u{d3} using TOUCH_PAD\\[2\\] as i2c clk"]
    #[inline(always)]
    pub fn rtc_io_sar_i2c_scl_sel(&mut self) -> RTC_IO_SAR_I2C_SCL_SEL_W {
        RTC_IO_SAR_I2C_SCL_SEL_W { w: self }
    }
    #[doc = "Bits 23:27"]
    #[inline(always)]
    pub fn rtc_io_sar_debug_bit_sel(&mut self) -> RTC_IO_SAR_DEBUG_BIT_SEL_W {
        RTC_IO_SAR_DEBUG_BIT_SEL_W { w: self }
    }
}
