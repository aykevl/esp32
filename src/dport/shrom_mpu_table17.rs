#[doc = "Reader of register SHROM_MPU_TABLE17"]
pub type R = crate::R<u32, super::SHROM_MPU_TABLE17>;
#[doc = "Writer for register SHROM_MPU_TABLE17"]
pub type W = crate::W<u32, super::SHROM_MPU_TABLE17>;
#[doc = "Register SHROM_MPU_TABLE17 `reset()`'s with value 0"]
impl crate::ResetValue for super::SHROM_MPU_TABLE17 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SHROM_MPU_TABLE17`"]
pub type SHROM_MPU_TABLE17_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SHROM_MPU_TABLE17`"]
pub struct SHROM_MPU_TABLE17_W<'a> {
    w: &'a mut W,
}
impl<'a> SHROM_MPU_TABLE17_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn shrom_mpu_table17(&self) -> SHROM_MPU_TABLE17_R {
        SHROM_MPU_TABLE17_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn shrom_mpu_table17(&mut self) -> SHROM_MPU_TABLE17_W {
        SHROM_MPU_TABLE17_W { w: self }
    }
}
