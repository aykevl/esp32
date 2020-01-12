#[doc = "Reader of register LACTUPDATE"]
pub type R = crate::R<u32, super::LACTUPDATE>;
#[doc = "Writer for register LACTUPDATE"]
pub type W = crate::W<u32, super::LACTUPDATE>;
#[doc = "Register LACTUPDATE `reset()`'s with value 0"]
impl crate::ResetValue for super::LACTUPDATE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LACT_UPDATE`"]
pub type LACT_UPDATE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `LACT_UPDATE`"]
pub struct LACT_UPDATE_W<'a> {
    w: &'a mut W,
}
impl<'a> LACT_UPDATE_W<'a> {
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
    pub fn lact_update(&self) -> LACT_UPDATE_R {
        LACT_UPDATE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn lact_update(&mut self) -> LACT_UPDATE_W {
        LACT_UPDATE_W { w: self }
    }
}
