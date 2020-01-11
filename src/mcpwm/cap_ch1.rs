#[doc = "Reader of register CAP_CH1"]
pub type R = crate::R<u32, super::CAP_CH1>;
#[doc = "Writer for register CAP_CH1"]
pub type W = crate::W<u32, super::CAP_CH1>;
#[doc = "Register CAP_CH1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CAP_CH1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MCPWM_CAP1_VALUE`"]
pub type MCPWM_CAP1_VALUE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `MCPWM_CAP1_VALUE`"]
pub struct MCPWM_CAP1_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_CAP1_VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Value of last capture on channel 1"]
    #[inline(always)]
    pub fn mcpwm_cap1_value(&self) -> MCPWM_CAP1_VALUE_R {
        MCPWM_CAP1_VALUE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Value of last capture on channel 1"]
    #[inline(always)]
    pub fn mcpwm_cap1_value(&mut self) -> MCPWM_CAP1_VALUE_W {
        MCPWM_CAP1_VALUE_W { w: self }
    }
}
