#[doc = "Reader of register SDIO_ACT_CONF"]
pub type R = crate::R<u32, super::SDIO_ACT_CONF>;
#[doc = "Writer for register SDIO_ACT_CONF"]
pub type W = crate::W<u32, super::SDIO_ACT_CONF>;
#[doc = "Register SDIO_ACT_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::SDIO_ACT_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SDIO_ACT_DNUM`"]
pub type SDIO_ACT_DNUM_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SDIO_ACT_DNUM`"]
pub struct SDIO_ACT_DNUM_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_ACT_DNUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 22)) | (((value as u32) & 0x03ff) << 22);
        self.w
    }
}
impl R {
    #[doc = "Bits 22:31"]
    #[inline(always)]
    pub fn sdio_act_dnum(&self) -> SDIO_ACT_DNUM_R {
        SDIO_ACT_DNUM_R::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 22:31"]
    #[inline(always)]
    pub fn sdio_act_dnum(&mut self) -> SDIO_ACT_DNUM_W {
        SDIO_ACT_DNUM_W { w: self }
    }
}
