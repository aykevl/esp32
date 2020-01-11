#[doc = "Reader of register GEN2_TSTMP_A"]
pub type R = crate::R<u32, super::GEN2_TSTMP_A>;
#[doc = "Writer for register GEN2_TSTMP_A"]
pub type W = crate::W<u32, super::GEN2_TSTMP_A>;
#[doc = "Register GEN2_TSTMP_A `reset()`'s with value 0"]
impl crate::ResetValue for super::GEN2_TSTMP_A {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MCPWM_GEN2_A`"]
pub type MCPWM_GEN2_A_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MCPWM_GEN2_A`"]
pub struct MCPWM_GEN2_A_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_GEN2_A_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - PWM generator 2 time stamp A's shadow reg"]
    #[inline(always)]
    pub fn mcpwm_gen2_a(&self) -> MCPWM_GEN2_A_R {
        MCPWM_GEN2_A_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PWM generator 2 time stamp A's shadow reg"]
    #[inline(always)]
    pub fn mcpwm_gen2_a(&mut self) -> MCPWM_GEN2_A_W {
        MCPWM_GEN2_A_W { w: self }
    }
}
