#[doc = "Reader of register CH3_TX_LIM"]
pub type R = crate::R<u32, super::CH3_TX_LIM>;
#[doc = "Writer for register CH3_TX_LIM"]
pub type W = crate::W<u32, super::CH3_TX_LIM>;
#[doc = "Register CH3_TX_LIM `reset()`'s with value 0"]
impl crate::ResetValue for super::CH3_TX_LIM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TX_LIM_CH3`"]
pub type TX_LIM_CH3_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TX_LIM_CH3`"]
pub struct TX_LIM_CH3_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_LIM_CH3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - When channel3 sends more than reg_rmt_tx_lim_ch3 datas then channel3 produce the relative interrupt."]
    #[inline(always)]
    pub fn tx_lim_ch3(&self) -> TX_LIM_CH3_R {
        TX_LIM_CH3_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - When channel3 sends more than reg_rmt_tx_lim_ch3 datas then channel3 produce the relative interrupt."]
    #[inline(always)]
    pub fn tx_lim_ch3(&mut self) -> TX_LIM_CH3_W {
        TX_LIM_CH3_W { w: self }
    }
}
