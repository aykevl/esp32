#[doc = "Reader of register PRO_CACHE_LOCK_0_ADDR"]
pub type R = crate::R<u32, super::PRO_CACHE_LOCK_0_ADDR>;
#[doc = "Writer for register PRO_CACHE_LOCK_0_ADDR"]
pub type W = crate::W<u32, super::PRO_CACHE_LOCK_0_ADDR>;
#[doc = "Register PRO_CACHE_LOCK_0_ADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::PRO_CACHE_LOCK_0_ADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DPORT_PRO_CACHE_LOCK_0_ADDR_MAX`"]
pub type DPORT_PRO_CACHE_LOCK_0_ADDR_MAX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DPORT_PRO_CACHE_LOCK_0_ADDR_MAX`"]
pub struct DPORT_PRO_CACHE_LOCK_0_ADDR_MAX_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_PRO_CACHE_LOCK_0_ADDR_MAX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 18)) | (((value as u32) & 0x0f) << 18);
        self.w
    }
}
#[doc = "Reader of field `DPORT_PRO_CACHE_LOCK_0_ADDR_MIN`"]
pub type DPORT_PRO_CACHE_LOCK_0_ADDR_MIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DPORT_PRO_CACHE_LOCK_0_ADDR_MIN`"]
pub struct DPORT_PRO_CACHE_LOCK_0_ADDR_MIN_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_PRO_CACHE_LOCK_0_ADDR_MIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 14)) | (((value as u32) & 0x0f) << 14);
        self.w
    }
}
#[doc = "Reader of field `DPORT_PRO_CACHE_LOCK_0_ADDR_PRE`"]
pub type DPORT_PRO_CACHE_LOCK_0_ADDR_PRE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DPORT_PRO_CACHE_LOCK_0_ADDR_PRE`"]
pub struct DPORT_PRO_CACHE_LOCK_0_ADDR_PRE_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_PRO_CACHE_LOCK_0_ADDR_PRE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | ((value as u32) & 0x3fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 18:21"]
    #[inline(always)]
    pub fn dport_pro_cache_lock_0_addr_max(&self) -> DPORT_PRO_CACHE_LOCK_0_ADDR_MAX_R {
        DPORT_PRO_CACHE_LOCK_0_ADDR_MAX_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 14:17"]
    #[inline(always)]
    pub fn dport_pro_cache_lock_0_addr_min(&self) -> DPORT_PRO_CACHE_LOCK_0_ADDR_MIN_R {
        DPORT_PRO_CACHE_LOCK_0_ADDR_MIN_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn dport_pro_cache_lock_0_addr_pre(&self) -> DPORT_PRO_CACHE_LOCK_0_ADDR_PRE_R {
        DPORT_PRO_CACHE_LOCK_0_ADDR_PRE_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 18:21"]
    #[inline(always)]
    pub fn dport_pro_cache_lock_0_addr_max(&mut self) -> DPORT_PRO_CACHE_LOCK_0_ADDR_MAX_W {
        DPORT_PRO_CACHE_LOCK_0_ADDR_MAX_W { w: self }
    }
    #[doc = "Bits 14:17"]
    #[inline(always)]
    pub fn dport_pro_cache_lock_0_addr_min(&mut self) -> DPORT_PRO_CACHE_LOCK_0_ADDR_MIN_W {
        DPORT_PRO_CACHE_LOCK_0_ADDR_MIN_W { w: self }
    }
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn dport_pro_cache_lock_0_addr_pre(&mut self) -> DPORT_PRO_CACHE_LOCK_0_ADDR_PRE_W {
        DPORT_PRO_CACHE_LOCK_0_ADDR_PRE_W { w: self }
    }
}
