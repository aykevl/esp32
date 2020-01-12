#[doc = "Reader of register APP_VECBASE_CTRL"]
pub type R = crate::R<u32, super::APP_VECBASE_CTRL>;
#[doc = "Writer for register APP_VECBASE_CTRL"]
pub type W = crate::W<u32, super::APP_VECBASE_CTRL>;
#[doc = "Register APP_VECBASE_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::APP_VECBASE_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `APP_OUT_VECBASE_SEL`"]
pub type APP_OUT_VECBASE_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `APP_OUT_VECBASE_SEL`"]
pub struct APP_OUT_VECBASE_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_OUT_VECBASE_SEL_W<'a> {
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
    pub fn app_out_vecbase_sel(&self) -> APP_OUT_VECBASE_SEL_R {
        APP_OUT_VECBASE_SEL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn app_out_vecbase_sel(&mut self) -> APP_OUT_VECBASE_SEL_W {
        APP_OUT_VECBASE_SEL_W { w: self }
    }
}
