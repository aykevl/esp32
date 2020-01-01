#[doc = "Reader of register PRO_SPI_INTR_0_MAP"]
pub type R = crate::R<u32, super::PRO_SPI_INTR_0_MAP>;
#[doc = "Writer for register PRO_SPI_INTR_0_MAP"]
pub type W = crate::W<u32, super::PRO_SPI_INTR_0_MAP>;
#[doc = "Register PRO_SPI_INTR_0_MAP `reset()`'s with value 0"]
impl crate::ResetValue for super::PRO_SPI_INTR_0_MAP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DPORT_PRO_SPI_INTR_0_MAP`"]
pub type DPORT_PRO_SPI_INTR_0_MAP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DPORT_PRO_SPI_INTR_0_MAP`"]
pub struct DPORT_PRO_SPI_INTR_0_MAP_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_PRO_SPI_INTR_0_MAP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn dport_pro_spi_intr_0_map(&self) -> DPORT_PRO_SPI_INTR_0_MAP_R {
        DPORT_PRO_SPI_INTR_0_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn dport_pro_spi_intr_0_map(&mut self) -> DPORT_PRO_SPI_INTR_0_MAP_W {
        DPORT_PRO_SPI_INTR_0_MAP_W { w: self }
    }
}
