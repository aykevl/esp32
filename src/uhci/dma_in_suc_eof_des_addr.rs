#[doc = "Reader of register DMA_IN_SUC_EOF_DES_ADDR"]
pub type R = crate::R<u32, super::DMA_IN_SUC_EOF_DES_ADDR>;
#[doc = "Writer for register DMA_IN_SUC_EOF_DES_ADDR"]
pub type W = crate::W<u32, super::DMA_IN_SUC_EOF_DES_ADDR>;
#[doc = "Register DMA_IN_SUC_EOF_DES_ADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_IN_SUC_EOF_DES_ADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IN_SUC_EOF_DES_ADDR`"]
pub type IN_SUC_EOF_DES_ADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `IN_SUC_EOF_DES_ADDR`"]
pub struct IN_SUC_EOF_DES_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_SUC_EOF_DES_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - This register stores the address of in link descriptor when eof bit in this descriptor is 1."]
    #[inline(always)]
    pub fn in_suc_eof_des_addr(&self) -> IN_SUC_EOF_DES_ADDR_R {
        IN_SUC_EOF_DES_ADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - This register stores the address of in link descriptor when eof bit in this descriptor is 1."]
    #[inline(always)]
    pub fn in_suc_eof_des_addr(&mut self) -> IN_SUC_EOF_DES_ADDR_W {
        IN_SUC_EOF_DES_ADDR_W { w: self }
    }
}
