#[doc = "Reader of register ENABLE_W1TC"]
pub type R = crate::R<u32, super::ENABLE_W1TC>;
#[doc = "Writer for register ENABLE_W1TC"]
pub type W = crate::W<u32, super::ENABLE_W1TC>;
#[doc = "Register ENABLE_W1TC `reset()`'s with value 0"]
impl crate::ResetValue for super::ENABLE_W1TC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ENABLE_DATA_W1TC`"]
pub type ENABLE_DATA_W1TC_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ENABLE_DATA_W1TC`"]
pub struct ENABLE_DATA_W1TC_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_DATA_W1TC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - GPIO0~31 output enable write 1 to clear"]
    #[inline(always)]
    pub fn enable_data_w1tc(&self) -> ENABLE_DATA_W1TC_R {
        ENABLE_DATA_W1TC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - GPIO0~31 output enable write 1 to clear"]
    #[inline(always)]
    pub fn enable_data_w1tc(&mut self) -> ENABLE_DATA_W1TC_W {
        ENABLE_DATA_W1TC_W { w: self }
    }
}
