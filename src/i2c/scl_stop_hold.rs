#[doc = "Reader of register SCL_STOP_HOLD"]
pub type R = crate::R<u32, super::SCL_STOP_HOLD>;
#[doc = "Writer for register SCL_STOP_HOLD"]
pub type W = crate::W<u32, super::SCL_STOP_HOLD>;
#[doc = "Register SCL_STOP_HOLD `reset()`'s with value 0"]
impl crate::ResetValue for super::SCL_STOP_HOLD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SCL_STOP_HOLD_TIME`"]
pub type SCL_STOP_HOLD_TIME_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SCL_STOP_HOLD_TIME`"]
pub struct SCL_STOP_HOLD_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> SCL_STOP_HOLD_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | ((value as u32) & 0x3fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:13 - This register is used to configure the clock num after the STOP bit's posedge."]
    #[inline(always)]
    pub fn scl_stop_hold_time(&self) -> SCL_STOP_HOLD_TIME_R {
        SCL_STOP_HOLD_TIME_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - This register is used to configure the clock num after the STOP bit's posedge."]
    #[inline(always)]
    pub fn scl_stop_hold_time(&mut self) -> SCL_STOP_HOLD_TIME_W {
        SCL_STOP_HOLD_TIME_W { w: self }
    }
}
