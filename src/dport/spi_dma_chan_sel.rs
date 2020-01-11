#[doc = "Reader of register SPI_DMA_CHAN_SEL"]
pub type R = crate::R<u32, super::SPI_DMA_CHAN_SEL>;
#[doc = "Writer for register SPI_DMA_CHAN_SEL"]
pub type W = crate::W<u32, super::SPI_DMA_CHAN_SEL>;
#[doc = "Register SPI_DMA_CHAN_SEL `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI_DMA_CHAN_SEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DPORT_SPI3_DMA_CHAN_SEL`"]
pub type DPORT_SPI3_DMA_CHAN_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DPORT_SPI3_DMA_CHAN_SEL`"]
pub struct DPORT_SPI3_DMA_CHAN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_SPI3_DMA_CHAN_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `DPORT_SPI2_DMA_CHAN_SEL`"]
pub type DPORT_SPI2_DMA_CHAN_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DPORT_SPI2_DMA_CHAN_SEL`"]
pub struct DPORT_SPI2_DMA_CHAN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_SPI2_DMA_CHAN_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `DPORT_SPI1_DMA_CHAN_SEL`"]
pub type DPORT_SPI1_DMA_CHAN_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DPORT_SPI1_DMA_CHAN_SEL`"]
pub struct DPORT_SPI1_DMA_CHAN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_SPI1_DMA_CHAN_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn dport_spi3_dma_chan_sel(&self) -> DPORT_SPI3_DMA_CHAN_SEL_R {
        DPORT_SPI3_DMA_CHAN_SEL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn dport_spi2_dma_chan_sel(&self) -> DPORT_SPI2_DMA_CHAN_SEL_R {
        DPORT_SPI2_DMA_CHAN_SEL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn dport_spi1_dma_chan_sel(&self) -> DPORT_SPI1_DMA_CHAN_SEL_R {
        DPORT_SPI1_DMA_CHAN_SEL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn dport_spi3_dma_chan_sel(&mut self) -> DPORT_SPI3_DMA_CHAN_SEL_W {
        DPORT_SPI3_DMA_CHAN_SEL_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn dport_spi2_dma_chan_sel(&mut self) -> DPORT_SPI2_DMA_CHAN_SEL_W {
        DPORT_SPI2_DMA_CHAN_SEL_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn dport_spi1_dma_chan_sel(&mut self) -> DPORT_SPI1_DMA_CHAN_SEL_W {
        DPORT_SPI1_DMA_CHAN_SEL_W { w: self }
    }
}
