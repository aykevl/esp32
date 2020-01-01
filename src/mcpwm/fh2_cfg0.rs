#[doc = "Reader of register FH2_CFG0"]
pub type R = crate::R<u32, super::FH2_CFG0>;
#[doc = "Writer for register FH2_CFG0"]
pub type W = crate::W<u32, super::FH2_CFG0>;
#[doc = "Register FH2_CFG0 `reset()`'s with value 0"]
impl crate::ResetValue for super::FH2_CFG0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MCPWM_FH2_B_OST_U`"]
pub type MCPWM_FH2_B_OST_U_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MCPWM_FH2_B_OST_U`"]
pub struct MCPWM_FH2_B_OST_U_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_FH2_B_OST_U_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Reader of field `MCPWM_FH2_B_OST_D`"]
pub type MCPWM_FH2_B_OST_D_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MCPWM_FH2_B_OST_D`"]
pub struct MCPWM_FH2_B_OST_D_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_FH2_B_OST_D_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Reader of field `MCPWM_FH2_B_CBC_U`"]
pub type MCPWM_FH2_B_CBC_U_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MCPWM_FH2_B_CBC_U`"]
pub struct MCPWM_FH2_B_CBC_U_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_FH2_B_CBC_U_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Reader of field `MCPWM_FH2_B_CBC_D`"]
pub type MCPWM_FH2_B_CBC_D_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MCPWM_FH2_B_CBC_D`"]
pub struct MCPWM_FH2_B_CBC_D_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_FH2_B_CBC_D_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `MCPWM_FH2_A_OST_U`"]
pub type MCPWM_FH2_A_OST_U_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MCPWM_FH2_A_OST_U`"]
pub struct MCPWM_FH2_A_OST_U_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_FH2_A_OST_U_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `MCPWM_FH2_A_OST_D`"]
pub type MCPWM_FH2_A_OST_D_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MCPWM_FH2_A_OST_D`"]
pub struct MCPWM_FH2_A_OST_D_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_FH2_A_OST_D_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `MCPWM_FH2_A_CBC_U`"]
pub type MCPWM_FH2_A_CBC_U_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MCPWM_FH2_A_CBC_U`"]
pub struct MCPWM_FH2_A_CBC_U_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_FH2_A_CBC_U_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `MCPWM_FH2_A_CBC_D`"]
pub type MCPWM_FH2_A_CBC_D_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MCPWM_FH2_A_CBC_D`"]
pub struct MCPWM_FH2_A_CBC_D_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_FH2_A_CBC_D_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `MCPWM_FH2_F0_OST`"]
pub type MCPWM_FH2_F0_OST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_FH2_F0_OST`"]
pub struct MCPWM_FH2_F0_OST_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_FH2_F0_OST_W<'a> {
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
#[doc = "Reader of field `MCPWM_FH2_F1_OST`"]
pub type MCPWM_FH2_F1_OST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_FH2_F1_OST`"]
pub struct MCPWM_FH2_F1_OST_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_FH2_F1_OST_W<'a> {
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
#[doc = "Reader of field `MCPWM_FH2_F2_OST`"]
pub type MCPWM_FH2_F2_OST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_FH2_F2_OST`"]
pub struct MCPWM_FH2_F2_OST_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_FH2_F2_OST_W<'a> {
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
#[doc = "Reader of field `MCPWM_FH2_SW_OST`"]
pub type MCPWM_FH2_SW_OST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_FH2_SW_OST`"]
pub struct MCPWM_FH2_SW_OST_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_FH2_SW_OST_W<'a> {
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
#[doc = "Reader of field `MCPWM_FH2_F0_CBC`"]
pub type MCPWM_FH2_F0_CBC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_FH2_F0_CBC`"]
pub struct MCPWM_FH2_F0_CBC_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_FH2_F0_CBC_W<'a> {
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
#[doc = "Reader of field `MCPWM_FH2_F1_CBC`"]
pub type MCPWM_FH2_F1_CBC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_FH2_F1_CBC`"]
pub struct MCPWM_FH2_F1_CBC_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_FH2_F1_CBC_W<'a> {
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
#[doc = "Reader of field `MCPWM_FH2_F2_CBC`"]
pub type MCPWM_FH2_F2_CBC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_FH2_F2_CBC`"]
pub struct MCPWM_FH2_F2_CBC_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_FH2_F2_CBC_W<'a> {
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
#[doc = "Reader of field `MCPWM_FH2_SW_CBC`"]
pub type MCPWM_FH2_SW_CBC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_FH2_SW_CBC`"]
pub struct MCPWM_FH2_SW_CBC_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_FH2_SW_CBC_W<'a> {
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
    #[doc = "Bits 22:23 - One-shot mode action on PWM2B when fault event occurs and timer is increasing. 0: do nothing 1: force lo 2: force hi 3: toggle"]
    #[inline(always)]
    pub fn mcpwm_fh2_b_ost_u(&self) -> MCPWM_FH2_B_OST_U_R {
        MCPWM_FH2_B_OST_U_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - One-shot mode action on PWM2B when fault event occurs and timer is decreasing. 0: do nothing 1: force lo 2: force hi 3: toggle"]
    #[inline(always)]
    pub fn mcpwm_fh2_b_ost_d(&self) -> MCPWM_FH2_B_OST_D_R {
        MCPWM_FH2_B_OST_D_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Cycle-by-cycle mode action on PWM2B when fault event occurs and timer is increasing. 0: do nothing 1: force lo 2: force hi 3: toggle"]
    #[inline(always)]
    pub fn mcpwm_fh2_b_cbc_u(&self) -> MCPWM_FH2_B_CBC_U_R {
        MCPWM_FH2_B_CBC_U_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Cycle-by-cycle mode action on PWM2B when fault event occurs and timer is decreasing. 0: do nothing 1: force lo 2: force hi 3: toggle"]
    #[inline(always)]
    pub fn mcpwm_fh2_b_cbc_d(&self) -> MCPWM_FH2_B_CBC_D_R {
        MCPWM_FH2_B_CBC_D_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - One-shot mode action on PWM2A when fault event occurs and timer is increasing. 0: do nothing 1: force lo 2: force hi 3: toggle"]
    #[inline(always)]
    pub fn mcpwm_fh2_a_ost_u(&self) -> MCPWM_FH2_A_OST_U_R {
        MCPWM_FH2_A_OST_U_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - One-shot mode action on PWM2A when fault event occurs and timer is decreasing. 0: do nothing 1: force lo 2: force hi 3: toggle"]
    #[inline(always)]
    pub fn mcpwm_fh2_a_ost_d(&self) -> MCPWM_FH2_A_OST_D_R {
        MCPWM_FH2_A_OST_D_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Cycle-by-cycle mode action on PWM2A when fault event occurs and timer is increasing. 0: do nothing 1: force lo 2: force hi 3: toggle"]
    #[inline(always)]
    pub fn mcpwm_fh2_a_cbc_u(&self) -> MCPWM_FH2_A_CBC_U_R {
        MCPWM_FH2_A_CBC_U_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Cycle-by-cycle mode action on PWM2A when fault event occurs and timer is decreasing. 0: do nothing 1: force lo 2: force hi 3: toggle"]
    #[inline(always)]
    pub fn mcpwm_fh2_a_cbc_d(&self) -> MCPWM_FH2_A_CBC_D_R {
        MCPWM_FH2_A_CBC_D_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 7 - event_f0 will trigger one-shot mode action. 0: disable 1: enable"]
    #[inline(always)]
    pub fn mcpwm_fh2_f0_ost(&self) -> MCPWM_FH2_F0_OST_R {
        MCPWM_FH2_F0_OST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - event_f1 will trigger one-shot mode action. 0: disable 1: enable"]
    #[inline(always)]
    pub fn mcpwm_fh2_f1_ost(&self) -> MCPWM_FH2_F1_OST_R {
        MCPWM_FH2_F1_OST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - event_f2 will trigger one-shot mode action. 0: disable 1: enable"]
    #[inline(always)]
    pub fn mcpwm_fh2_f2_ost(&self) -> MCPWM_FH2_F2_OST_R {
        MCPWM_FH2_F2_OST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable register for software force one-shot mode action. 0: disable 1: enable"]
    #[inline(always)]
    pub fn mcpwm_fh2_sw_ost(&self) -> MCPWM_FH2_SW_OST_R {
        MCPWM_FH2_SW_OST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - event_f0 will trigger cycle-by-cycle mode action. 0: disable 1: enable"]
    #[inline(always)]
    pub fn mcpwm_fh2_f0_cbc(&self) -> MCPWM_FH2_F0_CBC_R {
        MCPWM_FH2_F0_CBC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - event_f1 will trigger cycle-by-cycle mode action. 0: disable 1: enable"]
    #[inline(always)]
    pub fn mcpwm_fh2_f1_cbc(&self) -> MCPWM_FH2_F1_CBC_R {
        MCPWM_FH2_F1_CBC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - event_f2 will trigger cycle-by-cycle mode action. 0: disable 1: enable"]
    #[inline(always)]
    pub fn mcpwm_fh2_f2_cbc(&self) -> MCPWM_FH2_F2_CBC_R {
        MCPWM_FH2_F2_CBC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Enable register for software force cycle-by-cycle mode action. 0: disable 1: enable"]
    #[inline(always)]
    pub fn mcpwm_fh2_sw_cbc(&self) -> MCPWM_FH2_SW_CBC_R {
        MCPWM_FH2_SW_CBC_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 22:23 - One-shot mode action on PWM2B when fault event occurs and timer is increasing. 0: do nothing 1: force lo 2: force hi 3: toggle"]
    #[inline(always)]
    pub fn mcpwm_fh2_b_ost_u(&mut self) -> MCPWM_FH2_B_OST_U_W {
        MCPWM_FH2_B_OST_U_W { w: self }
    }
    #[doc = "Bits 20:21 - One-shot mode action on PWM2B when fault event occurs and timer is decreasing. 0: do nothing 1: force lo 2: force hi 3: toggle"]
    #[inline(always)]
    pub fn mcpwm_fh2_b_ost_d(&mut self) -> MCPWM_FH2_B_OST_D_W {
        MCPWM_FH2_B_OST_D_W { w: self }
    }
    #[doc = "Bits 18:19 - Cycle-by-cycle mode action on PWM2B when fault event occurs and timer is increasing. 0: do nothing 1: force lo 2: force hi 3: toggle"]
    #[inline(always)]
    pub fn mcpwm_fh2_b_cbc_u(&mut self) -> MCPWM_FH2_B_CBC_U_W {
        MCPWM_FH2_B_CBC_U_W { w: self }
    }
    #[doc = "Bits 16:17 - Cycle-by-cycle mode action on PWM2B when fault event occurs and timer is decreasing. 0: do nothing 1: force lo 2: force hi 3: toggle"]
    #[inline(always)]
    pub fn mcpwm_fh2_b_cbc_d(&mut self) -> MCPWM_FH2_B_CBC_D_W {
        MCPWM_FH2_B_CBC_D_W { w: self }
    }
    #[doc = "Bits 14:15 - One-shot mode action on PWM2A when fault event occurs and timer is increasing. 0: do nothing 1: force lo 2: force hi 3: toggle"]
    #[inline(always)]
    pub fn mcpwm_fh2_a_ost_u(&mut self) -> MCPWM_FH2_A_OST_U_W {
        MCPWM_FH2_A_OST_U_W { w: self }
    }
    #[doc = "Bits 12:13 - One-shot mode action on PWM2A when fault event occurs and timer is decreasing. 0: do nothing 1: force lo 2: force hi 3: toggle"]
    #[inline(always)]
    pub fn mcpwm_fh2_a_ost_d(&mut self) -> MCPWM_FH2_A_OST_D_W {
        MCPWM_FH2_A_OST_D_W { w: self }
    }
    #[doc = "Bits 10:11 - Cycle-by-cycle mode action on PWM2A when fault event occurs and timer is increasing. 0: do nothing 1: force lo 2: force hi 3: toggle"]
    #[inline(always)]
    pub fn mcpwm_fh2_a_cbc_u(&mut self) -> MCPWM_FH2_A_CBC_U_W {
        MCPWM_FH2_A_CBC_U_W { w: self }
    }
    #[doc = "Bits 8:9 - Cycle-by-cycle mode action on PWM2A when fault event occurs and timer is decreasing. 0: do nothing 1: force lo 2: force hi 3: toggle"]
    #[inline(always)]
    pub fn mcpwm_fh2_a_cbc_d(&mut self) -> MCPWM_FH2_A_CBC_D_W {
        MCPWM_FH2_A_CBC_D_W { w: self }
    }
    #[doc = "Bit 7 - event_f0 will trigger one-shot mode action. 0: disable 1: enable"]
    #[inline(always)]
    pub fn mcpwm_fh2_f0_ost(&mut self) -> MCPWM_FH2_F0_OST_W {
        MCPWM_FH2_F0_OST_W { w: self }
    }
    #[doc = "Bit 6 - event_f1 will trigger one-shot mode action. 0: disable 1: enable"]
    #[inline(always)]
    pub fn mcpwm_fh2_f1_ost(&mut self) -> MCPWM_FH2_F1_OST_W {
        MCPWM_FH2_F1_OST_W { w: self }
    }
    #[doc = "Bit 5 - event_f2 will trigger one-shot mode action. 0: disable 1: enable"]
    #[inline(always)]
    pub fn mcpwm_fh2_f2_ost(&mut self) -> MCPWM_FH2_F2_OST_W {
        MCPWM_FH2_F2_OST_W { w: self }
    }
    #[doc = "Bit 4 - Enable register for software force one-shot mode action. 0: disable 1: enable"]
    #[inline(always)]
    pub fn mcpwm_fh2_sw_ost(&mut self) -> MCPWM_FH2_SW_OST_W {
        MCPWM_FH2_SW_OST_W { w: self }
    }
    #[doc = "Bit 3 - event_f0 will trigger cycle-by-cycle mode action. 0: disable 1: enable"]
    #[inline(always)]
    pub fn mcpwm_fh2_f0_cbc(&mut self) -> MCPWM_FH2_F0_CBC_W {
        MCPWM_FH2_F0_CBC_W { w: self }
    }
    #[doc = "Bit 2 - event_f1 will trigger cycle-by-cycle mode action. 0: disable 1: enable"]
    #[inline(always)]
    pub fn mcpwm_fh2_f1_cbc(&mut self) -> MCPWM_FH2_F1_CBC_W {
        MCPWM_FH2_F1_CBC_W { w: self }
    }
    #[doc = "Bit 1 - event_f2 will trigger cycle-by-cycle mode action. 0: disable 1: enable"]
    #[inline(always)]
    pub fn mcpwm_fh2_f2_cbc(&mut self) -> MCPWM_FH2_F2_CBC_W {
        MCPWM_FH2_F2_CBC_W { w: self }
    }
    #[doc = "Bit 0 - Enable register for software force cycle-by-cycle mode action. 0: disable 1: enable"]
    #[inline(always)]
    pub fn mcpwm_fh2_sw_cbc(&mut self) -> MCPWM_FH2_SW_CBC_W {
        MCPWM_FH2_SW_CBC_W { w: self }
    }
}
