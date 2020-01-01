#[doc = "Reader of register CONF0"]
pub type R = crate::R<u32, super::CONF0>;
#[doc = "Writer for register CONF0"]
pub type W = crate::W<u32, super::CONF0>;
#[doc = "Register CONF0 `reset()`'s with value 0"]
impl crate::ResetValue for super::CONF0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UHCI_UART_RX_BRK_EOF_EN`"]
pub type UHCI_UART_RX_BRK_EOF_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_UART_RX_BRK_EOF_EN`"]
pub struct UHCI_UART_RX_BRK_EOF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_UART_RX_BRK_EOF_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `UHCI_CLK_EN`"]
pub type UHCI_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_CLK_EN`"]
pub struct UHCI_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_CLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `UHCI_ENCODE_CRC_EN`"]
pub type UHCI_ENCODE_CRC_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_ENCODE_CRC_EN`"]
pub struct UHCI_ENCODE_CRC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_ENCODE_CRC_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `UHCI_LEN_EOF_EN`"]
pub type UHCI_LEN_EOF_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_LEN_EOF_EN`"]
pub struct UHCI_LEN_EOF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_LEN_EOF_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `UHCI_UART_IDLE_EOF_EN`"]
pub type UHCI_UART_IDLE_EOF_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_UART_IDLE_EOF_EN`"]
pub struct UHCI_UART_IDLE_EOF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_UART_IDLE_EOF_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `UHCI_CRC_REC_EN`"]
pub type UHCI_CRC_REC_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_CRC_REC_EN`"]
pub struct UHCI_CRC_REC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_CRC_REC_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `UHCI_HEAD_EN`"]
pub type UHCI_HEAD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_HEAD_EN`"]
pub struct UHCI_HEAD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_HEAD_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `UHCI_SEPER_EN`"]
pub type UHCI_SEPER_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_SEPER_EN`"]
pub struct UHCI_SEPER_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_SEPER_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `UHCI_MEM_TRANS_EN`"]
pub type UHCI_MEM_TRANS_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_MEM_TRANS_EN`"]
pub struct UHCI_MEM_TRANS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_MEM_TRANS_EN_W<'a> {
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
#[doc = "Reader of field `UHCI_OUT_DATA_BURST_EN`"]
pub type UHCI_OUT_DATA_BURST_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_OUT_DATA_BURST_EN`"]
pub struct UHCI_OUT_DATA_BURST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_OUT_DATA_BURST_EN_W<'a> {
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
#[doc = "Reader of field `UHCI_INDSCR_BURST_EN`"]
pub type UHCI_INDSCR_BURST_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_INDSCR_BURST_EN`"]
pub struct UHCI_INDSCR_BURST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_INDSCR_BURST_EN_W<'a> {
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
#[doc = "Reader of field `UHCI_OUTDSCR_BURST_EN`"]
pub type UHCI_OUTDSCR_BURST_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_OUTDSCR_BURST_EN`"]
pub struct UHCI_OUTDSCR_BURST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_OUTDSCR_BURST_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `UHCI_UART2_CE`"]
pub type UHCI_UART2_CE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_UART2_CE`"]
pub struct UHCI_UART2_CE_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_UART2_CE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `UHCI_UART1_CE`"]
pub type UHCI_UART1_CE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_UART1_CE`"]
pub struct UHCI_UART1_CE_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_UART1_CE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `UHCI_UART0_CE`"]
pub type UHCI_UART0_CE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_UART0_CE`"]
pub struct UHCI_UART0_CE_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_UART0_CE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `UHCI_OUT_EOF_MODE`"]
pub type UHCI_OUT_EOF_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_OUT_EOF_MODE`"]
pub struct UHCI_OUT_EOF_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_OUT_EOF_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `UHCI_OUT_NO_RESTART_CLR`"]
pub type UHCI_OUT_NO_RESTART_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_OUT_NO_RESTART_CLR`"]
pub struct UHCI_OUT_NO_RESTART_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_OUT_NO_RESTART_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `UHCI_OUT_AUTO_WRBACK`"]
pub type UHCI_OUT_AUTO_WRBACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_OUT_AUTO_WRBACK`"]
pub struct UHCI_OUT_AUTO_WRBACK_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_OUT_AUTO_WRBACK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `UHCI_OUT_LOOP_TEST`"]
pub type UHCI_OUT_LOOP_TEST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_OUT_LOOP_TEST`"]
pub struct UHCI_OUT_LOOP_TEST_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_OUT_LOOP_TEST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `UHCI_IN_LOOP_TEST`"]
pub type UHCI_IN_LOOP_TEST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_IN_LOOP_TEST`"]
pub struct UHCI_IN_LOOP_TEST_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_IN_LOOP_TEST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `UHCI_AHBM_RST`"]
pub type UHCI_AHBM_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_AHBM_RST`"]
pub struct UHCI_AHBM_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_AHBM_RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `UHCI_AHBM_FIFO_RST`"]
pub type UHCI_AHBM_FIFO_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_AHBM_FIFO_RST`"]
pub struct UHCI_AHBM_FIFO_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_AHBM_FIFO_RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `UHCI_OUT_RST`"]
pub type UHCI_OUT_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_OUT_RST`"]
pub struct UHCI_OUT_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_OUT_RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `UHCI_IN_RST`"]
pub type UHCI_IN_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_IN_RST`"]
pub struct UHCI_IN_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_IN_RST_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 23 - Set this bit to enable to use brk char as the end of a data frame."]
    #[inline(always)]
    pub fn uhci_uart_rx_brk_eof_en(&self) -> UHCI_UART_RX_BRK_EOF_EN_R {
        UHCI_UART_RX_BRK_EOF_EN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Set this bit to enable clock-gating for read or write registers."]
    #[inline(always)]
    pub fn uhci_clk_en(&self) -> UHCI_CLK_EN_R {
        UHCI_CLK_EN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Set this bit to enable crc calculation for data frame when bit6 in the head packet is 1."]
    #[inline(always)]
    pub fn uhci_encode_crc_en(&self) -> UHCI_ENCODE_CRC_EN_R {
        UHCI_ENCODE_CRC_EN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Set this bit to enable to use packet_len in packet head when the received data is equal to packet_len this means the end of a data frame."]
    #[inline(always)]
    pub fn uhci_len_eof_en(&self) -> UHCI_LEN_EOF_EN_R {
        UHCI_LEN_EOF_EN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Set this bit to enable to use idle time when the idle time after data frame is satisfied this means the end of a data frame."]
    #[inline(always)]
    pub fn uhci_uart_idle_eof_en(&self) -> UHCI_UART_IDLE_EOF_EN_R {
        UHCI_UART_IDLE_EOF_EN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Set this bit to enable receiver''s ability of crc calculation when crc_en bit in head packet is 1 then there will be crc bytes after data_frame"]
    #[inline(always)]
    pub fn uhci_crc_rec_en(&self) -> UHCI_CRC_REC_EN_R {
        UHCI_CRC_REC_EN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Set this bit to enable to use head packet before the data frame."]
    #[inline(always)]
    pub fn uhci_head_en(&self) -> UHCI_HEAD_EN_R {
        UHCI_HEAD_EN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Set this bit to use special char to separate the data frame."]
    #[inline(always)]
    pub fn uhci_seper_en(&self) -> UHCI_SEPER_EN_R {
        UHCI_SEPER_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn uhci_mem_trans_en(&self) -> UHCI_MEM_TRANS_EN_R {
        UHCI_MEM_TRANS_EN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Set this bit to enable DMA burst MODE"]
    #[inline(always)]
    pub fn uhci_out_data_burst_en(&self) -> UHCI_OUT_DATA_BURST_EN_R {
        UHCI_OUT_DATA_BURST_EN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Set this bit to enable DMA out links to use burst mode."]
    #[inline(always)]
    pub fn uhci_indscr_burst_en(&self) -> UHCI_INDSCR_BURST_EN_R {
        UHCI_INDSCR_BURST_EN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Set this bit to enable DMA in links to use burst mode."]
    #[inline(always)]
    pub fn uhci_outdscr_burst_en(&self) -> UHCI_OUTDSCR_BURST_EN_R {
        UHCI_OUTDSCR_BURST_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Set this bit to use UART2 to transmit or receive data."]
    #[inline(always)]
    pub fn uhci_uart2_ce(&self) -> UHCI_UART2_CE_R {
        UHCI_UART2_CE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Set this bit to use UART1 to transmit or receive data."]
    #[inline(always)]
    pub fn uhci_uart1_ce(&self) -> UHCI_UART1_CE_R {
        UHCI_UART1_CE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Set this bit to use UART to transmit or receive data."]
    #[inline(always)]
    pub fn uhci_uart0_ce(&self) -> UHCI_UART0_CE_R {
        UHCI_UART0_CE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Set this bit to produce eof after DMA pops all data clear this bit to produce eof after DMA pushes all data"]
    #[inline(always)]
    pub fn uhci_out_eof_mode(&self) -> UHCI_OUT_EOF_MODE_R {
        UHCI_OUT_EOF_MODE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - don't use"]
    #[inline(always)]
    pub fn uhci_out_no_restart_clr(&self) -> UHCI_OUT_NO_RESTART_CLR_R {
        UHCI_OUT_NO_RESTART_CLR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - when in link's length is 0 go on to use the next in link automatically."]
    #[inline(always)]
    pub fn uhci_out_auto_wrback(&self) -> UHCI_OUT_AUTO_WRBACK_R {
        UHCI_OUT_AUTO_WRBACK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Set this bit to enable loop test for out links."]
    #[inline(always)]
    pub fn uhci_out_loop_test(&self) -> UHCI_OUT_LOOP_TEST_R {
        UHCI_OUT_LOOP_TEST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Set this bit to enable loop test for in links."]
    #[inline(always)]
    pub fn uhci_in_loop_test(&self) -> UHCI_IN_LOOP_TEST_R {
        UHCI_IN_LOOP_TEST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Set this bit to reset dma ahb interface."]
    #[inline(always)]
    pub fn uhci_ahbm_rst(&self) -> UHCI_AHBM_RST_R {
        UHCI_AHBM_RST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Set this bit to reset dma ahb fifo."]
    #[inline(always)]
    pub fn uhci_ahbm_fifo_rst(&self) -> UHCI_AHBM_FIFO_RST_R {
        UHCI_AHBM_FIFO_RST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Set this bit to reset out link operations."]
    #[inline(always)]
    pub fn uhci_out_rst(&self) -> UHCI_OUT_RST_R {
        UHCI_OUT_RST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Set this bit to reset in link operations."]
    #[inline(always)]
    pub fn uhci_in_rst(&self) -> UHCI_IN_RST_R {
        UHCI_IN_RST_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 23 - Set this bit to enable to use brk char as the end of a data frame."]
    #[inline(always)]
    pub fn uhci_uart_rx_brk_eof_en(&mut self) -> UHCI_UART_RX_BRK_EOF_EN_W {
        UHCI_UART_RX_BRK_EOF_EN_W { w: self }
    }
    #[doc = "Bit 22 - Set this bit to enable clock-gating for read or write registers."]
    #[inline(always)]
    pub fn uhci_clk_en(&mut self) -> UHCI_CLK_EN_W {
        UHCI_CLK_EN_W { w: self }
    }
    #[doc = "Bit 21 - Set this bit to enable crc calculation for data frame when bit6 in the head packet is 1."]
    #[inline(always)]
    pub fn uhci_encode_crc_en(&mut self) -> UHCI_ENCODE_CRC_EN_W {
        UHCI_ENCODE_CRC_EN_W { w: self }
    }
    #[doc = "Bit 20 - Set this bit to enable to use packet_len in packet head when the received data is equal to packet_len this means the end of a data frame."]
    #[inline(always)]
    pub fn uhci_len_eof_en(&mut self) -> UHCI_LEN_EOF_EN_W {
        UHCI_LEN_EOF_EN_W { w: self }
    }
    #[doc = "Bit 19 - Set this bit to enable to use idle time when the idle time after data frame is satisfied this means the end of a data frame."]
    #[inline(always)]
    pub fn uhci_uart_idle_eof_en(&mut self) -> UHCI_UART_IDLE_EOF_EN_W {
        UHCI_UART_IDLE_EOF_EN_W { w: self }
    }
    #[doc = "Bit 18 - Set this bit to enable receiver''s ability of crc calculation when crc_en bit in head packet is 1 then there will be crc bytes after data_frame"]
    #[inline(always)]
    pub fn uhci_crc_rec_en(&mut self) -> UHCI_CRC_REC_EN_W {
        UHCI_CRC_REC_EN_W { w: self }
    }
    #[doc = "Bit 17 - Set this bit to enable to use head packet before the data frame."]
    #[inline(always)]
    pub fn uhci_head_en(&mut self) -> UHCI_HEAD_EN_W {
        UHCI_HEAD_EN_W { w: self }
    }
    #[doc = "Bit 16 - Set this bit to use special char to separate the data frame."]
    #[inline(always)]
    pub fn uhci_seper_en(&mut self) -> UHCI_SEPER_EN_W {
        UHCI_SEPER_EN_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn uhci_mem_trans_en(&mut self) -> UHCI_MEM_TRANS_EN_W {
        UHCI_MEM_TRANS_EN_W { w: self }
    }
    #[doc = "Bit 14 - Set this bit to enable DMA burst MODE"]
    #[inline(always)]
    pub fn uhci_out_data_burst_en(&mut self) -> UHCI_OUT_DATA_BURST_EN_W {
        UHCI_OUT_DATA_BURST_EN_W { w: self }
    }
    #[doc = "Bit 13 - Set this bit to enable DMA out links to use burst mode."]
    #[inline(always)]
    pub fn uhci_indscr_burst_en(&mut self) -> UHCI_INDSCR_BURST_EN_W {
        UHCI_INDSCR_BURST_EN_W { w: self }
    }
    #[doc = "Bit 12 - Set this bit to enable DMA in links to use burst mode."]
    #[inline(always)]
    pub fn uhci_outdscr_burst_en(&mut self) -> UHCI_OUTDSCR_BURST_EN_W {
        UHCI_OUTDSCR_BURST_EN_W { w: self }
    }
    #[doc = "Bit 11 - Set this bit to use UART2 to transmit or receive data."]
    #[inline(always)]
    pub fn uhci_uart2_ce(&mut self) -> UHCI_UART2_CE_W {
        UHCI_UART2_CE_W { w: self }
    }
    #[doc = "Bit 10 - Set this bit to use UART1 to transmit or receive data."]
    #[inline(always)]
    pub fn uhci_uart1_ce(&mut self) -> UHCI_UART1_CE_W {
        UHCI_UART1_CE_W { w: self }
    }
    #[doc = "Bit 9 - Set this bit to use UART to transmit or receive data."]
    #[inline(always)]
    pub fn uhci_uart0_ce(&mut self) -> UHCI_UART0_CE_W {
        UHCI_UART0_CE_W { w: self }
    }
    #[doc = "Bit 8 - Set this bit to produce eof after DMA pops all data clear this bit to produce eof after DMA pushes all data"]
    #[inline(always)]
    pub fn uhci_out_eof_mode(&mut self) -> UHCI_OUT_EOF_MODE_W {
        UHCI_OUT_EOF_MODE_W { w: self }
    }
    #[doc = "Bit 7 - don't use"]
    #[inline(always)]
    pub fn uhci_out_no_restart_clr(&mut self) -> UHCI_OUT_NO_RESTART_CLR_W {
        UHCI_OUT_NO_RESTART_CLR_W { w: self }
    }
    #[doc = "Bit 6 - when in link's length is 0 go on to use the next in link automatically."]
    #[inline(always)]
    pub fn uhci_out_auto_wrback(&mut self) -> UHCI_OUT_AUTO_WRBACK_W {
        UHCI_OUT_AUTO_WRBACK_W { w: self }
    }
    #[doc = "Bit 5 - Set this bit to enable loop test for out links."]
    #[inline(always)]
    pub fn uhci_out_loop_test(&mut self) -> UHCI_OUT_LOOP_TEST_W {
        UHCI_OUT_LOOP_TEST_W { w: self }
    }
    #[doc = "Bit 4 - Set this bit to enable loop test for in links."]
    #[inline(always)]
    pub fn uhci_in_loop_test(&mut self) -> UHCI_IN_LOOP_TEST_W {
        UHCI_IN_LOOP_TEST_W { w: self }
    }
    #[doc = "Bit 3 - Set this bit to reset dma ahb interface."]
    #[inline(always)]
    pub fn uhci_ahbm_rst(&mut self) -> UHCI_AHBM_RST_W {
        UHCI_AHBM_RST_W { w: self }
    }
    #[doc = "Bit 2 - Set this bit to reset dma ahb fifo."]
    #[inline(always)]
    pub fn uhci_ahbm_fifo_rst(&mut self) -> UHCI_AHBM_FIFO_RST_W {
        UHCI_AHBM_FIFO_RST_W { w: self }
    }
    #[doc = "Bit 1 - Set this bit to reset out link operations."]
    #[inline(always)]
    pub fn uhci_out_rst(&mut self) -> UHCI_OUT_RST_W {
        UHCI_OUT_RST_W { w: self }
    }
    #[doc = "Bit 0 - Set this bit to reset in link operations."]
    #[inline(always)]
    pub fn uhci_in_rst(&mut self) -> UHCI_IN_RST_W {
        UHCI_IN_RST_W { w: self }
    }
}
