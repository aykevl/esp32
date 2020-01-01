#[doc = "Reader of register BLK2_RDATA1"]
pub type R = crate::R<u32, super::BLK2_RDATA1>;
#[doc = "Writer for register BLK2_RDATA1"]
pub type W = crate::W<u32, super::BLK2_RDATA1>;
#[doc = "Register BLK2_RDATA1 `reset()`'s with value 0"]
impl crate::ResetValue for super::BLK2_RDATA1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EFUSE_BLK2_DOUT1`"]
pub type EFUSE_BLK2_DOUT1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `EFUSE_BLK2_DOUT1`"]
pub struct EFUSE_BLK2_DOUT1_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_BLK2_DOUT1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - read for BLOCK2"]
    #[inline(always)]
    pub fn efuse_blk2_dout1(&self) -> EFUSE_BLK2_DOUT1_R {
        EFUSE_BLK2_DOUT1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - read for BLOCK2"]
    #[inline(always)]
    pub fn efuse_blk2_dout1(&mut self) -> EFUSE_BLK2_DOUT1_W {
        EFUSE_BLK2_DOUT1_W { w: self }
    }
}
