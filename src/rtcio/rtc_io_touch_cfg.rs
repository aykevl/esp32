#[doc = "Reader of register RTC_IO_TOUCH_CFG"]
pub type R = crate::R<u32, super::RTC_IO_TOUCH_CFG>;
#[doc = "Writer for register RTC_IO_TOUCH_CFG"]
pub type W = crate::W<u32, super::RTC_IO_TOUCH_CFG>;
#[doc = "Register RTC_IO_TOUCH_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::RTC_IO_TOUCH_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_IO_TOUCH_XPD_BIAS`"]
pub type RTC_IO_TOUCH_XPD_BIAS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_TOUCH_XPD_BIAS`"]
pub struct RTC_IO_TOUCH_XPD_BIAS_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_TOUCH_XPD_BIAS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `RTC_IO_TOUCH_DREFH`"]
pub type RTC_IO_TOUCH_DREFH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_IO_TOUCH_DREFH`"]
pub struct RTC_IO_TOUCH_DREFH_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_TOUCH_DREFH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 29)) | (((value as u32) & 0x03) << 29);
        self.w
    }
}
#[doc = "Reader of field `RTC_IO_TOUCH_DREFL`"]
pub type RTC_IO_TOUCH_DREFL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_IO_TOUCH_DREFL`"]
pub struct RTC_IO_TOUCH_DREFL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_TOUCH_DREFL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 27)) | (((value as u32) & 0x03) << 27);
        self.w
    }
}
#[doc = "Reader of field `RTC_IO_TOUCH_DRANGE`"]
pub type RTC_IO_TOUCH_DRANGE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_IO_TOUCH_DRANGE`"]
pub struct RTC_IO_TOUCH_DRANGE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_TOUCH_DRANGE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 25)) | (((value as u32) & 0x03) << 25);
        self.w
    }
}
#[doc = "Reader of field `RTC_IO_TOUCH_DCUR`"]
pub type RTC_IO_TOUCH_DCUR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_IO_TOUCH_DCUR`"]
pub struct RTC_IO_TOUCH_DCUR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_TOUCH_DCUR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 23)) | (((value as u32) & 0x03) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - touch sensor bias power on."]
    #[inline(always)]
    pub fn rtc_io_touch_xpd_bias(&self) -> RTC_IO_TOUCH_XPD_BIAS_R {
        RTC_IO_TOUCH_XPD_BIAS_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 29:30 - touch sensor saw wave top voltage."]
    #[inline(always)]
    pub fn rtc_io_touch_drefh(&self) -> RTC_IO_TOUCH_DREFH_R {
        RTC_IO_TOUCH_DREFH_R::new(((self.bits >> 29) & 0x03) as u8)
    }
    #[doc = "Bits 27:28 - touch sensor saw wave bottom voltage."]
    #[inline(always)]
    pub fn rtc_io_touch_drefl(&self) -> RTC_IO_TOUCH_DREFL_R {
        RTC_IO_TOUCH_DREFL_R::new(((self.bits >> 27) & 0x03) as u8)
    }
    #[doc = "Bits 25:26 - touch sensor saw wave voltage range."]
    #[inline(always)]
    pub fn rtc_io_touch_drange(&self) -> RTC_IO_TOUCH_DRANGE_R {
        RTC_IO_TOUCH_DRANGE_R::new(((self.bits >> 25) & 0x03) as u8)
    }
    #[doc = "Bits 23:24 - touch sensor bias current. Should have option to tie with BIAS_SLEEP(When BIAS_SLEEP this setting is available"]
    #[inline(always)]
    pub fn rtc_io_touch_dcur(&self) -> RTC_IO_TOUCH_DCUR_R {
        RTC_IO_TOUCH_DCUR_R::new(((self.bits >> 23) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - touch sensor bias power on."]
    #[inline(always)]
    pub fn rtc_io_touch_xpd_bias(&mut self) -> RTC_IO_TOUCH_XPD_BIAS_W {
        RTC_IO_TOUCH_XPD_BIAS_W { w: self }
    }
    #[doc = "Bits 29:30 - touch sensor saw wave top voltage."]
    #[inline(always)]
    pub fn rtc_io_touch_drefh(&mut self) -> RTC_IO_TOUCH_DREFH_W {
        RTC_IO_TOUCH_DREFH_W { w: self }
    }
    #[doc = "Bits 27:28 - touch sensor saw wave bottom voltage."]
    #[inline(always)]
    pub fn rtc_io_touch_drefl(&mut self) -> RTC_IO_TOUCH_DREFL_W {
        RTC_IO_TOUCH_DREFL_W { w: self }
    }
    #[doc = "Bits 25:26 - touch sensor saw wave voltage range."]
    #[inline(always)]
    pub fn rtc_io_touch_drange(&mut self) -> RTC_IO_TOUCH_DRANGE_W {
        RTC_IO_TOUCH_DRANGE_W { w: self }
    }
    #[doc = "Bits 23:24 - touch sensor bias current. Should have option to tie with BIAS_SLEEP(When BIAS_SLEEP this setting is available"]
    #[inline(always)]
    pub fn rtc_io_touch_dcur(&mut self) -> RTC_IO_TOUCH_DCUR_W {
        RTC_IO_TOUCH_DCUR_W { w: self }
    }
}
