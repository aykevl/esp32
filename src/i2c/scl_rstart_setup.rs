#[doc = "Reader of register SCL_RSTART_SETUP"]
pub type R = crate::R<u32, super::SCL_RSTART_SETUP>;
#[doc = "Writer for register SCL_RSTART_SETUP"]
pub type W = crate::W<u32, super::SCL_RSTART_SETUP>;
#[doc = "Register SCL_RSTART_SETUP `reset()`'s with value 0"]
impl crate::ResetValue for super::SCL_RSTART_SETUP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SCL_RSTART_SETUP_TIME`"]
pub type SCL_RSTART_SETUP_TIME_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SCL_RSTART_SETUP_TIME`"]
pub struct SCL_RSTART_SETUP_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> SCL_RSTART_SETUP_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - This register is used to configure the clock num between the posedge of SCL and the negedge of SDA for restart mark."]
    #[inline(always)]
    pub fn scl_rstart_setup_time(&self) -> SCL_RSTART_SETUP_TIME_R {
        SCL_RSTART_SETUP_TIME_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - This register is used to configure the clock num between the posedge of SCL and the negedge of SDA for restart mark."]
    #[inline(always)]
    pub fn scl_rstart_setup_time(&mut self) -> SCL_RSTART_SETUP_TIME_W {
        SCL_RSTART_SETUP_TIME_W { w: self }
    }
}
