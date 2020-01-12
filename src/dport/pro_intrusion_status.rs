#[doc = "Reader of register PRO_INTRUSION_STATUS"]
pub type R = crate::R<u32, super::PRO_INTRUSION_STATUS>;
#[doc = "Writer for register PRO_INTRUSION_STATUS"]
pub type W = crate::W<u32, super::PRO_INTRUSION_STATUS>;
#[doc = "Register PRO_INTRUSION_STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::PRO_INTRUSION_STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PRO_INTRUSION_RECORD`"]
pub type PRO_INTRUSION_RECORD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRO_INTRUSION_RECORD`"]
pub struct PRO_INTRUSION_RECORD_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_INTRUSION_RECORD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn pro_intrusion_record(&self) -> PRO_INTRUSION_RECORD_R {
        PRO_INTRUSION_RECORD_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn pro_intrusion_record(&mut self) -> PRO_INTRUSION_RECORD_W {
        PRO_INTRUSION_RECORD_W { w: self }
    }
}
