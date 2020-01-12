#[doc = "Reader of register FH2_STATUS"]
pub type R = crate::R<u32, super::FH2_STATUS>;
#[doc = "Writer for register FH2_STATUS"]
pub type W = crate::W<u32, super::FH2_STATUS>;
#[doc = "Register FH2_STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::FH2_STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FH2_OST_ON`"]
pub type FH2_OST_ON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FH2_OST_ON`"]
pub struct FH2_OST_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> FH2_OST_ON_W<'a> {
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
#[doc = "Reader of field `FH2_CBC_ON`"]
pub type FH2_CBC_ON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FH2_CBC_ON`"]
pub struct FH2_CBC_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> FH2_CBC_ON_W<'a> {
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
    #[doc = "Bit 1 - Set and reset by hardware. If set an one-shot mode action is on going"]
    #[inline(always)]
    pub fn fh2_ost_on(&self) -> FH2_OST_ON_R {
        FH2_OST_ON_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Set and reset by hardware. If set an cycle-by-cycle mode action is on going"]
    #[inline(always)]
    pub fn fh2_cbc_on(&self) -> FH2_CBC_ON_R {
        FH2_CBC_ON_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Set and reset by hardware. If set an one-shot mode action is on going"]
    #[inline(always)]
    pub fn fh2_ost_on(&mut self) -> FH2_OST_ON_W {
        FH2_OST_ON_W { w: self }
    }
    #[doc = "Bit 0 - Set and reset by hardware. If set an cycle-by-cycle mode action is on going"]
    #[inline(always)]
    pub fn fh2_cbc_on(&mut self) -> FH2_CBC_ON_W {
        FH2_CBC_ON_W { w: self }
    }
}
