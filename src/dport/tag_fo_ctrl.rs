#[doc = "Reader of register TAG_FO_CTRL"]
pub type R = crate::R<u32, super::TAG_FO_CTRL>;
#[doc = "Writer for register TAG_FO_CTRL"]
pub type W = crate::W<u32, super::TAG_FO_CTRL>;
#[doc = "Register TAG_FO_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::TAG_FO_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DPORT_APP_CACHE_TAG_PD`"]
pub type DPORT_APP_CACHE_TAG_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPORT_APP_CACHE_TAG_PD`"]
pub struct DPORT_APP_CACHE_TAG_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_APP_CACHE_TAG_PD_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `DPORT_APP_CACHE_TAG_FORCE_ON`"]
pub type DPORT_APP_CACHE_TAG_FORCE_ON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPORT_APP_CACHE_TAG_FORCE_ON`"]
pub struct DPORT_APP_CACHE_TAG_FORCE_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_APP_CACHE_TAG_FORCE_ON_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `DPORT_PRO_CACHE_TAG_PD`"]
pub type DPORT_PRO_CACHE_TAG_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPORT_PRO_CACHE_TAG_PD`"]
pub struct DPORT_PRO_CACHE_TAG_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_PRO_CACHE_TAG_PD_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `DPORT_PRO_CACHE_TAG_FORCE_ON`"]
pub type DPORT_PRO_CACHE_TAG_FORCE_ON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPORT_PRO_CACHE_TAG_FORCE_ON`"]
pub struct DPORT_PRO_CACHE_TAG_FORCE_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_PRO_CACHE_TAG_FORCE_ON_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn dport_app_cache_tag_pd(&self) -> DPORT_APP_CACHE_TAG_PD_R {
        DPORT_APP_CACHE_TAG_PD_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dport_app_cache_tag_force_on(&self) -> DPORT_APP_CACHE_TAG_FORCE_ON_R {
        DPORT_APP_CACHE_TAG_FORCE_ON_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dport_pro_cache_tag_pd(&self) -> DPORT_PRO_CACHE_TAG_PD_R {
        DPORT_PRO_CACHE_TAG_PD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dport_pro_cache_tag_force_on(&self) -> DPORT_PRO_CACHE_TAG_FORCE_ON_R {
        DPORT_PRO_CACHE_TAG_FORCE_ON_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn dport_app_cache_tag_pd(&mut self) -> DPORT_APP_CACHE_TAG_PD_W {
        DPORT_APP_CACHE_TAG_PD_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dport_app_cache_tag_force_on(&mut self) -> DPORT_APP_CACHE_TAG_FORCE_ON_W {
        DPORT_APP_CACHE_TAG_FORCE_ON_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dport_pro_cache_tag_pd(&mut self) -> DPORT_PRO_CACHE_TAG_PD_W {
        DPORT_PRO_CACHE_TAG_PD_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dport_pro_cache_tag_force_on(&mut self) -> DPORT_PRO_CACHE_TAG_FORCE_ON_W {
        DPORT_PRO_CACHE_TAG_FORCE_ON_W { w: self }
    }
}
