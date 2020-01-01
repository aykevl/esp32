#[doc = "Reader of register MEM_CNT_STATUS"]
pub type R = crate::R<u32, super::MEM_CNT_STATUS>;
#[doc = "Writer for register MEM_CNT_STATUS"]
pub type W = crate::W<u32, super::MEM_CNT_STATUS>;
#[doc = "Register MEM_CNT_STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::MEM_CNT_STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UART_TX_MEM_CNT`"]
pub type UART_TX_MEM_CNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UART_TX_MEM_CNT`"]
pub struct UART_TX_MEM_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_TX_MEM_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
        self.w
    }
}
#[doc = "Reader of field `UART_RX_MEM_CNT`"]
pub type UART_RX_MEM_CNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UART_RX_MEM_CNT`"]
pub struct UART_RX_MEM_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_RX_MEM_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:5 - refer to the txfifo_cnt's describtion."]
    #[inline(always)]
    pub fn uart_tx_mem_cnt(&self) -> UART_TX_MEM_CNT_R {
        UART_TX_MEM_CNT_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 0:2 - refer to the rxfifo_cnt's describtion."]
    #[inline(always)]
    pub fn uart_rx_mem_cnt(&self) -> UART_RX_MEM_CNT_R {
        UART_RX_MEM_CNT_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 3:5 - refer to the txfifo_cnt's describtion."]
    #[inline(always)]
    pub fn uart_tx_mem_cnt(&mut self) -> UART_TX_MEM_CNT_W {
        UART_TX_MEM_CNT_W { w: self }
    }
    #[doc = "Bits 0:2 - refer to the rxfifo_cnt's describtion."]
    #[inline(always)]
    pub fn uart_rx_mem_cnt(&mut self) -> UART_RX_MEM_CNT_W {
        UART_RX_MEM_CNT_W { w: self }
    }
}
