#[doc = "Reader of register SAR_TOUCH_THRES4"]
pub type R = crate::R<u32, super::SAR_TOUCH_THRES4>;
#[doc = "Writer for register SAR_TOUCH_THRES4"]
pub type W = crate::W<u32, super::SAR_TOUCH_THRES4>;
#[doc = "Register SAR_TOUCH_THRES4 `reset()`'s with value 0"]
impl crate::ResetValue for super::SAR_TOUCH_THRES4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TOUCH_OUT_TH6`"]
pub type TOUCH_OUT_TH6_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TOUCH_OUT_TH6`"]
pub struct TOUCH_OUT_TH6_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_OUT_TH6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `TOUCH_OUT_TH7`"]
pub type TOUCH_OUT_TH7_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TOUCH_OUT_TH7`"]
pub struct TOUCH_OUT_TH7_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_OUT_TH7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - the threshold for touch pad 6"]
    #[inline(always)]
    pub fn touch_out_th6(&self) -> TOUCH_OUT_TH6_R {
        TOUCH_OUT_TH6_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - the threshold for touch pad 7"]
    #[inline(always)]
    pub fn touch_out_th7(&self) -> TOUCH_OUT_TH7_R {
        TOUCH_OUT_TH7_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - the threshold for touch pad 6"]
    #[inline(always)]
    pub fn touch_out_th6(&mut self) -> TOUCH_OUT_TH6_W {
        TOUCH_OUT_TH6_W { w: self }
    }
    #[doc = "Bits 0:15 - the threshold for touch pad 7"]
    #[inline(always)]
    pub fn touch_out_th7(&mut self) -> TOUCH_OUT_TH7_W {
        TOUCH_OUT_TH7_W { w: self }
    }
}
