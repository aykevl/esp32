#[doc = "Reader of register SAR_TOUCH_OUT3"]
pub type R = crate::R<u32, super::SAR_TOUCH_OUT3>;
#[doc = "Writer for register SAR_TOUCH_OUT3"]
pub type W = crate::W<u32, super::SAR_TOUCH_OUT3>;
#[doc = "Register SAR_TOUCH_OUT3 `reset()`'s with value 0"]
impl crate::ResetValue for super::SAR_TOUCH_OUT3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SENS_TOUCH_MEAS_OUT4`"]
pub type SENS_TOUCH_MEAS_OUT4_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SENS_TOUCH_MEAS_OUT4`"]
pub struct SENS_TOUCH_MEAS_OUT4_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_TOUCH_MEAS_OUT4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `SENS_TOUCH_MEAS_OUT5`"]
pub type SENS_TOUCH_MEAS_OUT5_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SENS_TOUCH_MEAS_OUT5`"]
pub struct SENS_TOUCH_MEAS_OUT5_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_TOUCH_MEAS_OUT5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - the counter for touch pad 4"]
    #[inline(always)]
    pub fn sens_touch_meas_out4(&self) -> SENS_TOUCH_MEAS_OUT4_R {
        SENS_TOUCH_MEAS_OUT4_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - the counter for touch pad 5"]
    #[inline(always)]
    pub fn sens_touch_meas_out5(&self) -> SENS_TOUCH_MEAS_OUT5_R {
        SENS_TOUCH_MEAS_OUT5_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - the counter for touch pad 4"]
    #[inline(always)]
    pub fn sens_touch_meas_out4(&mut self) -> SENS_TOUCH_MEAS_OUT4_W {
        SENS_TOUCH_MEAS_OUT4_W { w: self }
    }
    #[doc = "Bits 0:15 - the counter for touch pad 5"]
    #[inline(always)]
    pub fn sens_touch_meas_out5(&mut self) -> SENS_TOUCH_MEAS_OUT5_W {
        SENS_TOUCH_MEAS_OUT5_W { w: self }
    }
}
