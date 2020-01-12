#[doc = "Reader of register SAR_TSENS_CTRL"]
pub type R = crate::R<u32, super::SAR_TSENS_CTRL>;
#[doc = "Writer for register SAR_TSENS_CTRL"]
pub type W = crate::W<u32, super::SAR_TSENS_CTRL>;
#[doc = "Register SAR_TSENS_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::SAR_TSENS_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TSENS_DUMP_OUT`"]
pub type TSENS_DUMP_OUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSENS_DUMP_OUT`"]
pub struct TSENS_DUMP_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> TSENS_DUMP_OUT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `TSENS_POWER_UP_FORCE`"]
pub type TSENS_POWER_UP_FORCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSENS_POWER_UP_FORCE`"]
pub struct TSENS_POWER_UP_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> TSENS_POWER_UP_FORCE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `TSENS_POWER_UP`"]
pub type TSENS_POWER_UP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSENS_POWER_UP`"]
pub struct TSENS_POWER_UP_W<'a> {
    w: &'a mut W,
}
impl<'a> TSENS_POWER_UP_W<'a> {
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
#[doc = "Reader of field `TSENS_CLK_DIV`"]
pub type TSENS_CLK_DIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TSENS_CLK_DIV`"]
pub struct TSENS_CLK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> TSENS_CLK_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `TSENS_IN_INV`"]
pub type TSENS_IN_INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSENS_IN_INV`"]
pub struct TSENS_IN_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> TSENS_IN_INV_W<'a> {
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
#[doc = "Reader of field `TSENS_CLK_GATED`"]
pub type TSENS_CLK_GATED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSENS_CLK_GATED`"]
pub struct TSENS_CLK_GATED_W<'a> {
    w: &'a mut W,
}
impl<'a> TSENS_CLK_GATED_W<'a> {
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
#[doc = "Reader of field `TSENS_CLK_INV`"]
pub type TSENS_CLK_INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSENS_CLK_INV`"]
pub struct TSENS_CLK_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> TSENS_CLK_INV_W<'a> {
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
#[doc = "Reader of field `TSENS_XPD_FORCE`"]
pub type TSENS_XPD_FORCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSENS_XPD_FORCE`"]
pub struct TSENS_XPD_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> TSENS_XPD_FORCE_W<'a> {
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
#[doc = "Reader of field `TSENS_XPD_WAIT`"]
pub type TSENS_XPD_WAIT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TSENS_XPD_WAIT`"]
pub struct TSENS_XPD_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> TSENS_XPD_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bit 26 - temperature sensor dump out only active when reg_tsens_power_up_force = 1"]
    #[inline(always)]
    pub fn tsens_dump_out(&self) -> TSENS_DUMP_OUT_R {
        TSENS_DUMP_OUT_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - 1: dump out & power up controlled by SW 0: by FSM"]
    #[inline(always)]
    pub fn tsens_power_up_force(&self) -> TSENS_POWER_UP_FORCE_R {
        TSENS_POWER_UP_FORCE_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - temperature sensor power up"]
    #[inline(always)]
    pub fn tsens_power_up(&self) -> TSENS_POWER_UP_R {
        TSENS_POWER_UP_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - temperature sensor clock divider"]
    #[inline(always)]
    pub fn tsens_clk_div(&self) -> TSENS_CLK_DIV_R {
        TSENS_CLK_DIV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 15 - invert temperature sensor data"]
    #[inline(always)]
    pub fn tsens_in_inv(&self) -> TSENS_IN_INV_R {
        TSENS_IN_INV_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn tsens_clk_gated(&self) -> TSENS_CLK_GATED_R {
        TSENS_CLK_GATED_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn tsens_clk_inv(&self) -> TSENS_CLK_INV_R {
        TSENS_CLK_INV_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn tsens_xpd_force(&self) -> TSENS_XPD_FORCE_R {
        TSENS_XPD_FORCE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn tsens_xpd_wait(&self) -> TSENS_XPD_WAIT_R {
        TSENS_XPD_WAIT_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 26 - temperature sensor dump out only active when reg_tsens_power_up_force = 1"]
    #[inline(always)]
    pub fn tsens_dump_out(&mut self) -> TSENS_DUMP_OUT_W {
        TSENS_DUMP_OUT_W { w: self }
    }
    #[doc = "Bit 25 - 1: dump out & power up controlled by SW 0: by FSM"]
    #[inline(always)]
    pub fn tsens_power_up_force(&mut self) -> TSENS_POWER_UP_FORCE_W {
        TSENS_POWER_UP_FORCE_W { w: self }
    }
    #[doc = "Bit 24 - temperature sensor power up"]
    #[inline(always)]
    pub fn tsens_power_up(&mut self) -> TSENS_POWER_UP_W {
        TSENS_POWER_UP_W { w: self }
    }
    #[doc = "Bits 16:23 - temperature sensor clock divider"]
    #[inline(always)]
    pub fn tsens_clk_div(&mut self) -> TSENS_CLK_DIV_W {
        TSENS_CLK_DIV_W { w: self }
    }
    #[doc = "Bit 15 - invert temperature sensor data"]
    #[inline(always)]
    pub fn tsens_in_inv(&mut self) -> TSENS_IN_INV_W {
        TSENS_IN_INV_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn tsens_clk_gated(&mut self) -> TSENS_CLK_GATED_W {
        TSENS_CLK_GATED_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn tsens_clk_inv(&mut self) -> TSENS_CLK_INV_W {
        TSENS_CLK_INV_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn tsens_xpd_force(&mut self) -> TSENS_XPD_FORCE_W {
        TSENS_XPD_FORCE_W { w: self }
    }
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn tsens_xpd_wait(&mut self) -> TSENS_XPD_WAIT_W {
        TSENS_XPD_WAIT_W { w: self }
    }
}
