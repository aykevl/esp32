#[doc = "Reader of register AT_CMD_PRECNT"]
pub type R = crate::R<u32, super::AT_CMD_PRECNT>;
#[doc = "Writer for register AT_CMD_PRECNT"]
pub type W = crate::W<u32, super::AT_CMD_PRECNT>;
#[doc = "Register AT_CMD_PRECNT `reset()`'s with value 0"]
impl crate::ResetValue for super::AT_CMD_PRECNT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PRE_IDLE_NUM`"]
pub type PRE_IDLE_NUM_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PRE_IDLE_NUM`"]
pub struct PRE_IDLE_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> PRE_IDLE_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - This register is used to configure the idle duration time before the first at_cmd is received by receiver. when the the duration is less than this register value it will not take the next data received as at_cmd char."]
    #[inline(always)]
    pub fn pre_idle_num(&self) -> PRE_IDLE_NUM_R {
        PRE_IDLE_NUM_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - This register is used to configure the idle duration time before the first at_cmd is received by receiver. when the the duration is less than this register value it will not take the next data received as at_cmd char."]
    #[inline(always)]
    pub fn pre_idle_num(&mut self) -> PRE_IDLE_NUM_W {
        PRE_IDLE_NUM_W { w: self }
    }
}
