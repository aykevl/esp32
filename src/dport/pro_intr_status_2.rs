#[doc = "Reader of register PRO_INTR_STATUS_2"]
pub type R = crate::R<u32, super::PRO_INTR_STATUS_2>;
#[doc = "Writer for register PRO_INTR_STATUS_2"]
pub type W = crate::W<u32, super::PRO_INTR_STATUS_2>;
#[doc = "Register PRO_INTR_STATUS_2 `reset()`'s with value 0"]
impl crate::ResetValue for super::PRO_INTR_STATUS_2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PRO_INTR_STATUS_2`"]
pub type PRO_INTR_STATUS_2_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PRO_INTR_STATUS_2`"]
pub struct PRO_INTR_STATUS_2_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_INTR_STATUS_2_W<'a> {
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
    pub fn pro_intr_status_2(&self) -> PRO_INTR_STATUS_2_R {
        PRO_INTR_STATUS_2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn pro_intr_status_2(&mut self) -> PRO_INTR_STATUS_2_W {
        PRO_INTR_STATUS_2_W { w: self }
    }
}
