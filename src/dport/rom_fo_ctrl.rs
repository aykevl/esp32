#[doc = "Reader of register ROM_FO_CTRL"]
pub type R = crate::R<u32, super::ROM_FO_CTRL>;
#[doc = "Writer for register ROM_FO_CTRL"]
pub type W = crate::W<u32, super::ROM_FO_CTRL>;
#[doc = "Register ROM_FO_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::ROM_FO_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DPORT_SHARE_ROM_FO`"]
pub type DPORT_SHARE_ROM_FO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DPORT_SHARE_ROM_FO`"]
pub struct DPORT_SHARE_ROM_FO_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_SHARE_ROM_FO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 2)) | (((value as u32) & 0x3f) << 2);
        self.w
    }
}
#[doc = "Reader of field `DPORT_APP_ROM_FO`"]
pub type DPORT_APP_ROM_FO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPORT_APP_ROM_FO`"]
pub struct DPORT_APP_ROM_FO_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_APP_ROM_FO_W<'a> {
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
#[doc = "Reader of field `DPORT_PRO_ROM_FO`"]
pub type DPORT_PRO_ROM_FO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPORT_PRO_ROM_FO`"]
pub struct DPORT_PRO_ROM_FO_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_PRO_ROM_FO_W<'a> {
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
    #[doc = "Bits 2:7"]
    #[inline(always)]
    pub fn dport_share_rom_fo(&self) -> DPORT_SHARE_ROM_FO_R {
        DPORT_SHARE_ROM_FO_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dport_app_rom_fo(&self) -> DPORT_APP_ROM_FO_R {
        DPORT_APP_ROM_FO_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dport_pro_rom_fo(&self) -> DPORT_PRO_ROM_FO_R {
        DPORT_PRO_ROM_FO_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 2:7"]
    #[inline(always)]
    pub fn dport_share_rom_fo(&mut self) -> DPORT_SHARE_ROM_FO_W {
        DPORT_SHARE_ROM_FO_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dport_app_rom_fo(&mut self) -> DPORT_APP_ROM_FO_W {
        DPORT_APP_ROM_FO_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dport_pro_rom_fo(&mut self) -> DPORT_PRO_ROM_FO_W {
        DPORT_PRO_ROM_FO_W { w: self }
    }
}
