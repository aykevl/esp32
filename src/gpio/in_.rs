#[doc = "Reader of register IN"]
pub type R = crate::R<u32, super::IN>;
#[doc = "Writer for register IN"]
pub type W = crate::W<u32, super::IN>;
#[doc = "Register IN `reset()`'s with value 0"]
impl crate::ResetValue for super::IN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GPIO_IN_DATA`"]
pub type GPIO_IN_DATA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `GPIO_IN_DATA`"]
pub struct GPIO_IN_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_IN_DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - GPIO0~31 input value"]
    #[inline(always)]
    pub fn gpio_in_data(&self) -> GPIO_IN_DATA_R {
        GPIO_IN_DATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - GPIO0~31 input value"]
    #[inline(always)]
    pub fn gpio_in_data(&mut self) -> GPIO_IN_DATA_W {
        GPIO_IN_DATA_W { w: self }
    }
}
