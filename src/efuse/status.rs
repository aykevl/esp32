#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Writer for register STATUS"]
pub type W = crate::W<u32, super::STATUS>;
#[doc = "Register STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EFUSE_DEBUG`"]
pub type EFUSE_DEBUG_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `EFUSE_DEBUG`"]
pub struct EFUSE_DEBUG_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_DEBUG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn efuse_debug(&self) -> EFUSE_DEBUG_R {
        EFUSE_DEBUG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn efuse_debug(&mut self) -> EFUSE_DEBUG_W {
        EFUSE_DEBUG_W { w: self }
    }
}
