#[doc = "Reader of register HOST_SLCHOST_CONF_W3"]
pub type R = crate::R<u32, super::HOST_SLCHOST_CONF_W3>;
#[doc = "Writer for register HOST_SLCHOST_CONF_W3"]
pub type W = crate::W<u32, super::HOST_SLCHOST_CONF_W3>;
#[doc = "Register HOST_SLCHOST_CONF_W3 `reset()`'s with value 0"]
impl crate::ResetValue for super::HOST_SLCHOST_CONF_W3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HOST_SLCHOST_CONF15`"]
pub type HOST_SLCHOST_CONF15_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOST_SLCHOST_CONF15`"]
pub struct HOST_SLCHOST_CONF15_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_CONF15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `HOST_SLCHOST_CONF14`"]
pub type HOST_SLCHOST_CONF14_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOST_SLCHOST_CONF14`"]
pub struct HOST_SLCHOST_CONF14_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_CONF14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `HOST_SLCHOST_CONF13`"]
pub type HOST_SLCHOST_CONF13_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOST_SLCHOST_CONF13`"]
pub struct HOST_SLCHOST_CONF13_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_CONF13_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `HOST_SLCHOST_CONF12`"]
pub type HOST_SLCHOST_CONF12_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOST_SLCHOST_CONF12`"]
pub struct HOST_SLCHOST_CONF12_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_CONF12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn host_slchost_conf15(&self) -> HOST_SLCHOST_CONF15_R {
        HOST_SLCHOST_CONF15_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn host_slchost_conf14(&self) -> HOST_SLCHOST_CONF14_R {
        HOST_SLCHOST_CONF14_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn host_slchost_conf13(&self) -> HOST_SLCHOST_CONF13_R {
        HOST_SLCHOST_CONF13_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn host_slchost_conf12(&self) -> HOST_SLCHOST_CONF12_R {
        HOST_SLCHOST_CONF12_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn host_slchost_conf15(&mut self) -> HOST_SLCHOST_CONF15_W {
        HOST_SLCHOST_CONF15_W { w: self }
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn host_slchost_conf14(&mut self) -> HOST_SLCHOST_CONF14_W {
        HOST_SLCHOST_CONF14_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn host_slchost_conf13(&mut self) -> HOST_SLCHOST_CONF13_W {
        HOST_SLCHOST_CONF13_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn host_slchost_conf12(&mut self) -> HOST_SLCHOST_CONF12_W {
        HOST_SLCHOST_CONF12_W { w: self }
    }
}
