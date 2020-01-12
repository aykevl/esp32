#[doc = "Reader of register LACTRTC"]
pub type R = crate::R<u32, super::LACTRTC>;
#[doc = "Writer for register LACTRTC"]
pub type W = crate::W<u32, super::LACTRTC>;
#[doc = "Register LACTRTC `reset()`'s with value 0"]
impl crate::ResetValue for super::LACTRTC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LACT_RTC_STEP_LEN`"]
pub type LACT_RTC_STEP_LEN_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `LACT_RTC_STEP_LEN`"]
pub struct LACT_RTC_STEP_LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LACT_RTC_STEP_LEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff_ffff << 6)) | (((value as u32) & 0x03ff_ffff) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 6:31"]
    #[inline(always)]
    pub fn lact_rtc_step_len(&self) -> LACT_RTC_STEP_LEN_R {
        LACT_RTC_STEP_LEN_R::new(((self.bits >> 6) & 0x03ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 6:31"]
    #[inline(always)]
    pub fn lact_rtc_step_len(&mut self) -> LACT_RTC_STEP_LEN_W {
        LACT_RTC_STEP_LEN_W { w: self }
    }
}
