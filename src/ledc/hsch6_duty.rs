#[doc = "Reader of register HSCH6_DUTY"]
pub type R = crate::R<u32, super::HSCH6_DUTY>;
#[doc = "Writer for register HSCH6_DUTY"]
pub type W = crate::W<u32, super::HSCH6_DUTY>;
#[doc = "Register HSCH6_DUTY `reset()`'s with value 0"]
impl crate::ResetValue for super::HSCH6_DUTY {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DUTY_HSCH6`"]
pub type DUTY_HSCH6_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DUTY_HSCH6`"]
pub struct DUTY_HSCH6_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_HSCH6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff_ffff) | ((value as u32) & 0x01ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:24 - The register is used to control output duty. When hstimerx(x=\\[0 3\\]) choosed by high speed channel6 has reached reg_lpoint_hsch6 the output signal changes to low. reg_lpoint_hsch6=(reg_hpoint_hsch6\\[19:0\\]+reg_duty_hsch6\\[24:4\\]) (1) reg_lpoint_hsch6=(reg_hpoint_hsch6\\[19:0\\]+reg_duty_hsch6\\[24:4\\] +1) (2) The least four bits in this register represent the decimal part and determines when to choose (1) or (2)"]
    #[inline(always)]
    pub fn duty_hsch6(&self) -> DUTY_HSCH6_R {
        DUTY_HSCH6_R::new((self.bits & 0x01ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:24 - The register is used to control output duty. When hstimerx(x=\\[0 3\\]) choosed by high speed channel6 has reached reg_lpoint_hsch6 the output signal changes to low. reg_lpoint_hsch6=(reg_hpoint_hsch6\\[19:0\\]+reg_duty_hsch6\\[24:4\\]) (1) reg_lpoint_hsch6=(reg_hpoint_hsch6\\[19:0\\]+reg_duty_hsch6\\[24:4\\] +1) (2) The least four bits in this register represent the decimal part and determines when to choose (1) or (2)"]
    #[inline(always)]
    pub fn duty_hsch6(&mut self) -> DUTY_HSCH6_W {
        DUTY_HSCH6_W { w: self }
    }
}
