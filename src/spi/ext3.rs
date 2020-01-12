#[doc = "Reader of register EXT3"]
pub type R = crate::R<u32, super::EXT3>;
#[doc = "Writer for register EXT3"]
pub type W = crate::W<u32, super::EXT3>;
#[doc = "Register EXT3 `reset()`'s with value 0"]
impl crate::ResetValue for super::EXT3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INT_HOLD_ENA`"]
pub type INT_HOLD_ENA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INT_HOLD_ENA`"]
pub struct INT_HOLD_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_HOLD_ENA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - This register is for two SPI masters to share the same cs clock and data signals. The bits of one SPI are set if the other SPI is busy the SPI will be hold. 1(3): hold at \u{a1}\u{b0}idle\u{a1}\u{b1} phase 2: hold at \u{a1}\u{b0}prepare\u{a1}\u{b1} phase."]
    #[inline(always)]
    pub fn int_hold_ena(&self) -> INT_HOLD_ENA_R {
        INT_HOLD_ENA_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - This register is for two SPI masters to share the same cs clock and data signals. The bits of one SPI are set if the other SPI is busy the SPI will be hold. 1(3): hold at \u{a1}\u{b0}idle\u{a1}\u{b1} phase 2: hold at \u{a1}\u{b0}prepare\u{a1}\u{b1} phase."]
    #[inline(always)]
    pub fn int_hold_ena(&mut self) -> INT_HOLD_ENA_W {
        INT_HOLD_ENA_W { w: self }
    }
}
