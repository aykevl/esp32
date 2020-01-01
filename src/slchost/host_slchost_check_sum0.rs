#[doc = "Reader of register HOST_SLCHOST_CHECK_SUM0"]
pub type R = crate::R<u32, super::HOST_SLCHOST_CHECK_SUM0>;
#[doc = "Writer for register HOST_SLCHOST_CHECK_SUM0"]
pub type W = crate::W<u32, super::HOST_SLCHOST_CHECK_SUM0>;
#[doc = "Register HOST_SLCHOST_CHECK_SUM0 `reset()`'s with value 0"]
impl crate::ResetValue for super::HOST_SLCHOST_CHECK_SUM0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HOST_SLCHOST_CHECK_SUM0`"]
pub type HOST_SLCHOST_CHECK_SUM0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `HOST_SLCHOST_CHECK_SUM0`"]
pub struct HOST_SLCHOST_CHECK_SUM0_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_CHECK_SUM0_W<'a> {
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
    pub fn host_slchost_check_sum0(&self) -> HOST_SLCHOST_CHECK_SUM0_R {
        HOST_SLCHOST_CHECK_SUM0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn host_slchost_check_sum0(&mut self) -> HOST_SLCHOST_CHECK_SUM0_W {
        HOST_SLCHOST_CHECK_SUM0_W { w: self }
    }
}
