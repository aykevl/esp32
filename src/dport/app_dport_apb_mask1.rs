#[doc = "Reader of register APP_DPORT_APB_MASK1"]
pub type R = crate::R<u32, super::APP_DPORT_APB_MASK1>;
#[doc = "Writer for register APP_DPORT_APB_MASK1"]
pub type W = crate::W<u32, super::APP_DPORT_APB_MASK1>;
#[doc = "Register APP_DPORT_APB_MASK1 `reset()`'s with value 0"]
impl crate::ResetValue for super::APP_DPORT_APB_MASK1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `APPDPORT_APB_MASK1`"]
pub type APPDPORT_APB_MASK1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `APPDPORT_APB_MASK1`"]
pub struct APPDPORT_APB_MASK1_W<'a> {
    w: &'a mut W,
}
impl<'a> APPDPORT_APB_MASK1_W<'a> {
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
    pub fn appdport_apb_mask1(&self) -> APPDPORT_APB_MASK1_R {
        APPDPORT_APB_MASK1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn appdport_apb_mask1(&mut self) -> APPDPORT_APB_MASK1_W {
        APPDPORT_APB_MASK1_W { w: self }
    }
}
