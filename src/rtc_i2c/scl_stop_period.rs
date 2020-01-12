#[doc = "Reader of register SCL_STOP_PERIOD"]
pub type R = crate::R<u32, super::SCL_STOP_PERIOD>;
#[doc = "Writer for register SCL_STOP_PERIOD"]
pub type W = crate::W<u32, super::SCL_STOP_PERIOD>;
#[doc = "Register SCL_STOP_PERIOD `reset()`'s with value 0"]
impl crate::ResetValue for super::SCL_STOP_PERIOD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SCL_STOP_PERIOD`"]
pub type SCL_STOP_PERIOD_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SCL_STOP_PERIOD`"]
pub struct SCL_STOP_PERIOD_W<'a> {
    w: &'a mut W,
}
impl<'a> SCL_STOP_PERIOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | ((value as u32) & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:19 - Number of FAST_CLK cycles to wait before generating stop condition"]
    #[inline(always)]
    pub fn scl_stop_period(&self) -> SCL_STOP_PERIOD_R {
        SCL_STOP_PERIOD_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:19 - Number of FAST_CLK cycles to wait before generating stop condition"]
    #[inline(always)]
    pub fn scl_stop_period(&mut self) -> SCL_STOP_PERIOD_W {
        SCL_STOP_PERIOD_W { w: self }
    }
}
