#[doc = "Reader of register EXT_XTL_CONF"]
pub type R = crate::R<u32, super::EXT_XTL_CONF>;
#[doc = "Writer for register EXT_XTL_CONF"]
pub type W = crate::W<u32, super::EXT_XTL_CONF>;
#[doc = "Register EXT_XTL_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::EXT_XTL_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `XTL_EXT_CTR_EN`"]
pub type XTL_EXT_CTR_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XTL_EXT_CTR_EN`"]
pub struct XTL_EXT_CTR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> XTL_EXT_CTR_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `XTL_EXT_CTR_LV`"]
pub type XTL_EXT_CTR_LV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XTL_EXT_CTR_LV`"]
pub struct XTL_EXT_CTR_LV_W<'a> {
    w: &'a mut W,
}
impl<'a> XTL_EXT_CTR_LV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - enable control XTAL by external pads"]
    #[inline(always)]
    pub fn xtl_ext_ctr_en(&self) -> XTL_EXT_CTR_EN_R {
        XTL_EXT_CTR_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - 0: power down XTAL at high level 1: power down XTAL at low level"]
    #[inline(always)]
    pub fn xtl_ext_ctr_lv(&self) -> XTL_EXT_CTR_LV_R {
        XTL_EXT_CTR_LV_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - enable control XTAL by external pads"]
    #[inline(always)]
    pub fn xtl_ext_ctr_en(&mut self) -> XTL_EXT_CTR_EN_W {
        XTL_EXT_CTR_EN_W { w: self }
    }
    #[doc = "Bit 30 - 0: power down XTAL at high level 1: power down XTAL at low level"]
    #[inline(always)]
    pub fn xtl_ext_ctr_lv(&mut self) -> XTL_EXT_CTR_LV_W {
        XTL_EXT_CTR_LV_W { w: self }
    }
}
