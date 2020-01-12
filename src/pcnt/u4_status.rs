#[doc = "Reader of register U4_STATUS"]
pub type R = crate::R<u32, super::U4_STATUS>;
#[doc = "Writer for register U4_STATUS"]
pub type W = crate::W<u32, super::U4_STATUS>;
#[doc = "Register U4_STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::U4_STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CORE_STATUS_U4`"]
pub type CORE_STATUS_U4_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CORE_STATUS_U4`"]
pub struct CORE_STATUS_U4_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_STATUS_U4_W<'a> {
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
    pub fn core_status_u4(&self) -> CORE_STATUS_U4_R {
        CORE_STATUS_U4_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn core_status_u4(&mut self) -> CORE_STATUS_U4_W {
        CORE_STATUS_U4_W { w: self }
    }
}
