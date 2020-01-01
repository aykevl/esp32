#[doc = "Reader of register OUT_EOF_BFR_DES_ADDR"]
pub type R = crate::R<u32, super::OUT_EOF_BFR_DES_ADDR>;
#[doc = "Writer for register OUT_EOF_BFR_DES_ADDR"]
pub type W = crate::W<u32, super::OUT_EOF_BFR_DES_ADDR>;
#[doc = "Register OUT_EOF_BFR_DES_ADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::OUT_EOF_BFR_DES_ADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI_DMA_OUT_EOF_BFR_DES_ADDR`"]
pub type SPI_DMA_OUT_EOF_BFR_DES_ADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SPI_DMA_OUT_EOF_BFR_DES_ADDR`"]
pub struct SPI_DMA_OUT_EOF_BFR_DES_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_DMA_OUT_EOF_BFR_DES_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - The address of buffer relative to the outlink descriptor that produce eof."]
    #[inline(always)]
    pub fn spi_dma_out_eof_bfr_des_addr(&self) -> SPI_DMA_OUT_EOF_BFR_DES_ADDR_R {
        SPI_DMA_OUT_EOF_BFR_DES_ADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - The address of buffer relative to the outlink descriptor that produce eof."]
    #[inline(always)]
    pub fn spi_dma_out_eof_bfr_des_addr(&mut self) -> SPI_DMA_OUT_EOF_BFR_DES_ADDR_W {
        SPI_DMA_OUT_EOF_BFR_DES_ADDR_W { w: self }
    }
}
