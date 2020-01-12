#[doc = "Reader of register HSCH3_HPOINT"]
pub type R = crate::R<u32, super::HSCH3_HPOINT>;
#[doc = "Writer for register HSCH3_HPOINT"]
pub type W = crate::W<u32, super::HSCH3_HPOINT>;
#[doc = "Register HSCH3_HPOINT `reset()`'s with value 0"]
impl crate::ResetValue for super::HSCH3_HPOINT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HPOINT_HSCH3`"]
pub type HPOINT_HSCH3_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `HPOINT_HSCH3`"]
pub struct HPOINT_HSCH3_W<'a> {
    w: &'a mut W,
}
impl<'a> HPOINT_HSCH3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | ((value as u32) & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:19 - The output value changes to high when htimerx(x=\\[0 3\\]) selected by high speed channel3 has reached reg_hpoint_hsch3\\[19:0\\]"]
    #[inline(always)]
    pub fn hpoint_hsch3(&self) -> HPOINT_HSCH3_R {
        HPOINT_HSCH3_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:19 - The output value changes to high when htimerx(x=\\[0 3\\]) selected by high speed channel3 has reached reg_hpoint_hsch3\\[19:0\\]"]
    #[inline(always)]
    pub fn hpoint_hsch3(&mut self) -> HPOINT_HSCH3_W {
        HPOINT_HSCH3_W { w: self }
    }
}
