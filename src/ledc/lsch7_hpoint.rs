#[doc = "Reader of register LSCH7_HPOINT"]
pub type R = crate::R<u32, super::LSCH7_HPOINT>;
#[doc = "Writer for register LSCH7_HPOINT"]
pub type W = crate::W<u32, super::LSCH7_HPOINT>;
#[doc = "Register LSCH7_HPOINT `reset()`'s with value 0"]
impl crate::ResetValue for super::LSCH7_HPOINT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HPOINT_LSCH7`"]
pub type HPOINT_LSCH7_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `HPOINT_LSCH7`"]
pub struct HPOINT_LSCH7_W<'a> {
    w: &'a mut W,
}
impl<'a> HPOINT_LSCH7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | ((value as u32) & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:19 - The output value changes to high when lstimerx(x=\\[0 3\\]) selected by low speed channel7 has reached reg_hpoint_lsch7\\[19:0\\]"]
    #[inline(always)]
    pub fn hpoint_lsch7(&self) -> HPOINT_LSCH7_R {
        HPOINT_LSCH7_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:19 - The output value changes to high when lstimerx(x=\\[0 3\\]) selected by low speed channel7 has reached reg_hpoint_lsch7\\[19:0\\]"]
    #[inline(always)]
    pub fn hpoint_lsch7(&mut self) -> HPOINT_LSCH7_W {
        HPOINT_LSCH7_W { w: self }
    }
}
