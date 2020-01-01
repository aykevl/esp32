#[doc = "Reader of register 1INT_RAW"]
pub type R = crate::R<u32, super::_1INT_RAW>;
#[doc = "Writer for register 1INT_RAW"]
pub type W = crate::W<u32, super::_1INT_RAW>;
#[doc = "Register 1INT_RAW `reset()`'s with value 0"]
impl crate::ResetValue for super::_1INT_RAW {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SLC_SLC1_TX_ERR_EOF_INT_RAW`"]
pub type SLC_SLC1_TX_ERR_EOF_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC1_TX_ERR_EOF_INT_RAW`"]
pub struct SLC_SLC1_TX_ERR_EOF_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC1_TX_ERR_EOF_INT_RAW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `SLC_SLC1_WR_RETRY_DONE_INT_RAW`"]
pub type SLC_SLC1_WR_RETRY_DONE_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC1_WR_RETRY_DONE_INT_RAW`"]
pub struct SLC_SLC1_WR_RETRY_DONE_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC1_WR_RETRY_DONE_INT_RAW_W<'a> {
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
#[doc = "Reader of field `SLC_SLC1_HOST_RD_ACK_INT_RAW`"]
pub type SLC_SLC1_HOST_RD_ACK_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC1_HOST_RD_ACK_INT_RAW`"]
pub struct SLC_SLC1_HOST_RD_ACK_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC1_HOST_RD_ACK_INT_RAW_W<'a> {
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
#[doc = "Reader of field `SLC_SLC1_TX_DSCR_EMPTY_INT_RAW`"]
pub type SLC_SLC1_TX_DSCR_EMPTY_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC1_TX_DSCR_EMPTY_INT_RAW`"]
pub struct SLC_SLC1_TX_DSCR_EMPTY_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC1_TX_DSCR_EMPTY_INT_RAW_W<'a> {
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
#[doc = "Reader of field `SLC_SLC1_RX_DSCR_ERR_INT_RAW`"]
pub type SLC_SLC1_RX_DSCR_ERR_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC1_RX_DSCR_ERR_INT_RAW`"]
pub struct SLC_SLC1_RX_DSCR_ERR_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC1_RX_DSCR_ERR_INT_RAW_W<'a> {
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
#[doc = "Reader of field `SLC_SLC1_TX_DSCR_ERR_INT_RAW`"]
pub type SLC_SLC1_TX_DSCR_ERR_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC1_TX_DSCR_ERR_INT_RAW`"]
pub struct SLC_SLC1_TX_DSCR_ERR_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC1_TX_DSCR_ERR_INT_RAW_W<'a> {
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
#[doc = "Reader of field `SLC_SLC1_TOHOST_INT_RAW`"]
pub type SLC_SLC1_TOHOST_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC1_TOHOST_INT_RAW`"]
pub struct SLC_SLC1_TOHOST_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC1_TOHOST_INT_RAW_W<'a> {
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
#[doc = "Reader of field `SLC_SLC1_RX_EOF_INT_RAW`"]
pub type SLC_SLC1_RX_EOF_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC1_RX_EOF_INT_RAW`"]
pub struct SLC_SLC1_RX_EOF_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC1_RX_EOF_INT_RAW_W<'a> {
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
#[doc = "Reader of field `SLC_SLC1_RX_DONE_INT_RAW`"]
pub type SLC_SLC1_RX_DONE_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC1_RX_DONE_INT_RAW`"]
pub struct SLC_SLC1_RX_DONE_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC1_RX_DONE_INT_RAW_W<'a> {
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
#[doc = "Reader of field `SLC_SLC1_TX_SUC_EOF_INT_RAW`"]
pub type SLC_SLC1_TX_SUC_EOF_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC1_TX_SUC_EOF_INT_RAW`"]
pub struct SLC_SLC1_TX_SUC_EOF_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC1_TX_SUC_EOF_INT_RAW_W<'a> {
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
#[doc = "Reader of field `SLC_SLC1_TX_DONE_INT_RAW`"]
pub type SLC_SLC1_TX_DONE_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC1_TX_DONE_INT_RAW`"]
pub struct SLC_SLC1_TX_DONE_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC1_TX_DONE_INT_RAW_W<'a> {
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
#[doc = "Reader of field `SLC_SLC1_TOKEN1_1TO0_INT_RAW`"]
pub type SLC_SLC1_TOKEN1_1TO0_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC1_TOKEN1_1TO0_INT_RAW`"]
pub struct SLC_SLC1_TOKEN1_1TO0_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC1_TOKEN1_1TO0_INT_RAW_W<'a> {
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
#[doc = "Reader of field `SLC_SLC1_TOKEN0_1TO0_INT_RAW`"]
pub type SLC_SLC1_TOKEN0_1TO0_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC1_TOKEN0_1TO0_INT_RAW`"]
pub struct SLC_SLC1_TOKEN0_1TO0_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC1_TOKEN0_1TO0_INT_RAW_W<'a> {
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
#[doc = "Reader of field `SLC_SLC1_TX_OVF_INT_RAW`"]
pub type SLC_SLC1_TX_OVF_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC1_TX_OVF_INT_RAW`"]
pub struct SLC_SLC1_TX_OVF_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC1_TX_OVF_INT_RAW_W<'a> {
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
#[doc = "Reader of field `SLC_SLC1_RX_UDF_INT_RAW`"]
pub type SLC_SLC1_RX_UDF_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC1_RX_UDF_INT_RAW`"]
pub struct SLC_SLC1_RX_UDF_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC1_RX_UDF_INT_RAW_W<'a> {
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
#[doc = "Reader of field `SLC_SLC1_TX_START_INT_RAW`"]
pub type SLC_SLC1_TX_START_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC1_TX_START_INT_RAW`"]
pub struct SLC_SLC1_TX_START_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC1_TX_START_INT_RAW_W<'a> {
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
#[doc = "Reader of field `SLC_SLC1_RX_START_INT_RAW`"]
pub type SLC_SLC1_RX_START_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC1_RX_START_INT_RAW`"]
pub struct SLC_SLC1_RX_START_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC1_RX_START_INT_RAW_W<'a> {
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
#[doc = "Reader of field `SLC_FRHOST_BIT15_INT_RAW`"]
pub type SLC_FRHOST_BIT15_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_FRHOST_BIT15_INT_RAW`"]
pub struct SLC_FRHOST_BIT15_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_FRHOST_BIT15_INT_RAW_W<'a> {
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
#[doc = "Reader of field `SLC_FRHOST_BIT14_INT_RAW`"]
pub type SLC_FRHOST_BIT14_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_FRHOST_BIT14_INT_RAW`"]
pub struct SLC_FRHOST_BIT14_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_FRHOST_BIT14_INT_RAW_W<'a> {
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
#[doc = "Reader of field `SLC_FRHOST_BIT13_INT_RAW`"]
pub type SLC_FRHOST_BIT13_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_FRHOST_BIT13_INT_RAW`"]
pub struct SLC_FRHOST_BIT13_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_FRHOST_BIT13_INT_RAW_W<'a> {
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
#[doc = "Reader of field `SLC_FRHOST_BIT12_INT_RAW`"]
pub type SLC_FRHOST_BIT12_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_FRHOST_BIT12_INT_RAW`"]
pub struct SLC_FRHOST_BIT12_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_FRHOST_BIT12_INT_RAW_W<'a> {
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
#[doc = "Reader of field `SLC_FRHOST_BIT11_INT_RAW`"]
pub type SLC_FRHOST_BIT11_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_FRHOST_BIT11_INT_RAW`"]
pub struct SLC_FRHOST_BIT11_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_FRHOST_BIT11_INT_RAW_W<'a> {
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
#[doc = "Reader of field `SLC_FRHOST_BIT10_INT_RAW`"]
pub type SLC_FRHOST_BIT10_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_FRHOST_BIT10_INT_RAW`"]
pub struct SLC_FRHOST_BIT10_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_FRHOST_BIT10_INT_RAW_W<'a> {
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
#[doc = "Reader of field `SLC_FRHOST_BIT9_INT_RAW`"]
pub type SLC_FRHOST_BIT9_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_FRHOST_BIT9_INT_RAW`"]
pub struct SLC_FRHOST_BIT9_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_FRHOST_BIT9_INT_RAW_W<'a> {
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
#[doc = "Reader of field `SLC_FRHOST_BIT8_INT_RAW`"]
pub type SLC_FRHOST_BIT8_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_FRHOST_BIT8_INT_RAW`"]
pub struct SLC_FRHOST_BIT8_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_FRHOST_BIT8_INT_RAW_W<'a> {
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
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn slc_slc1_tx_err_eof_int_raw(&self) -> SLC_SLC1_TX_ERR_EOF_INT_RAW_R {
        SLC_SLC1_TX_ERR_EOF_INT_RAW_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn slc_slc1_wr_retry_done_int_raw(&self) -> SLC_SLC1_WR_RETRY_DONE_INT_RAW_R {
        SLC_SLC1_WR_RETRY_DONE_INT_RAW_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn slc_slc1_host_rd_ack_int_raw(&self) -> SLC_SLC1_HOST_RD_ACK_INT_RAW_R {
        SLC_SLC1_HOST_RD_ACK_INT_RAW_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn slc_slc1_tx_dscr_empty_int_raw(&self) -> SLC_SLC1_TX_DSCR_EMPTY_INT_RAW_R {
        SLC_SLC1_TX_DSCR_EMPTY_INT_RAW_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn slc_slc1_rx_dscr_err_int_raw(&self) -> SLC_SLC1_RX_DSCR_ERR_INT_RAW_R {
        SLC_SLC1_RX_DSCR_ERR_INT_RAW_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn slc_slc1_tx_dscr_err_int_raw(&self) -> SLC_SLC1_TX_DSCR_ERR_INT_RAW_R {
        SLC_SLC1_TX_DSCR_ERR_INT_RAW_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn slc_slc1_tohost_int_raw(&self) -> SLC_SLC1_TOHOST_INT_RAW_R {
        SLC_SLC1_TOHOST_INT_RAW_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn slc_slc1_rx_eof_int_raw(&self) -> SLC_SLC1_RX_EOF_INT_RAW_R {
        SLC_SLC1_RX_EOF_INT_RAW_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slc_slc1_rx_done_int_raw(&self) -> SLC_SLC1_RX_DONE_INT_RAW_R {
        SLC_SLC1_RX_DONE_INT_RAW_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn slc_slc1_tx_suc_eof_int_raw(&self) -> SLC_SLC1_TX_SUC_EOF_INT_RAW_R {
        SLC_SLC1_TX_SUC_EOF_INT_RAW_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn slc_slc1_tx_done_int_raw(&self) -> SLC_SLC1_TX_DONE_INT_RAW_R {
        SLC_SLC1_TX_DONE_INT_RAW_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn slc_slc1_token1_1to0_int_raw(&self) -> SLC_SLC1_TOKEN1_1TO0_INT_RAW_R {
        SLC_SLC1_TOKEN1_1TO0_INT_RAW_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn slc_slc1_token0_1to0_int_raw(&self) -> SLC_SLC1_TOKEN0_1TO0_INT_RAW_R {
        SLC_SLC1_TOKEN0_1TO0_INT_RAW_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn slc_slc1_tx_ovf_int_raw(&self) -> SLC_SLC1_TX_OVF_INT_RAW_R {
        SLC_SLC1_TX_OVF_INT_RAW_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn slc_slc1_rx_udf_int_raw(&self) -> SLC_SLC1_RX_UDF_INT_RAW_R {
        SLC_SLC1_RX_UDF_INT_RAW_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn slc_slc1_tx_start_int_raw(&self) -> SLC_SLC1_TX_START_INT_RAW_R {
        SLC_SLC1_TX_START_INT_RAW_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn slc_slc1_rx_start_int_raw(&self) -> SLC_SLC1_RX_START_INT_RAW_R {
        SLC_SLC1_RX_START_INT_RAW_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn slc_frhost_bit15_int_raw(&self) -> SLC_FRHOST_BIT15_INT_RAW_R {
        SLC_FRHOST_BIT15_INT_RAW_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn slc_frhost_bit14_int_raw(&self) -> SLC_FRHOST_BIT14_INT_RAW_R {
        SLC_FRHOST_BIT14_INT_RAW_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn slc_frhost_bit13_int_raw(&self) -> SLC_FRHOST_BIT13_INT_RAW_R {
        SLC_FRHOST_BIT13_INT_RAW_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn slc_frhost_bit12_int_raw(&self) -> SLC_FRHOST_BIT12_INT_RAW_R {
        SLC_FRHOST_BIT12_INT_RAW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn slc_frhost_bit11_int_raw(&self) -> SLC_FRHOST_BIT11_INT_RAW_R {
        SLC_FRHOST_BIT11_INT_RAW_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn slc_frhost_bit10_int_raw(&self) -> SLC_FRHOST_BIT10_INT_RAW_R {
        SLC_FRHOST_BIT10_INT_RAW_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn slc_frhost_bit9_int_raw(&self) -> SLC_FRHOST_BIT9_INT_RAW_R {
        SLC_FRHOST_BIT9_INT_RAW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn slc_frhost_bit8_int_raw(&self) -> SLC_FRHOST_BIT8_INT_RAW_R {
        SLC_FRHOST_BIT8_INT_RAW_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn slc_slc1_tx_err_eof_int_raw(&mut self) -> SLC_SLC1_TX_ERR_EOF_INT_RAW_W {
        SLC_SLC1_TX_ERR_EOF_INT_RAW_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn slc_slc1_wr_retry_done_int_raw(&mut self) -> SLC_SLC1_WR_RETRY_DONE_INT_RAW_W {
        SLC_SLC1_WR_RETRY_DONE_INT_RAW_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn slc_slc1_host_rd_ack_int_raw(&mut self) -> SLC_SLC1_HOST_RD_ACK_INT_RAW_W {
        SLC_SLC1_HOST_RD_ACK_INT_RAW_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn slc_slc1_tx_dscr_empty_int_raw(&mut self) -> SLC_SLC1_TX_DSCR_EMPTY_INT_RAW_W {
        SLC_SLC1_TX_DSCR_EMPTY_INT_RAW_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn slc_slc1_rx_dscr_err_int_raw(&mut self) -> SLC_SLC1_RX_DSCR_ERR_INT_RAW_W {
        SLC_SLC1_RX_DSCR_ERR_INT_RAW_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn slc_slc1_tx_dscr_err_int_raw(&mut self) -> SLC_SLC1_TX_DSCR_ERR_INT_RAW_W {
        SLC_SLC1_TX_DSCR_ERR_INT_RAW_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn slc_slc1_tohost_int_raw(&mut self) -> SLC_SLC1_TOHOST_INT_RAW_W {
        SLC_SLC1_TOHOST_INT_RAW_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn slc_slc1_rx_eof_int_raw(&mut self) -> SLC_SLC1_RX_EOF_INT_RAW_W {
        SLC_SLC1_RX_EOF_INT_RAW_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slc_slc1_rx_done_int_raw(&mut self) -> SLC_SLC1_RX_DONE_INT_RAW_W {
        SLC_SLC1_RX_DONE_INT_RAW_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn slc_slc1_tx_suc_eof_int_raw(&mut self) -> SLC_SLC1_TX_SUC_EOF_INT_RAW_W {
        SLC_SLC1_TX_SUC_EOF_INT_RAW_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn slc_slc1_tx_done_int_raw(&mut self) -> SLC_SLC1_TX_DONE_INT_RAW_W {
        SLC_SLC1_TX_DONE_INT_RAW_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn slc_slc1_token1_1to0_int_raw(&mut self) -> SLC_SLC1_TOKEN1_1TO0_INT_RAW_W {
        SLC_SLC1_TOKEN1_1TO0_INT_RAW_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn slc_slc1_token0_1to0_int_raw(&mut self) -> SLC_SLC1_TOKEN0_1TO0_INT_RAW_W {
        SLC_SLC1_TOKEN0_1TO0_INT_RAW_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn slc_slc1_tx_ovf_int_raw(&mut self) -> SLC_SLC1_TX_OVF_INT_RAW_W {
        SLC_SLC1_TX_OVF_INT_RAW_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn slc_slc1_rx_udf_int_raw(&mut self) -> SLC_SLC1_RX_UDF_INT_RAW_W {
        SLC_SLC1_RX_UDF_INT_RAW_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn slc_slc1_tx_start_int_raw(&mut self) -> SLC_SLC1_TX_START_INT_RAW_W {
        SLC_SLC1_TX_START_INT_RAW_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn slc_slc1_rx_start_int_raw(&mut self) -> SLC_SLC1_RX_START_INT_RAW_W {
        SLC_SLC1_RX_START_INT_RAW_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn slc_frhost_bit15_int_raw(&mut self) -> SLC_FRHOST_BIT15_INT_RAW_W {
        SLC_FRHOST_BIT15_INT_RAW_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn slc_frhost_bit14_int_raw(&mut self) -> SLC_FRHOST_BIT14_INT_RAW_W {
        SLC_FRHOST_BIT14_INT_RAW_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn slc_frhost_bit13_int_raw(&mut self) -> SLC_FRHOST_BIT13_INT_RAW_W {
        SLC_FRHOST_BIT13_INT_RAW_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn slc_frhost_bit12_int_raw(&mut self) -> SLC_FRHOST_BIT12_INT_RAW_W {
        SLC_FRHOST_BIT12_INT_RAW_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn slc_frhost_bit11_int_raw(&mut self) -> SLC_FRHOST_BIT11_INT_RAW_W {
        SLC_FRHOST_BIT11_INT_RAW_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn slc_frhost_bit10_int_raw(&mut self) -> SLC_FRHOST_BIT10_INT_RAW_W {
        SLC_FRHOST_BIT10_INT_RAW_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn slc_frhost_bit9_int_raw(&mut self) -> SLC_FRHOST_BIT9_INT_RAW_W {
        SLC_FRHOST_BIT9_INT_RAW_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn slc_frhost_bit8_int_raw(&mut self) -> SLC_FRHOST_BIT8_INT_RAW_W {
        SLC_FRHOST_BIT8_INT_RAW_W { w: self }
    }
}
