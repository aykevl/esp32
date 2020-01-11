#[doc = "Reader of register AHBLITE_MPU_TABLE_CAN"]
pub type R = crate::R<u32, super::AHBLITE_MPU_TABLE_CAN>;
#[doc = "Writer for register AHBLITE_MPU_TABLE_CAN"]
pub type W = crate::W<u32, super::AHBLITE_MPU_TABLE_CAN>;
#[doc = "Register AHBLITE_MPU_TABLE_CAN `reset()`'s with value 0"]
impl crate::ResetValue for super::AHBLITE_MPU_TABLE_CAN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DPORT_CAN_ACCESS_GRANT_CONFIG`"]
pub type DPORT_CAN_ACCESS_GRANT_CONFIG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DPORT_CAN_ACCESS_GRANT_CONFIG`"]
pub struct DPORT_CAN_ACCESS_GRANT_CONFIG_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_CAN_ACCESS_GRANT_CONFIG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn dport_can_access_grant_config(&self) -> DPORT_CAN_ACCESS_GRANT_CONFIG_R {
        DPORT_CAN_ACCESS_GRANT_CONFIG_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn dport_can_access_grant_config(&mut self) -> DPORT_CAN_ACCESS_GRANT_CONFIG_W {
        DPORT_CAN_ACCESS_GRANT_CONFIG_W { w: self }
    }
}
