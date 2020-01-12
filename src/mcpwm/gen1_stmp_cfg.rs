#[doc = "Reader of register GEN1_STMP_CFG"]
pub type R = crate::R<u32, super::GEN1_STMP_CFG>;
#[doc = "Writer for register GEN1_STMP_CFG"]
pub type W = crate::W<u32, super::GEN1_STMP_CFG>;
#[doc = "Register GEN1_STMP_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::GEN1_STMP_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GEN1_B_SHDW_FULL`"]
pub type GEN1_B_SHDW_FULL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GEN1_B_SHDW_FULL`"]
pub struct GEN1_B_SHDW_FULL_W<'a> {
    w: &'a mut W,
}
impl<'a> GEN1_B_SHDW_FULL_W<'a> {
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
#[doc = "Reader of field `GEN1_A_SHDW_FULL`"]
pub type GEN1_A_SHDW_FULL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GEN1_A_SHDW_FULL`"]
pub struct GEN1_A_SHDW_FULL_W<'a> {
    w: &'a mut W,
}
impl<'a> GEN1_A_SHDW_FULL_W<'a> {
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
#[doc = "Reader of field `GEN1_B_UPMETHOD`"]
pub type GEN1_B_UPMETHOD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GEN1_B_UPMETHOD`"]
pub struct GEN1_B_UPMETHOD_W<'a> {
    w: &'a mut W,
}
impl<'a> GEN1_B_UPMETHOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `GEN1_A_UPMETHOD`"]
pub type GEN1_A_UPMETHOD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GEN1_A_UPMETHOD`"]
pub struct GEN1_A_UPMETHOD_W<'a> {
    w: &'a mut W,
}
impl<'a> GEN1_A_UPMETHOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bit 9 - Set and reset by hardware. If set PWM generator 1 time stamp B's shadow reg is filled and waiting to be transferred to B's active reg. If cleared B's active reg has been updated with shadow reg latest value"]
    #[inline(always)]
    pub fn gen1_b_shdw_full(&self) -> GEN1_B_SHDW_FULL_R {
        GEN1_B_SHDW_FULL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Set and reset by hardware. If set PWM generator 1 time stamp A's shadow reg is filled and waiting to be transferred to A's active reg. If cleared A's active reg has been updated with shadow reg latest value"]
    #[inline(always)]
    pub fn gen1_a_shdw_full(&self) -> GEN1_A_SHDW_FULL_R {
        GEN1_A_SHDW_FULL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - Update method for PWM generator 1 time stamp B's active reg. 0: immediate bit0: TEZ bit1: TEP bit2: sync bit3: disable update"]
    #[inline(always)]
    pub fn gen1_b_upmethod(&self) -> GEN1_B_UPMETHOD_R {
        GEN1_B_UPMETHOD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - Update method for PWM generator 1 time stamp A's active reg. 0: immediate bit0: TEZ bit1: TEP bit2: sync bit3: disable update"]
    #[inline(always)]
    pub fn gen1_a_upmethod(&self) -> GEN1_A_UPMETHOD_R {
        GEN1_A_UPMETHOD_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 9 - Set and reset by hardware. If set PWM generator 1 time stamp B's shadow reg is filled and waiting to be transferred to B's active reg. If cleared B's active reg has been updated with shadow reg latest value"]
    #[inline(always)]
    pub fn gen1_b_shdw_full(&mut self) -> GEN1_B_SHDW_FULL_W {
        GEN1_B_SHDW_FULL_W { w: self }
    }
    #[doc = "Bit 8 - Set and reset by hardware. If set PWM generator 1 time stamp A's shadow reg is filled and waiting to be transferred to A's active reg. If cleared A's active reg has been updated with shadow reg latest value"]
    #[inline(always)]
    pub fn gen1_a_shdw_full(&mut self) -> GEN1_A_SHDW_FULL_W {
        GEN1_A_SHDW_FULL_W { w: self }
    }
    #[doc = "Bits 4:7 - Update method for PWM generator 1 time stamp B's active reg. 0: immediate bit0: TEZ bit1: TEP bit2: sync bit3: disable update"]
    #[inline(always)]
    pub fn gen1_b_upmethod(&mut self) -> GEN1_B_UPMETHOD_W {
        GEN1_B_UPMETHOD_W { w: self }
    }
    #[doc = "Bits 0:3 - Update method for PWM generator 1 time stamp A's active reg. 0: immediate bit0: TEZ bit1: TEP bit2: sync bit3: disable update"]
    #[inline(always)]
    pub fn gen1_a_upmethod(&mut self) -> GEN1_A_UPMETHOD_W {
        GEN1_A_UPMETHOD_W { w: self }
    }
}
