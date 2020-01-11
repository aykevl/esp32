#[doc = "Reader of register PRO_DCACHE_DBUG6"]
pub type R = crate::R<u32, super::PRO_DCACHE_DBUG6>;
#[doc = "Writer for register PRO_DCACHE_DBUG6"]
pub type W = crate::W<u32, super::PRO_DCACHE_DBUG6>;
#[doc = "Register PRO_DCACHE_DBUG6 `reset()`'s with value 0"]
impl crate::ResetValue for super::PRO_DCACHE_DBUG6 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DPORT_PRO_IRAM0ADDR_IA`"]
pub type DPORT_PRO_IRAM0ADDR_IA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DPORT_PRO_IRAM0ADDR_IA`"]
pub struct DPORT_PRO_IRAM0ADDR_IA_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_PRO_IRAM0ADDR_IA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | ((value as u32) & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn dport_pro_iram0addr_ia(&self) -> DPORT_PRO_IRAM0ADDR_IA_R {
        DPORT_PRO_IRAM0ADDR_IA_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn dport_pro_iram0addr_ia(&mut self) -> DPORT_PRO_IRAM0ADDR_IA_W {
        DPORT_PRO_IRAM0ADDR_IA_W { w: self }
    }
}
