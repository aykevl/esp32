#[doc = "Reader of register PIN20"]
pub type R = crate::R<u32, super::PIN20>;
#[doc = "Writer for register PIN20"]
pub type W = crate::W<u32, super::PIN20>;
#[doc = "Register PIN20 `reset()`'s with value 0"]
impl crate::ResetValue for super::PIN20 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PIN20_INT_ENA`"]
pub type PIN20_INT_ENA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PIN20_INT_ENA`"]
pub struct PIN20_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN20_INT_ENA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 13)) | (((value as u32) & 0x1f) << 13);
        self.w
    }
}
#[doc = "Reader of field `PIN20_CONFIG`"]
pub type PIN20_CONFIG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PIN20_CONFIG`"]
pub struct PIN20_CONFIG_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN20_CONFIG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | (((value as u32) & 0x03) << 11);
        self.w
    }
}
#[doc = "Reader of field `PIN20_WAKEUP_ENABLE`"]
pub type PIN20_WAKEUP_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PIN20_WAKEUP_ENABLE`"]
pub struct PIN20_WAKEUP_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN20_WAKEUP_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `PIN20_INT_TYPE`"]
pub type PIN20_INT_TYPE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PIN20_INT_TYPE`"]
pub struct PIN20_INT_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN20_INT_TYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 7)) | (((value as u32) & 0x07) << 7);
        self.w
    }
}
#[doc = "Reader of field `PIN20_PAD_DRIVER`"]
pub type PIN20_PAD_DRIVER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PIN20_PAD_DRIVER`"]
pub struct PIN20_PAD_DRIVER_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN20_PAD_DRIVER_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 13:17 - bit0: APP CPU interrupt enable bit1: APP CPU non-mask interrupt enable bit3: PRO CPU interrupt enable bit4: PRO CPU non-mask interrupt enable bit5: SDIO's extent interrupt enable"]
    #[inline(always)]
    pub fn pin20_int_ena(&self) -> PIN20_INT_ENA_R {
        PIN20_INT_ENA_R::new(((self.bits >> 13) & 0x1f) as u8)
    }
    #[doc = "Bits 11:12 - NA"]
    #[inline(always)]
    pub fn pin20_config(&self) -> PIN20_CONFIG_R {
        PIN20_CONFIG_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bit 10 - GPIO wake up enable only available in light sleep"]
    #[inline(always)]
    pub fn pin20_wakeup_enable(&self) -> PIN20_WAKEUP_ENABLE_R {
        PIN20_WAKEUP_ENABLE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 7:9 - if set to 0: GPIO interrupt disable if set to 1: rising edge trigger if set to 2: falling edge trigger if set to 3: any edge trigger if set to 4: low level trigger if set to 5: high level trigger"]
    #[inline(always)]
    pub fn pin20_int_type(&self) -> PIN20_INT_TYPE_R {
        PIN20_INT_TYPE_R::new(((self.bits >> 7) & 0x07) as u8)
    }
    #[doc = "Bit 2 - if set to 0: normal output if set to 1: open drain"]
    #[inline(always)]
    pub fn pin20_pad_driver(&self) -> PIN20_PAD_DRIVER_R {
        PIN20_PAD_DRIVER_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 13:17 - bit0: APP CPU interrupt enable bit1: APP CPU non-mask interrupt enable bit3: PRO CPU interrupt enable bit4: PRO CPU non-mask interrupt enable bit5: SDIO's extent interrupt enable"]
    #[inline(always)]
    pub fn pin20_int_ena(&mut self) -> PIN20_INT_ENA_W {
        PIN20_INT_ENA_W { w: self }
    }
    #[doc = "Bits 11:12 - NA"]
    #[inline(always)]
    pub fn pin20_config(&mut self) -> PIN20_CONFIG_W {
        PIN20_CONFIG_W { w: self }
    }
    #[doc = "Bit 10 - GPIO wake up enable only available in light sleep"]
    #[inline(always)]
    pub fn pin20_wakeup_enable(&mut self) -> PIN20_WAKEUP_ENABLE_W {
        PIN20_WAKEUP_ENABLE_W { w: self }
    }
    #[doc = "Bits 7:9 - if set to 0: GPIO interrupt disable if set to 1: rising edge trigger if set to 2: falling edge trigger if set to 3: any edge trigger if set to 4: low level trigger if set to 5: high level trigger"]
    #[inline(always)]
    pub fn pin20_int_type(&mut self) -> PIN20_INT_TYPE_W {
        PIN20_INT_TYPE_W { w: self }
    }
    #[doc = "Bit 2 - if set to 0: normal output if set to 1: open drain"]
    #[inline(always)]
    pub fn pin20_pad_driver(&mut self) -> PIN20_PAD_DRIVER_W {
        PIN20_PAD_DRIVER_W { w: self }
    }
}
