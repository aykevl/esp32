#[doc = "Reader of register DMA_INT_ENA"]
pub type R = crate::R<u32, super::DMA_INT_ENA>;
#[doc = "Writer for register DMA_INT_ENA"]
pub type W = crate::W<u32, super::DMA_INT_ENA>;
#[doc = "Register DMA_INT_ENA `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_INT_ENA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI_OUT_TOTAL_EOF_INT_ENA`"]
pub type SPI_OUT_TOTAL_EOF_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_OUT_TOTAL_EOF_INT_ENA`"]
pub struct SPI_OUT_TOTAL_EOF_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_OUT_TOTAL_EOF_INT_ENA_W<'a> {
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
#[doc = "Reader of field `SPI_OUT_EOF_INT_ENA`"]
pub type SPI_OUT_EOF_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_OUT_EOF_INT_ENA`"]
pub struct SPI_OUT_EOF_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_OUT_EOF_INT_ENA_W<'a> {
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
#[doc = "Reader of field `SPI_OUT_DONE_INT_ENA`"]
pub type SPI_OUT_DONE_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_OUT_DONE_INT_ENA`"]
pub struct SPI_OUT_DONE_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_OUT_DONE_INT_ENA_W<'a> {
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
#[doc = "Reader of field `SPI_IN_SUC_EOF_INT_ENA`"]
pub type SPI_IN_SUC_EOF_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_IN_SUC_EOF_INT_ENA`"]
pub struct SPI_IN_SUC_EOF_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_IN_SUC_EOF_INT_ENA_W<'a> {
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
#[doc = "Reader of field `SPI_IN_ERR_EOF_INT_ENA`"]
pub type SPI_IN_ERR_EOF_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_IN_ERR_EOF_INT_ENA`"]
pub struct SPI_IN_ERR_EOF_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_IN_ERR_EOF_INT_ENA_W<'a> {
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
#[doc = "Reader of field `SPI_IN_DONE_INT_ENA`"]
pub type SPI_IN_DONE_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_IN_DONE_INT_ENA`"]
pub struct SPI_IN_DONE_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_IN_DONE_INT_ENA_W<'a> {
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
#[doc = "Reader of field `SPI_INLINK_DSCR_ERROR_INT_ENA`"]
pub type SPI_INLINK_DSCR_ERROR_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_INLINK_DSCR_ERROR_INT_ENA`"]
pub struct SPI_INLINK_DSCR_ERROR_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_INLINK_DSCR_ERROR_INT_ENA_W<'a> {
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
#[doc = "Reader of field `SPI_OUTLINK_DSCR_ERROR_INT_ENA`"]
pub type SPI_OUTLINK_DSCR_ERROR_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_OUTLINK_DSCR_ERROR_INT_ENA`"]
pub struct SPI_OUTLINK_DSCR_ERROR_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_OUTLINK_DSCR_ERROR_INT_ENA_W<'a> {
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
#[doc = "Reader of field `SPI_INLINK_DSCR_EMPTY_INT_ENA`"]
pub type SPI_INLINK_DSCR_EMPTY_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_INLINK_DSCR_EMPTY_INT_ENA`"]
pub struct SPI_INLINK_DSCR_EMPTY_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_INLINK_DSCR_EMPTY_INT_ENA_W<'a> {
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
    #[doc = "Bit 8 - The enable bit for sending all the packets to host done."]
    #[inline(always)]
    pub fn spi_out_total_eof_int_ena(&self) -> SPI_OUT_TOTAL_EOF_INT_ENA_R {
        SPI_OUT_TOTAL_EOF_INT_ENA_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - The enable bit for sending a packet to host done."]
    #[inline(always)]
    pub fn spi_out_eof_int_ena(&self) -> SPI_OUT_EOF_INT_ENA_R {
        SPI_OUT_EOF_INT_ENA_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - The enable bit for completing usage of a outlink descriptor ."]
    #[inline(always)]
    pub fn spi_out_done_int_ena(&self) -> SPI_OUT_DONE_INT_ENA_R {
        SPI_OUT_DONE_INT_ENA_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - The enable bit for completing receiving all the packets from host."]
    #[inline(always)]
    pub fn spi_in_suc_eof_int_ena(&self) -> SPI_IN_SUC_EOF_INT_ENA_R {
        SPI_IN_SUC_EOF_INT_ENA_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - The enable bit for receiving error."]
    #[inline(always)]
    pub fn spi_in_err_eof_int_ena(&self) -> SPI_IN_ERR_EOF_INT_ENA_R {
        SPI_IN_ERR_EOF_INT_ENA_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - The enable bit for completing usage of a inlink descriptor."]
    #[inline(always)]
    pub fn spi_in_done_int_ena(&self) -> SPI_IN_DONE_INT_ENA_R {
        SPI_IN_DONE_INT_ENA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - The enable bit for inlink descriptor error."]
    #[inline(always)]
    pub fn spi_inlink_dscr_error_int_ena(&self) -> SPI_INLINK_DSCR_ERROR_INT_ENA_R {
        SPI_INLINK_DSCR_ERROR_INT_ENA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - The enable bit for outlink descriptor error."]
    #[inline(always)]
    pub fn spi_outlink_dscr_error_int_ena(&self) -> SPI_OUTLINK_DSCR_ERROR_INT_ENA_R {
        SPI_OUTLINK_DSCR_ERROR_INT_ENA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - The enable bit for lack of enough inlink descriptors."]
    #[inline(always)]
    pub fn spi_inlink_dscr_empty_int_ena(&self) -> SPI_INLINK_DSCR_EMPTY_INT_ENA_R {
        SPI_INLINK_DSCR_EMPTY_INT_ENA_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - The enable bit for sending all the packets to host done."]
    #[inline(always)]
    pub fn spi_out_total_eof_int_ena(&mut self) -> SPI_OUT_TOTAL_EOF_INT_ENA_W {
        SPI_OUT_TOTAL_EOF_INT_ENA_W { w: self }
    }
    #[doc = "Bit 7 - The enable bit for sending a packet to host done."]
    #[inline(always)]
    pub fn spi_out_eof_int_ena(&mut self) -> SPI_OUT_EOF_INT_ENA_W {
        SPI_OUT_EOF_INT_ENA_W { w: self }
    }
    #[doc = "Bit 6 - The enable bit for completing usage of a outlink descriptor ."]
    #[inline(always)]
    pub fn spi_out_done_int_ena(&mut self) -> SPI_OUT_DONE_INT_ENA_W {
        SPI_OUT_DONE_INT_ENA_W { w: self }
    }
    #[doc = "Bit 5 - The enable bit for completing receiving all the packets from host."]
    #[inline(always)]
    pub fn spi_in_suc_eof_int_ena(&mut self) -> SPI_IN_SUC_EOF_INT_ENA_W {
        SPI_IN_SUC_EOF_INT_ENA_W { w: self }
    }
    #[doc = "Bit 4 - The enable bit for receiving error."]
    #[inline(always)]
    pub fn spi_in_err_eof_int_ena(&mut self) -> SPI_IN_ERR_EOF_INT_ENA_W {
        SPI_IN_ERR_EOF_INT_ENA_W { w: self }
    }
    #[doc = "Bit 3 - The enable bit for completing usage of a inlink descriptor."]
    #[inline(always)]
    pub fn spi_in_done_int_ena(&mut self) -> SPI_IN_DONE_INT_ENA_W {
        SPI_IN_DONE_INT_ENA_W { w: self }
    }
    #[doc = "Bit 2 - The enable bit for inlink descriptor error."]
    #[inline(always)]
    pub fn spi_inlink_dscr_error_int_ena(&mut self) -> SPI_INLINK_DSCR_ERROR_INT_ENA_W {
        SPI_INLINK_DSCR_ERROR_INT_ENA_W { w: self }
    }
    #[doc = "Bit 1 - The enable bit for outlink descriptor error."]
    #[inline(always)]
    pub fn spi_outlink_dscr_error_int_ena(&mut self) -> SPI_OUTLINK_DSCR_ERROR_INT_ENA_W {
        SPI_OUTLINK_DSCR_ERROR_INT_ENA_W { w: self }
    }
    #[doc = "Bit 0 - The enable bit for lack of enough inlink descriptors."]
    #[inline(always)]
    pub fn spi_inlink_dscr_empty_int_ena(&mut self) -> SPI_INLINK_DSCR_EMPTY_INT_ENA_W {
        SPI_INLINK_DSCR_EMPTY_INT_ENA_W { w: self }
    }
}
