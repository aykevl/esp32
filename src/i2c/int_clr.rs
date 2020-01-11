#[doc = "Reader of register INT_CLR"]
pub type R = crate::R<u32, super::INT_CLR>;
#[doc = "Writer for register INT_CLR"]
pub type W = crate::W<u32, super::INT_CLR>;
#[doc = "Register INT_CLR `reset()`'s with value 0"]
impl crate::ResetValue for super::INT_CLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2C_TX_SEND_EMPTY_INT_CLR`"]
pub type I2C_TX_SEND_EMPTY_INT_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_TX_SEND_EMPTY_INT_CLR`"]
pub struct I2C_TX_SEND_EMPTY_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_TX_SEND_EMPTY_INT_CLR_W<'a> {
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
#[doc = "Reader of field `I2C_RX_REC_FULL_INT_CLR`"]
pub type I2C_RX_REC_FULL_INT_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_RX_REC_FULL_INT_CLR`"]
pub struct I2C_RX_REC_FULL_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_RX_REC_FULL_INT_CLR_W<'a> {
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
#[doc = "Reader of field `I2C_ACK_ERR_INT_CLR`"]
pub type I2C_ACK_ERR_INT_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_ACK_ERR_INT_CLR`"]
pub struct I2C_ACK_ERR_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_ACK_ERR_INT_CLR_W<'a> {
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
#[doc = "Reader of field `I2C_TRANS_START_INT_CLR`"]
pub type I2C_TRANS_START_INT_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_TRANS_START_INT_CLR`"]
pub struct I2C_TRANS_START_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_TRANS_START_INT_CLR_W<'a> {
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
#[doc = "Reader of field `I2C_TIME_OUT_INT_CLR`"]
pub type I2C_TIME_OUT_INT_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_TIME_OUT_INT_CLR`"]
pub struct I2C_TIME_OUT_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_TIME_OUT_INT_CLR_W<'a> {
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
#[doc = "Reader of field `I2C_TRANS_COMPLETE_INT_CLR`"]
pub type I2C_TRANS_COMPLETE_INT_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_TRANS_COMPLETE_INT_CLR`"]
pub struct I2C_TRANS_COMPLETE_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_TRANS_COMPLETE_INT_CLR_W<'a> {
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
#[doc = "Reader of field `I2C_MASTER_TRAN_COMP_INT_CLR`"]
pub type I2C_MASTER_TRAN_COMP_INT_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_MASTER_TRAN_COMP_INT_CLR`"]
pub struct I2C_MASTER_TRAN_COMP_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_MASTER_TRAN_COMP_INT_CLR_W<'a> {
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
#[doc = "Reader of field `I2C_ARBITRATION_LOST_INT_CLR`"]
pub type I2C_ARBITRATION_LOST_INT_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_ARBITRATION_LOST_INT_CLR`"]
pub struct I2C_ARBITRATION_LOST_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_ARBITRATION_LOST_INT_CLR_W<'a> {
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
#[doc = "Reader of field `I2C_SLAVE_TRAN_COMP_INT_CLR`"]
pub type I2C_SLAVE_TRAN_COMP_INT_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_SLAVE_TRAN_COMP_INT_CLR`"]
pub struct I2C_SLAVE_TRAN_COMP_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_SLAVE_TRAN_COMP_INT_CLR_W<'a> {
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
#[doc = "Reader of field `I2C_END_DETECT_INT_CLR`"]
pub type I2C_END_DETECT_INT_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_END_DETECT_INT_CLR`"]
pub struct I2C_END_DETECT_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_END_DETECT_INT_CLR_W<'a> {
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
#[doc = "Reader of field `I2C_RXFIFO_OVF_INT_CLR`"]
pub type I2C_RXFIFO_OVF_INT_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_RXFIFO_OVF_INT_CLR`"]
pub struct I2C_RXFIFO_OVF_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_RXFIFO_OVF_INT_CLR_W<'a> {
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
#[doc = "Reader of field `I2C_TXFIFO_EMPTY_INT_CLR`"]
pub type I2C_TXFIFO_EMPTY_INT_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_TXFIFO_EMPTY_INT_CLR`"]
pub struct I2C_TXFIFO_EMPTY_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_TXFIFO_EMPTY_INT_CLR_W<'a> {
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
#[doc = "Reader of field `I2C_RXFIFO_FULL_INT_CLR`"]
pub type I2C_RXFIFO_FULL_INT_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_RXFIFO_FULL_INT_CLR`"]
pub struct I2C_RXFIFO_FULL_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_RXFIFO_FULL_INT_CLR_W<'a> {
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
    #[doc = "Bit 12 - Set this bit to clear the tx_send_empty_int interrupt."]
    #[inline(always)]
    pub fn i2c_tx_send_empty_int_clr(&self) -> I2C_TX_SEND_EMPTY_INT_CLR_R {
        I2C_TX_SEND_EMPTY_INT_CLR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Set this bit to clear the rx_rec_full_int interrupt."]
    #[inline(always)]
    pub fn i2c_rx_rec_full_int_clr(&self) -> I2C_RX_REC_FULL_INT_CLR_R {
        I2C_RX_REC_FULL_INT_CLR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Set this bit to clear the ack_err_int interrupt."]
    #[inline(always)]
    pub fn i2c_ack_err_int_clr(&self) -> I2C_ACK_ERR_INT_CLR_R {
        I2C_ACK_ERR_INT_CLR_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Set this bit to clear the trans_start_int interrupt."]
    #[inline(always)]
    pub fn i2c_trans_start_int_clr(&self) -> I2C_TRANS_START_INT_CLR_R {
        I2C_TRANS_START_INT_CLR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Set this bit to clear the time_out_int interrupt."]
    #[inline(always)]
    pub fn i2c_time_out_int_clr(&self) -> I2C_TIME_OUT_INT_CLR_R {
        I2C_TIME_OUT_INT_CLR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Set this bit to clear the trans_complete_int interrupt."]
    #[inline(always)]
    pub fn i2c_trans_complete_int_clr(&self) -> I2C_TRANS_COMPLETE_INT_CLR_R {
        I2C_TRANS_COMPLETE_INT_CLR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Set this bit to clear the master_tran_comp interrupt."]
    #[inline(always)]
    pub fn i2c_master_tran_comp_int_clr(&self) -> I2C_MASTER_TRAN_COMP_INT_CLR_R {
        I2C_MASTER_TRAN_COMP_INT_CLR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Set this bit to clear the arbitration_lost_int interrupt."]
    #[inline(always)]
    pub fn i2c_arbitration_lost_int_clr(&self) -> I2C_ARBITRATION_LOST_INT_CLR_R {
        I2C_ARBITRATION_LOST_INT_CLR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Set this bit to clear the slave_tran_comp_int interrupt."]
    #[inline(always)]
    pub fn i2c_slave_tran_comp_int_clr(&self) -> I2C_SLAVE_TRAN_COMP_INT_CLR_R {
        I2C_SLAVE_TRAN_COMP_INT_CLR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Set this bit to clear the end_detect_int interrupt."]
    #[inline(always)]
    pub fn i2c_end_detect_int_clr(&self) -> I2C_END_DETECT_INT_CLR_R {
        I2C_END_DETECT_INT_CLR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Set this bit to clear the rxfifo_ovf_int interrupt."]
    #[inline(always)]
    pub fn i2c_rxfifo_ovf_int_clr(&self) -> I2C_RXFIFO_OVF_INT_CLR_R {
        I2C_RXFIFO_OVF_INT_CLR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Set this bit to clear the txfifo_empty_int interrupt."]
    #[inline(always)]
    pub fn i2c_txfifo_empty_int_clr(&self) -> I2C_TXFIFO_EMPTY_INT_CLR_R {
        I2C_TXFIFO_EMPTY_INT_CLR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Set this bit to clear the rxfifo_full_int interrupt."]
    #[inline(always)]
    pub fn i2c_rxfifo_full_int_clr(&self) -> I2C_RXFIFO_FULL_INT_CLR_R {
        I2C_RXFIFO_FULL_INT_CLR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - Set this bit to clear the tx_send_empty_int interrupt."]
    #[inline(always)]
    pub fn i2c_tx_send_empty_int_clr(&mut self) -> I2C_TX_SEND_EMPTY_INT_CLR_W {
        I2C_TX_SEND_EMPTY_INT_CLR_W { w: self }
    }
    #[doc = "Bit 11 - Set this bit to clear the rx_rec_full_int interrupt."]
    #[inline(always)]
    pub fn i2c_rx_rec_full_int_clr(&mut self) -> I2C_RX_REC_FULL_INT_CLR_W {
        I2C_RX_REC_FULL_INT_CLR_W { w: self }
    }
    #[doc = "Bit 10 - Set this bit to clear the ack_err_int interrupt."]
    #[inline(always)]
    pub fn i2c_ack_err_int_clr(&mut self) -> I2C_ACK_ERR_INT_CLR_W {
        I2C_ACK_ERR_INT_CLR_W { w: self }
    }
    #[doc = "Bit 9 - Set this bit to clear the trans_start_int interrupt."]
    #[inline(always)]
    pub fn i2c_trans_start_int_clr(&mut self) -> I2C_TRANS_START_INT_CLR_W {
        I2C_TRANS_START_INT_CLR_W { w: self }
    }
    #[doc = "Bit 8 - Set this bit to clear the time_out_int interrupt."]
    #[inline(always)]
    pub fn i2c_time_out_int_clr(&mut self) -> I2C_TIME_OUT_INT_CLR_W {
        I2C_TIME_OUT_INT_CLR_W { w: self }
    }
    #[doc = "Bit 7 - Set this bit to clear the trans_complete_int interrupt."]
    #[inline(always)]
    pub fn i2c_trans_complete_int_clr(&mut self) -> I2C_TRANS_COMPLETE_INT_CLR_W {
        I2C_TRANS_COMPLETE_INT_CLR_W { w: self }
    }
    #[doc = "Bit 6 - Set this bit to clear the master_tran_comp interrupt."]
    #[inline(always)]
    pub fn i2c_master_tran_comp_int_clr(&mut self) -> I2C_MASTER_TRAN_COMP_INT_CLR_W {
        I2C_MASTER_TRAN_COMP_INT_CLR_W { w: self }
    }
    #[doc = "Bit 5 - Set this bit to clear the arbitration_lost_int interrupt."]
    #[inline(always)]
    pub fn i2c_arbitration_lost_int_clr(&mut self) -> I2C_ARBITRATION_LOST_INT_CLR_W {
        I2C_ARBITRATION_LOST_INT_CLR_W { w: self }
    }
    #[doc = "Bit 4 - Set this bit to clear the slave_tran_comp_int interrupt."]
    #[inline(always)]
    pub fn i2c_slave_tran_comp_int_clr(&mut self) -> I2C_SLAVE_TRAN_COMP_INT_CLR_W {
        I2C_SLAVE_TRAN_COMP_INT_CLR_W { w: self }
    }
    #[doc = "Bit 3 - Set this bit to clear the end_detect_int interrupt."]
    #[inline(always)]
    pub fn i2c_end_detect_int_clr(&mut self) -> I2C_END_DETECT_INT_CLR_W {
        I2C_END_DETECT_INT_CLR_W { w: self }
    }
    #[doc = "Bit 2 - Set this bit to clear the rxfifo_ovf_int interrupt."]
    #[inline(always)]
    pub fn i2c_rxfifo_ovf_int_clr(&mut self) -> I2C_RXFIFO_OVF_INT_CLR_W {
        I2C_RXFIFO_OVF_INT_CLR_W { w: self }
    }
    #[doc = "Bit 1 - Set this bit to clear the txfifo_empty_int interrupt."]
    #[inline(always)]
    pub fn i2c_txfifo_empty_int_clr(&mut self) -> I2C_TXFIFO_EMPTY_INT_CLR_W {
        I2C_TXFIFO_EMPTY_INT_CLR_W { w: self }
    }
    #[doc = "Bit 0 - Set this bit to clear the rxfifo_full_int interrupt."]
    #[inline(always)]
    pub fn i2c_rxfifo_full_int_clr(&mut self) -> I2C_RXFIFO_FULL_INT_CLR_W {
        I2C_RXFIFO_FULL_INT_CLR_W { w: self }
    }
}
