#[doc = "Reader of register APP_CPU_RECORD_PDEBUGLS0DATA"]
pub type R = crate::R<u32, super::APP_CPU_RECORD_PDEBUGLS0DATA>;
#[doc = "Writer for register APP_CPU_RECORD_PDEBUGLS0DATA"]
pub type W = crate::W<u32, super::APP_CPU_RECORD_PDEBUGLS0DATA>;
#[doc = "Register APP_CPU_RECORD_PDEBUGLS0DATA `reset()`'s with value 0"]
impl crate::ResetValue for super::APP_CPU_RECORD_PDEBUGLS0DATA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DPORT_RECORD_APP_PDEBUGLS0DATA`"]
pub type DPORT_RECORD_APP_PDEBUGLS0DATA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DPORT_RECORD_APP_PDEBUGLS0DATA`"]
pub struct DPORT_RECORD_APP_PDEBUGLS0DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_RECORD_APP_PDEBUGLS0DATA_W<'a> {
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
    pub fn dport_record_app_pdebugls0data(&self) -> DPORT_RECORD_APP_PDEBUGLS0DATA_R {
        DPORT_RECORD_APP_PDEBUGLS0DATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dport_record_app_pdebugls0data(&mut self) -> DPORT_RECORD_APP_PDEBUGLS0DATA_W {
        DPORT_RECORD_APP_PDEBUGLS0DATA_W { w: self }
    }
}
