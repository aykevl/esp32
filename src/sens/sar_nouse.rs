#[doc = "Reader of register SAR_NOUSE"]
pub type R = crate::R<u32, super::SAR_NOUSE>;
#[doc = "Writer for register SAR_NOUSE"]
pub type W = crate::W<u32, super::SAR_NOUSE>;
#[doc = "Register SAR_NOUSE `reset()`'s with value 0"]
impl crate::ResetValue for super::SAR_NOUSE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SAR_NOUSE`"]
pub type SAR_NOUSE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SAR_NOUSE`"]
pub struct SAR_NOUSE_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR_NOUSE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sar_nouse(&self) -> SAR_NOUSE_R {
        SAR_NOUSE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sar_nouse(&mut self) -> SAR_NOUSE_W {
        SAR_NOUSE_W { w: self }
    }
}
