#[doc = "Reader of register HOST_SLC0HOST_TOKEN_RDATA"]
pub type R = crate::R<u32, super::HOST_SLC0HOST_TOKEN_RDATA>;
#[doc = "Writer for register HOST_SLC0HOST_TOKEN_RDATA"]
pub type W = crate::W<u32, super::HOST_SLC0HOST_TOKEN_RDATA>;
#[doc = "Register HOST_SLC0HOST_TOKEN_RDATA `reset()`'s with value 0"]
impl crate::ResetValue for super::HOST_SLC0HOST_TOKEN_RDATA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HOST_SLC0_RX_PF_EOF`"]
pub type HOST_SLC0_RX_PF_EOF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOST_SLC0_RX_PF_EOF`"]
pub struct HOST_SLC0_RX_PF_EOF_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC0_RX_PF_EOF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
#[doc = "Reader of field `HOST_HOSTSLC0_TOKEN1`"]
pub type HOST_HOSTSLC0_TOKEN1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `HOST_HOSTSLC0_TOKEN1`"]
pub struct HOST_HOSTSLC0_TOKEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_HOSTSLC0_TOKEN1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | (((value as u32) & 0x0fff) << 16);
        self.w
    }
}
#[doc = "Reader of field `HOST_SLC0_RX_PF_VALID`"]
pub type HOST_SLC0_RX_PF_VALID_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HOST_SLC0_RX_PF_VALID`"]
pub struct HOST_SLC0_RX_PF_VALID_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC0_RX_PF_VALID_W<'a> {
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
#[doc = "Reader of field `HOST_SLC0_TOKEN0`"]
pub type HOST_SLC0_TOKEN0_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `HOST_SLC0_TOKEN0`"]
pub struct HOST_SLC0_TOKEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC0_TOKEN0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn host_slc0_rx_pf_eof(&self) -> HOST_SLC0_RX_PF_EOF_R {
        HOST_SLC0_RX_PF_EOF_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn host_hostslc0_token1(&self) -> HOST_HOSTSLC0_TOKEN1_R {
        HOST_HOSTSLC0_TOKEN1_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn host_slc0_rx_pf_valid(&self) -> HOST_SLC0_RX_PF_VALID_R {
        HOST_SLC0_RX_PF_VALID_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn host_slc0_token0(&self) -> HOST_SLC0_TOKEN0_R {
        HOST_SLC0_TOKEN0_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn host_slc0_rx_pf_eof(&mut self) -> HOST_SLC0_RX_PF_EOF_W {
        HOST_SLC0_RX_PF_EOF_W { w: self }
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn host_hostslc0_token1(&mut self) -> HOST_HOSTSLC0_TOKEN1_W {
        HOST_HOSTSLC0_TOKEN1_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn host_slc0_rx_pf_valid(&mut self) -> HOST_SLC0_RX_PF_VALID_W {
        HOST_SLC0_RX_PF_VALID_W { w: self }
    }
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn host_slc0_token0(&mut self) -> HOST_SLC0_TOKEN0_W {
        HOST_SLC0_TOKEN0_W { w: self }
    }
}
