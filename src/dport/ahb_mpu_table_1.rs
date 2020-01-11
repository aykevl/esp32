#[doc = "Reader of register AHB_MPU_TABLE_1"]
pub type R = crate::R<u32, super::AHB_MPU_TABLE_1>;
#[doc = "Writer for register AHB_MPU_TABLE_1"]
pub type W = crate::W<u32, super::AHB_MPU_TABLE_1>;
#[doc = "Register AHB_MPU_TABLE_1 `reset()`'s with value 0"]
impl crate::ResetValue for super::AHB_MPU_TABLE_1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DPORT_AHB_ACCESS_GRANT_1`"]
pub type DPORT_AHB_ACCESS_GRANT_1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DPORT_AHB_ACCESS_GRANT_1`"]
pub struct DPORT_AHB_ACCESS_GRANT_1_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_AHB_ACCESS_GRANT_1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn dport_ahb_access_grant_1(&self) -> DPORT_AHB_ACCESS_GRANT_1_R {
        DPORT_AHB_ACCESS_GRANT_1_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn dport_ahb_access_grant_1(&mut self) -> DPORT_AHB_ACCESS_GRANT_1_W {
        DPORT_AHB_ACCESS_GRANT_1_W { w: self }
    }
}
