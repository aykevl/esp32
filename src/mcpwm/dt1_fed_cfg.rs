#[doc = "Reader of register DT1_FED_CFG"]
pub type R = crate::R<u32, super::DT1_FED_CFG>;
#[doc = "Writer for register DT1_FED_CFG"]
pub type W = crate::W<u32, super::DT1_FED_CFG>;
#[doc = "Register DT1_FED_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::DT1_FED_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DT1_FED`"]
pub type DT1_FED_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DT1_FED`"]
pub struct DT1_FED_W<'a> {
    w: &'a mut W,
}
impl<'a> DT1_FED_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Shadow reg for FED"]
    #[inline(always)]
    pub fn dt1_fed(&self) -> DT1_FED_R {
        DT1_FED_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Shadow reg for FED"]
    #[inline(always)]
    pub fn dt1_fed(&mut self) -> DT1_FED_W {
        DT1_FED_W { w: self }
    }
}
