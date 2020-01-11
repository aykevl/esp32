#[doc = "Reader of register RTC_IO_TOUCH_PAD9"]
pub type R = crate::R<u32, super::RTC_IO_TOUCH_PAD9>;
#[doc = "Writer for register RTC_IO_TOUCH_PAD9"]
pub type W = crate::W<u32, super::RTC_IO_TOUCH_PAD9>;
#[doc = "Register RTC_IO_TOUCH_PAD9 `reset()`'s with value 0"]
impl crate::ResetValue for super::RTC_IO_TOUCH_PAD9 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_IO_TOUCH_PAD9_DAC`"]
pub type RTC_IO_TOUCH_PAD9_DAC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_IO_TOUCH_PAD9_DAC`"]
pub struct RTC_IO_TOUCH_PAD9_DAC_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_TOUCH_PAD9_DAC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 23)) | (((value as u32) & 0x07) << 23);
        self.w
    }
}
#[doc = "Reader of field `RTC_IO_TOUCH_PAD9_START`"]
pub type RTC_IO_TOUCH_PAD9_START_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_TOUCH_PAD9_START`"]
pub struct RTC_IO_TOUCH_PAD9_START_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_TOUCH_PAD9_START_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `RTC_IO_TOUCH_PAD9_TIE_OPT`"]
pub type RTC_IO_TOUCH_PAD9_TIE_OPT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_TOUCH_PAD9_TIE_OPT`"]
pub struct RTC_IO_TOUCH_PAD9_TIE_OPT_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_TOUCH_PAD9_TIE_OPT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `RTC_IO_TOUCH_PAD9_XPD`"]
pub type RTC_IO_TOUCH_PAD9_XPD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_TOUCH_PAD9_XPD`"]
pub struct RTC_IO_TOUCH_PAD9_XPD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_TOUCH_PAD9_XPD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `RTC_IO_TOUCH_PAD9_TO_GPIO`"]
pub type RTC_IO_TOUCH_PAD9_TO_GPIO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_TOUCH_PAD9_TO_GPIO`"]
pub struct RTC_IO_TOUCH_PAD9_TO_GPIO_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_TOUCH_PAD9_TO_GPIO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
impl R {
    #[doc = "Bits 23:25 - touch sensor slope control. 3-bit for each touch panel default 100."]
    #[inline(always)]
    pub fn rtc_io_touch_pad9_dac(&self) -> RTC_IO_TOUCH_PAD9_DAC_R {
        RTC_IO_TOUCH_PAD9_DAC_R::new(((self.bits >> 23) & 0x07) as u8)
    }
    #[doc = "Bit 22 - start touch sensor."]
    #[inline(always)]
    pub fn rtc_io_touch_pad9_start(&self) -> RTC_IO_TOUCH_PAD9_START_R {
        RTC_IO_TOUCH_PAD9_START_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - default touch sensor tie option. 0: tie low 1: tie high."]
    #[inline(always)]
    pub fn rtc_io_touch_pad9_tie_opt(&self) -> RTC_IO_TOUCH_PAD9_TIE_OPT_R {
        RTC_IO_TOUCH_PAD9_TIE_OPT_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - touch sensor power on."]
    #[inline(always)]
    pub fn rtc_io_touch_pad9_xpd(&self) -> RTC_IO_TOUCH_PAD9_XPD_R {
        RTC_IO_TOUCH_PAD9_XPD_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - connect the rtc pad input to digital pad input \u{d3}0\u{d3} is availbale"]
    #[inline(always)]
    pub fn rtc_io_touch_pad9_to_gpio(&self) -> RTC_IO_TOUCH_PAD9_TO_GPIO_R {
        RTC_IO_TOUCH_PAD9_TO_GPIO_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 23:25 - touch sensor slope control. 3-bit for each touch panel default 100."]
    #[inline(always)]
    pub fn rtc_io_touch_pad9_dac(&mut self) -> RTC_IO_TOUCH_PAD9_DAC_W {
        RTC_IO_TOUCH_PAD9_DAC_W { w: self }
    }
    #[doc = "Bit 22 - start touch sensor."]
    #[inline(always)]
    pub fn rtc_io_touch_pad9_start(&mut self) -> RTC_IO_TOUCH_PAD9_START_W {
        RTC_IO_TOUCH_PAD9_START_W { w: self }
    }
    #[doc = "Bit 21 - default touch sensor tie option. 0: tie low 1: tie high."]
    #[inline(always)]
    pub fn rtc_io_touch_pad9_tie_opt(&mut self) -> RTC_IO_TOUCH_PAD9_TIE_OPT_W {
        RTC_IO_TOUCH_PAD9_TIE_OPT_W { w: self }
    }
    #[doc = "Bit 20 - touch sensor power on."]
    #[inline(always)]
    pub fn rtc_io_touch_pad9_xpd(&mut self) -> RTC_IO_TOUCH_PAD9_XPD_W {
        RTC_IO_TOUCH_PAD9_XPD_W { w: self }
    }
    #[doc = "Bit 19 - connect the rtc pad input to digital pad input \u{d3}0\u{d3} is availbale"]
    #[inline(always)]
    pub fn rtc_io_touch_pad9_to_gpio(&mut self) -> RTC_IO_TOUCH_PAD9_TO_GPIO_W {
        RTC_IO_TOUCH_PAD9_TO_GPIO_W { w: self }
    }
}
