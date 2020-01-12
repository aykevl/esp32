#[doc = "Reader of register T1CONFIG"]
pub type R = crate::R<u32, super::T1CONFIG>;
#[doc = "Writer for register T1CONFIG"]
pub type W = crate::W<u32, super::T1CONFIG>;
#[doc = "Register T1CONFIG `reset()`'s with value 0"]
impl crate::ResetValue for super::T1CONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `T1_EN`"]
pub type T1_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `T1_EN`"]
pub struct T1_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> T1_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `T1_INCREASE`"]
pub type T1_INCREASE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `T1_INCREASE`"]
pub struct T1_INCREASE_W<'a> {
    w: &'a mut W,
}
impl<'a> T1_INCREASE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `T1_AUTORELOAD`"]
pub type T1_AUTORELOAD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `T1_AUTORELOAD`"]
pub struct T1_AUTORELOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> T1_AUTORELOAD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `T1_DIVIDER`"]
pub type T1_DIVIDER_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `T1_DIVIDER`"]
pub struct T1_DIVIDER_W<'a> {
    w: &'a mut W,
}
impl<'a> T1_DIVIDER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 13)) | (((value as u32) & 0xffff) << 13);
        self.w
    }
}
#[doc = "Reader of field `T1_EDGE_INT_EN`"]
pub type T1_EDGE_INT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `T1_EDGE_INT_EN`"]
pub struct T1_EDGE_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> T1_EDGE_INT_EN_W<'a> {
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
#[doc = "Reader of field `T1_LEVEL_INT_EN`"]
pub type T1_LEVEL_INT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `T1_LEVEL_INT_EN`"]
pub struct T1_LEVEL_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> T1_LEVEL_INT_EN_W<'a> {
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
#[doc = "Reader of field `T1_ALARM_EN`"]
pub type T1_ALARM_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `T1_ALARM_EN`"]
pub struct T1_ALARM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> T1_ALARM_EN_W<'a> {
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
impl R {
    #[doc = "Bit 31 - When set timer 1 time-base counter is enabled"]
    #[inline(always)]
    pub fn t1_en(&self) -> T1_EN_R {
        T1_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - When set timer 1 time-base counter increment. When cleared timer 1 time-base counter decrement."]
    #[inline(always)]
    pub fn t1_increase(&self) -> T1_INCREASE_R {
        T1_INCREASE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - When set timer 1 auto-reload at alarming is enabled"]
    #[inline(always)]
    pub fn t1_autoreload(&self) -> T1_AUTORELOAD_R {
        T1_AUTORELOAD_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bits 13:28 - Timer 1 clock (T1_clk) prescale value."]
    #[inline(always)]
    pub fn t1_divider(&self) -> T1_DIVIDER_R {
        T1_DIVIDER_R::new(((self.bits >> 13) & 0xffff) as u16)
    }
    #[doc = "Bit 12 - When set edge type interrupt will be generated during alarm"]
    #[inline(always)]
    pub fn t1_edge_int_en(&self) -> T1_EDGE_INT_EN_R {
        T1_EDGE_INT_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - When set level type interrupt will be generated during alarm"]
    #[inline(always)]
    pub fn t1_level_int_en(&self) -> T1_LEVEL_INT_EN_R {
        T1_LEVEL_INT_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - When set alarm is enabled"]
    #[inline(always)]
    pub fn t1_alarm_en(&self) -> T1_ALARM_EN_R {
        T1_ALARM_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - When set timer 1 time-base counter is enabled"]
    #[inline(always)]
    pub fn t1_en(&mut self) -> T1_EN_W {
        T1_EN_W { w: self }
    }
    #[doc = "Bit 30 - When set timer 1 time-base counter increment. When cleared timer 1 time-base counter decrement."]
    #[inline(always)]
    pub fn t1_increase(&mut self) -> T1_INCREASE_W {
        T1_INCREASE_W { w: self }
    }
    #[doc = "Bit 29 - When set timer 1 auto-reload at alarming is enabled"]
    #[inline(always)]
    pub fn t1_autoreload(&mut self) -> T1_AUTORELOAD_W {
        T1_AUTORELOAD_W { w: self }
    }
    #[doc = "Bits 13:28 - Timer 1 clock (T1_clk) prescale value."]
    #[inline(always)]
    pub fn t1_divider(&mut self) -> T1_DIVIDER_W {
        T1_DIVIDER_W { w: self }
    }
    #[doc = "Bit 12 - When set edge type interrupt will be generated during alarm"]
    #[inline(always)]
    pub fn t1_edge_int_en(&mut self) -> T1_EDGE_INT_EN_W {
        T1_EDGE_INT_EN_W { w: self }
    }
    #[doc = "Bit 11 - When set level type interrupt will be generated during alarm"]
    #[inline(always)]
    pub fn t1_level_int_en(&mut self) -> T1_LEVEL_INT_EN_W {
        T1_LEVEL_INT_EN_W { w: self }
    }
    #[doc = "Bit 10 - When set alarm is enabled"]
    #[inline(always)]
    pub fn t1_alarm_en(&mut self) -> T1_ALARM_EN_W {
        T1_ALARM_EN_W { w: self }
    }
}
