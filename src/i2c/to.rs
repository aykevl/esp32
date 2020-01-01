#[doc = "Reader of register TO"]
pub type R = crate::R<u32, super::TO>;
#[doc = "Writer for register TO"]
pub type W = crate::W<u32, super::TO>;
#[doc = "Register TO `reset()`'s with value 0"]
impl crate::ResetValue for super::TO {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2C_TIME_OUT_REG`"]
pub type I2C_TIME_OUT_REG_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `I2C_TIME_OUT_REG`"]
pub struct I2C_TIME_OUT_REG_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_TIME_OUT_REG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | ((value as u32) & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:19 - This register is used to configure the max clock number of receiving a data."]
    #[inline(always)]
    pub fn i2c_time_out_reg(&self) -> I2C_TIME_OUT_REG_R {
        I2C_TIME_OUT_REG_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:19 - This register is used to configure the max clock number of receiving a data."]
    #[inline(always)]
    pub fn i2c_time_out_reg(&mut self) -> I2C_TIME_OUT_REG_W {
        I2C_TIME_OUT_REG_W { w: self }
    }
}
