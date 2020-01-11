#[doc = "Reader of register AUTOBAUD"]
pub type R = crate::R<u32, super::AUTOBAUD>;
#[doc = "Writer for register AUTOBAUD"]
pub type W = crate::W<u32, super::AUTOBAUD>;
#[doc = "Register AUTOBAUD `reset()`'s with value 0"]
impl crate::ResetValue for super::AUTOBAUD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UART_GLITCH_FILT`"]
pub type UART_GLITCH_FILT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UART_GLITCH_FILT`"]
pub struct UART_GLITCH_FILT_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_GLITCH_FILT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `UART_AUTOBAUD_EN`"]
pub type UART_AUTOBAUD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_AUTOBAUD_EN`"]
pub struct UART_AUTOBAUD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_AUTOBAUD_EN_W<'a> {
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
    #[doc = "Bits 8:15 - when input pulse width is lower then this value igore this pulse.this register is used in autobaud detect process."]
    #[inline(always)]
    pub fn uart_glitch_filt(&self) -> UART_GLITCH_FILT_R {
        UART_GLITCH_FILT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 0 - This is the enable bit for detecting baudrate."]
    #[inline(always)]
    pub fn uart_autobaud_en(&self) -> UART_AUTOBAUD_EN_R {
        UART_AUTOBAUD_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 8:15 - when input pulse width is lower then this value igore this pulse.this register is used in autobaud detect process."]
    #[inline(always)]
    pub fn uart_glitch_filt(&mut self) -> UART_GLITCH_FILT_W {
        UART_GLITCH_FILT_W { w: self }
    }
    #[doc = "Bit 0 - This is the enable bit for detecting baudrate."]
    #[inline(always)]
    pub fn uart_autobaud_en(&mut self) -> UART_AUTOBAUD_EN_W {
        UART_AUTOBAUD_EN_W { w: self }
    }
}
