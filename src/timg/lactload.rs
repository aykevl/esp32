#[doc = "Reader of register LACTLOAD"]
pub type R = crate::R<u32, super::LACTLOAD>;
#[doc = "Writer for register LACTLOAD"]
pub type W = crate::W<u32, super::LACTLOAD>;
#[doc = "Register LACTLOAD `reset()`'s with value 0"]
impl crate::ResetValue for super::LACTLOAD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TIMG_LACT_LOAD`"]
pub type TIMG_LACT_LOAD_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TIMG_LACT_LOAD`"]
pub struct TIMG_LACT_LOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMG_LACT_LOAD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn timg_lact_load(&self) -> TIMG_LACT_LOAD_R {
        TIMG_LACT_LOAD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn timg_lact_load(&mut self) -> TIMG_LACT_LOAD_W {
        TIMG_LACT_LOAD_W { w: self }
    }
}
