#[doc = "Reader of register CH3ADDR"]
pub type R = crate::R<u32, super::CH3ADDR>;
#[doc = "Writer for register CH3ADDR"]
pub type W = crate::W<u32, super::CH3ADDR>;
#[doc = "Register CH3ADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::CH3ADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RMT_APB_MEM_ADDR_CH3`"]
pub type RMT_APB_MEM_ADDR_CH3_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RMT_APB_MEM_ADDR_CH3`"]
pub struct RMT_APB_MEM_ADDR_CH3_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_APB_MEM_ADDR_CH3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - The ram relative address in channel3 by apb fifo access"]
    #[inline(always)]
    pub fn rmt_apb_mem_addr_ch3(&self) -> RMT_APB_MEM_ADDR_CH3_R {
        RMT_APB_MEM_ADDR_CH3_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - The ram relative address in channel3 by apb fifo access"]
    #[inline(always)]
    pub fn rmt_apb_mem_addr_ch3(&mut self) -> RMT_APB_MEM_ADDR_CH3_W {
        RMT_APB_MEM_ADDR_CH3_W { w: self }
    }
}
