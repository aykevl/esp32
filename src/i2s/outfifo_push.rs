#[doc = "Reader of register OUTFIFO_PUSH"]
pub type R = crate::R<u32, super::OUTFIFO_PUSH>;
#[doc = "Writer for register OUTFIFO_PUSH"]
pub type W = crate::W<u32, super::OUTFIFO_PUSH>;
#[doc = "Register OUTFIFO_PUSH `reset()`'s with value 0"]
impl crate::ResetValue for super::OUTFIFO_PUSH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OUTFIFO_PUSH`"]
pub type OUTFIFO_PUSH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUTFIFO_PUSH`"]
pub struct OUTFIFO_PUSH_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTFIFO_PUSH_W<'a> {
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
#[doc = "Reader of field `OUTFIFO_WDATA`"]
pub type OUTFIFO_WDATA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `OUTFIFO_WDATA`"]
pub struct OUTFIFO_WDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTFIFO_WDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn outfifo_push(&self) -> OUTFIFO_PUSH_R {
        OUTFIFO_PUSH_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn outfifo_wdata(&self) -> OUTFIFO_WDATA_R {
        OUTFIFO_WDATA_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn outfifo_push(&mut self) -> OUTFIFO_PUSH_W {
        OUTFIFO_PUSH_W { w: self }
    }
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn outfifo_wdata(&mut self) -> OUTFIFO_WDATA_W {
        OUTFIFO_WDATA_W { w: self }
    }
}
