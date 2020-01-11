#[doc = "Reader of register CPUSDIO_INT1"]
pub type R = crate::R<u32, super::CPUSDIO_INT1>;
#[doc = "Writer for register CPUSDIO_INT1"]
pub type W = crate::W<u32, super::CPUSDIO_INT1>;
#[doc = "Register CPUSDIO_INT1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CPUSDIO_INT1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GPIO_SDIO_INT_H`"]
pub type GPIO_SDIO_INT_H_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GPIO_SDIO_INT_H`"]
pub struct GPIO_SDIO_INT_H_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_SDIO_INT_H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - SDIO's extent GPIO32~39 interrupt"]
    #[inline(always)]
    pub fn gpio_sdio_int_h(&self) -> GPIO_SDIO_INT_H_R {
        GPIO_SDIO_INT_H_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SDIO's extent GPIO32~39 interrupt"]
    #[inline(always)]
    pub fn gpio_sdio_int_h(&mut self) -> GPIO_SDIO_INT_H_W {
        GPIO_SDIO_INT_H_W { w: self }
    }
}
