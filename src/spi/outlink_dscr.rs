#[doc = "Reader of register OUTLINK_DSCR"]
pub type R = crate::R<u32, super::OUTLINK_DSCR>;
#[doc = "Writer for register OUTLINK_DSCR"]
pub type W = crate::W<u32, super::OUTLINK_DSCR>;
#[doc = "Register OUTLINK_DSCR `reset()`'s with value 0"]
impl crate::ResetValue for super::OUTLINK_DSCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DMA_OUTLINK_DSCR`"]
pub type DMA_OUTLINK_DSCR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DMA_OUTLINK_DSCR`"]
pub struct DMA_OUTLINK_DSCR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_OUTLINK_DSCR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - The content of current out descriptor pointer."]
    #[inline(always)]
    pub fn dma_outlink_dscr(&self) -> DMA_OUTLINK_DSCR_R {
        DMA_OUTLINK_DSCR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - The content of current out descriptor pointer."]
    #[inline(always)]
    pub fn dma_outlink_dscr(&mut self) -> DMA_OUTLINK_DSCR_W {
        DMA_OUTLINK_DSCR_W { w: self }
    }
}
