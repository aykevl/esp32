#[doc = "Reader of register 0_EOF_START_DES"]
pub type R = crate::R<u32, super::_0_EOF_START_DES>;
#[doc = "Writer for register 0_EOF_START_DES"]
pub type W = crate::W<u32, super::_0_EOF_START_DES>;
#[doc = "Register 0_EOF_START_DES `reset()`'s with value 0"]
impl crate::ResetValue for super::_0_EOF_START_DES {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SLC_SLC0_EOF_START_DES_ADDR`"]
pub type SLC_SLC0_EOF_START_DES_ADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SLC_SLC0_EOF_START_DES_ADDR`"]
pub struct SLC_SLC0_EOF_START_DES_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC0_EOF_START_DES_ADDR_W<'a> {
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
    pub fn slc_slc0_eof_start_des_addr(&self) -> SLC_SLC0_EOF_START_DES_ADDR_R {
        SLC_SLC0_EOF_START_DES_ADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn slc_slc0_eof_start_des_addr(&mut self) -> SLC_SLC0_EOF_START_DES_ADDR_W {
        SLC_SLC0_EOF_START_DES_ADDR_W { w: self }
    }
}
