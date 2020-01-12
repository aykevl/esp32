#[doc = "Reader of register DMA_IN_DSCR_BF1"]
pub type R = crate::R<u32, super::DMA_IN_DSCR_BF1>;
#[doc = "Writer for register DMA_IN_DSCR_BF1"]
pub type W = crate::W<u32, super::DMA_IN_DSCR_BF1>;
#[doc = "Register DMA_IN_DSCR_BF1 `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_IN_DSCR_BF1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INLINK_DSCR_BF1`"]
pub type INLINK_DSCR_BF1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `INLINK_DSCR_BF1`"]
pub struct INLINK_DSCR_BF1_W<'a> {
    w: &'a mut W,
}
impl<'a> INLINK_DSCR_BF1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - The content of current in link descriptor's second dword"]
    #[inline(always)]
    pub fn inlink_dscr_bf1(&self) -> INLINK_DSCR_BF1_R {
        INLINK_DSCR_BF1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - The content of current in link descriptor's second dword"]
    #[inline(always)]
    pub fn inlink_dscr_bf1(&mut self) -> INLINK_DSCR_BF1_W {
        INLINK_DSCR_BF1_W { w: self }
    }
}
