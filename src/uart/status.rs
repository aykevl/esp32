#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Writer for register STATUS"]
pub type W = crate::W<u32, super::STATUS>;
#[doc = "Register STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UART_TXD`"]
pub type UART_TXD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_TXD`"]
pub struct UART_TXD_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_TXD_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `UART_RTSN`"]
pub type UART_RTSN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_RTSN`"]
pub struct UART_RTSN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_RTSN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `UART_DTRN`"]
pub type UART_DTRN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_DTRN`"]
pub struct UART_DTRN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_DTRN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `UART_ST_UTX_OUT`"]
pub type UART_ST_UTX_OUT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UART_ST_UTX_OUT`"]
pub struct UART_ST_UTX_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_ST_UTX_OUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `UART_TXFIFO_CNT`"]
pub type UART_TXFIFO_CNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UART_TXFIFO_CNT`"]
pub struct UART_TXFIFO_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_TXFIFO_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `UART_RXD`"]
pub type UART_RXD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_RXD`"]
pub struct UART_RXD_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_RXD_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `UART_CTSN`"]
pub type UART_CTSN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_CTSN`"]
pub struct UART_CTSN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_CTSN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `UART_DSRN`"]
pub type UART_DSRN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_DSRN`"]
pub struct UART_DSRN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_DSRN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `UART_ST_URX_OUT`"]
pub type UART_ST_URX_OUT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UART_ST_URX_OUT`"]
pub struct UART_ST_URX_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_ST_URX_OUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `UART_RXFIFO_CNT`"]
pub type UART_RXFIFO_CNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UART_RXFIFO_CNT`"]
pub struct UART_RXFIFO_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_RXFIFO_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - This register represent the level value of the internal uart rxd signal."]
    #[inline(always)]
    pub fn uart_txd(&self) -> UART_TXD_R {
        UART_TXD_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - This register represent the level value of the internal uart cts signal."]
    #[inline(always)]
    pub fn uart_rtsn(&self) -> UART_RTSN_R {
        UART_RTSN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - The register represent the level value of the internal uart dsr signal."]
    #[inline(always)]
    pub fn uart_dtrn(&self) -> UART_DTRN_R {
        UART_DTRN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bits 24:27 - This register stores the value of transmitter's finite state machine. 0:TX_IDLE 1:TX_STRT 2:TX_DAT0 3:TX_DAT1 4:TX_DAT2 5:TX_DAT3 6:TX_DAT4 7:TX_DAT5 8:TX_DAT6 9:TX_DAT7 10:TX_PRTY 11:TX_STP1 12:TX_STP2 13:TX_DL0 14:TX_DL1"]
    #[inline(always)]
    pub fn uart_st_utx_out(&self) -> UART_ST_UTX_OUT_R {
        UART_ST_UTX_OUT_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - (tx_mem_cnt txfifo_cnt) stores the byte num of valid datas in transmitter's fifo.tx_mem_cnt stores the 3 most significant bits txfifo_cnt stores the 8 least significant bits."]
    #[inline(always)]
    pub fn uart_txfifo_cnt(&self) -> UART_TXFIFO_CNT_R {
        UART_TXFIFO_CNT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 15 - This register stores the level value of the internal uart rxd signal."]
    #[inline(always)]
    pub fn uart_rxd(&self) -> UART_RXD_R {
        UART_RXD_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - This register stores the level value of the internal uart cts signal."]
    #[inline(always)]
    pub fn uart_ctsn(&self) -> UART_CTSN_R {
        UART_CTSN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - This register stores the level value of the internal uart dsr signal."]
    #[inline(always)]
    pub fn uart_dsrn(&self) -> UART_DSRN_R {
        UART_DSRN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - This register stores the value of receiver's finite state machine. 0:RX_IDLE 1:RX_STRT 2:RX_DAT0 3:RX_DAT1 4:RX_DAT2 5:RX_DAT3 6:RX_DAT4 7:RX_DAT5 8:RX_DAT6 9:RX_DAT7 10:RX_PRTY 11:RX_STP1 12:RX_STP2 13:RX_DL1"]
    #[inline(always)]
    pub fn uart_st_urx_out(&self) -> UART_ST_URX_OUT_R {
        UART_ST_URX_OUT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 0:7 - (rx_mem_cnt rxfifo_cnt) stores the byte num of valid datas in receiver's fifo. rx_mem_cnt register stores the 3 most significant bits rxfifo_cnt stores the 8 least significant bits."]
    #[inline(always)]
    pub fn uart_rxfifo_cnt(&self) -> UART_RXFIFO_CNT_R {
        UART_RXFIFO_CNT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - This register represent the level value of the internal uart rxd signal."]
    #[inline(always)]
    pub fn uart_txd(&mut self) -> UART_TXD_W {
        UART_TXD_W { w: self }
    }
    #[doc = "Bit 30 - This register represent the level value of the internal uart cts signal."]
    #[inline(always)]
    pub fn uart_rtsn(&mut self) -> UART_RTSN_W {
        UART_RTSN_W { w: self }
    }
    #[doc = "Bit 29 - The register represent the level value of the internal uart dsr signal."]
    #[inline(always)]
    pub fn uart_dtrn(&mut self) -> UART_DTRN_W {
        UART_DTRN_W { w: self }
    }
    #[doc = "Bits 24:27 - This register stores the value of transmitter's finite state machine. 0:TX_IDLE 1:TX_STRT 2:TX_DAT0 3:TX_DAT1 4:TX_DAT2 5:TX_DAT3 6:TX_DAT4 7:TX_DAT5 8:TX_DAT6 9:TX_DAT7 10:TX_PRTY 11:TX_STP1 12:TX_STP2 13:TX_DL0 14:TX_DL1"]
    #[inline(always)]
    pub fn uart_st_utx_out(&mut self) -> UART_ST_UTX_OUT_W {
        UART_ST_UTX_OUT_W { w: self }
    }
    #[doc = "Bits 16:23 - (tx_mem_cnt txfifo_cnt) stores the byte num of valid datas in transmitter's fifo.tx_mem_cnt stores the 3 most significant bits txfifo_cnt stores the 8 least significant bits."]
    #[inline(always)]
    pub fn uart_txfifo_cnt(&mut self) -> UART_TXFIFO_CNT_W {
        UART_TXFIFO_CNT_W { w: self }
    }
    #[doc = "Bit 15 - This register stores the level value of the internal uart rxd signal."]
    #[inline(always)]
    pub fn uart_rxd(&mut self) -> UART_RXD_W {
        UART_RXD_W { w: self }
    }
    #[doc = "Bit 14 - This register stores the level value of the internal uart cts signal."]
    #[inline(always)]
    pub fn uart_ctsn(&mut self) -> UART_CTSN_W {
        UART_CTSN_W { w: self }
    }
    #[doc = "Bit 13 - This register stores the level value of the internal uart dsr signal."]
    #[inline(always)]
    pub fn uart_dsrn(&mut self) -> UART_DSRN_W {
        UART_DSRN_W { w: self }
    }
    #[doc = "Bits 8:11 - This register stores the value of receiver's finite state machine. 0:RX_IDLE 1:RX_STRT 2:RX_DAT0 3:RX_DAT1 4:RX_DAT2 5:RX_DAT3 6:RX_DAT4 7:RX_DAT5 8:RX_DAT6 9:RX_DAT7 10:RX_PRTY 11:RX_STP1 12:RX_STP2 13:RX_DL1"]
    #[inline(always)]
    pub fn uart_st_urx_out(&mut self) -> UART_ST_URX_OUT_W {
        UART_ST_URX_OUT_W { w: self }
    }
    #[doc = "Bits 0:7 - (rx_mem_cnt rxfifo_cnt) stores the byte num of valid datas in receiver's fifo. rx_mem_cnt register stores the 3 most significant bits rxfifo_cnt stores the 8 least significant bits."]
    #[inline(always)]
    pub fn uart_rxfifo_cnt(&mut self) -> UART_RXFIFO_CNT_W {
        UART_RXFIFO_CNT_W { w: self }
    }
}
