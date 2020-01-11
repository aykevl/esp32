#[doc = "Reader of register UPDATE_CFG"]
pub type R = crate::R<u32, super::UPDATE_CFG>;
#[doc = "Writer for register UPDATE_CFG"]
pub type W = crate::W<u32, super::UPDATE_CFG>;
#[doc = "Register UPDATE_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::UPDATE_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MCPWM_OP2_FORCE_UP`"]
pub type MCPWM_OP2_FORCE_UP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_OP2_FORCE_UP`"]
pub struct MCPWM_OP2_FORCE_UP_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_OP2_FORCE_UP_W<'a> {
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
#[doc = "Reader of field `MCPWM_OP2_UP_EN`"]
pub type MCPWM_OP2_UP_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_OP2_UP_EN`"]
pub struct MCPWM_OP2_UP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_OP2_UP_EN_W<'a> {
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
#[doc = "Reader of field `MCPWM_OP1_FORCE_UP`"]
pub type MCPWM_OP1_FORCE_UP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_OP1_FORCE_UP`"]
pub struct MCPWM_OP1_FORCE_UP_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_OP1_FORCE_UP_W<'a> {
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
#[doc = "Reader of field `MCPWM_OP1_UP_EN`"]
pub type MCPWM_OP1_UP_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_OP1_UP_EN`"]
pub struct MCPWM_OP1_UP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_OP1_UP_EN_W<'a> {
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
#[doc = "Reader of field `MCPWM_OP0_FORCE_UP`"]
pub type MCPWM_OP0_FORCE_UP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_OP0_FORCE_UP`"]
pub struct MCPWM_OP0_FORCE_UP_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_OP0_FORCE_UP_W<'a> {
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
#[doc = "Reader of field `MCPWM_OP0_UP_EN`"]
pub type MCPWM_OP0_UP_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_OP0_UP_EN`"]
pub struct MCPWM_OP0_UP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_OP0_UP_EN_W<'a> {
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
#[doc = "Reader of field `MCPWM_GLOBAL_FORCE_UP`"]
pub type MCPWM_GLOBAL_FORCE_UP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_GLOBAL_FORCE_UP`"]
pub struct MCPWM_GLOBAL_FORCE_UP_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_GLOBAL_FORCE_UP_W<'a> {
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
#[doc = "Reader of field `MCPWM_GLOBAL_UP_EN`"]
pub type MCPWM_GLOBAL_UP_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_GLOBAL_UP_EN`"]
pub struct MCPWM_GLOBAL_UP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_GLOBAL_UP_EN_W<'a> {
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
    #[doc = "Bit 7 - A toggle (software negation of value of this bit) will trigger a forced update of active registers in PWM operator 2"]
    #[inline(always)]
    pub fn mcpwm_op2_force_up(&self) -> MCPWM_OP2_FORCE_UP_R {
        MCPWM_OP2_FORCE_UP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - When set and PWM_GLOBAL_UP_EN is set update of active registers in PWM operator 2 are enabled"]
    #[inline(always)]
    pub fn mcpwm_op2_up_en(&self) -> MCPWM_OP2_UP_EN_R {
        MCPWM_OP2_UP_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - A toggle (software negation of value of this bit) will trigger a forced update of active registers in PWM operator 1"]
    #[inline(always)]
    pub fn mcpwm_op1_force_up(&self) -> MCPWM_OP1_FORCE_UP_R {
        MCPWM_OP1_FORCE_UP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - When set and PWM_GLOBAL_UP_EN is set update of active registers in PWM operator 1 are enabled"]
    #[inline(always)]
    pub fn mcpwm_op1_up_en(&self) -> MCPWM_OP1_UP_EN_R {
        MCPWM_OP1_UP_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - A toggle (software negation of value of this bit) will trigger a forced update of active registers in PWM operator 0"]
    #[inline(always)]
    pub fn mcpwm_op0_force_up(&self) -> MCPWM_OP0_FORCE_UP_R {
        MCPWM_OP0_FORCE_UP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - When set and PWM_GLOBAL_UP_EN is set update of active registers in PWM operator 0 are enabled"]
    #[inline(always)]
    pub fn mcpwm_op0_up_en(&self) -> MCPWM_OP0_UP_EN_R {
        MCPWM_OP0_UP_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - A toggle (software negation of value of this bit) will trigger a forced update of all active registers in MCPWM module"]
    #[inline(always)]
    pub fn mcpwm_global_force_up(&self) -> MCPWM_GLOBAL_FORCE_UP_R {
        MCPWM_GLOBAL_FORCE_UP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - The global enable of update of all active registers in MCPWM module"]
    #[inline(always)]
    pub fn mcpwm_global_up_en(&self) -> MCPWM_GLOBAL_UP_EN_R {
        MCPWM_GLOBAL_UP_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - A toggle (software negation of value of this bit) will trigger a forced update of active registers in PWM operator 2"]
    #[inline(always)]
    pub fn mcpwm_op2_force_up(&mut self) -> MCPWM_OP2_FORCE_UP_W {
        MCPWM_OP2_FORCE_UP_W { w: self }
    }
    #[doc = "Bit 6 - When set and PWM_GLOBAL_UP_EN is set update of active registers in PWM operator 2 are enabled"]
    #[inline(always)]
    pub fn mcpwm_op2_up_en(&mut self) -> MCPWM_OP2_UP_EN_W {
        MCPWM_OP2_UP_EN_W { w: self }
    }
    #[doc = "Bit 5 - A toggle (software negation of value of this bit) will trigger a forced update of active registers in PWM operator 1"]
    #[inline(always)]
    pub fn mcpwm_op1_force_up(&mut self) -> MCPWM_OP1_FORCE_UP_W {
        MCPWM_OP1_FORCE_UP_W { w: self }
    }
    #[doc = "Bit 4 - When set and PWM_GLOBAL_UP_EN is set update of active registers in PWM operator 1 are enabled"]
    #[inline(always)]
    pub fn mcpwm_op1_up_en(&mut self) -> MCPWM_OP1_UP_EN_W {
        MCPWM_OP1_UP_EN_W { w: self }
    }
    #[doc = "Bit 3 - A toggle (software negation of value of this bit) will trigger a forced update of active registers in PWM operator 0"]
    #[inline(always)]
    pub fn mcpwm_op0_force_up(&mut self) -> MCPWM_OP0_FORCE_UP_W {
        MCPWM_OP0_FORCE_UP_W { w: self }
    }
    #[doc = "Bit 2 - When set and PWM_GLOBAL_UP_EN is set update of active registers in PWM operator 0 are enabled"]
    #[inline(always)]
    pub fn mcpwm_op0_up_en(&mut self) -> MCPWM_OP0_UP_EN_W {
        MCPWM_OP0_UP_EN_W { w: self }
    }
    #[doc = "Bit 1 - A toggle (software negation of value of this bit) will trigger a forced update of all active registers in MCPWM module"]
    #[inline(always)]
    pub fn mcpwm_global_force_up(&mut self) -> MCPWM_GLOBAL_FORCE_UP_W {
        MCPWM_GLOBAL_FORCE_UP_W { w: self }
    }
    #[doc = "Bit 0 - The global enable of update of all active registers in MCPWM module"]
    #[inline(always)]
    pub fn mcpwm_global_up_en(&mut self) -> MCPWM_GLOBAL_UP_EN_W {
        MCPWM_GLOBAL_UP_EN_W { w: self }
    }
}
