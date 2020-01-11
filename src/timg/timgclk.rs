#[doc = "Reader of register TIMGCLK"]
pub type R = crate::R<u32, super::TIMGCLK>;
#[doc = "Writer for register TIMGCLK"]
pub type W = crate::W<u32, super::TIMGCLK>;
#[doc = "Register TIMGCLK `reset()`'s with value 0"]
impl crate::ResetValue for super::TIMGCLK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TIMG_CLK_EN`"]
pub type TIMG_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMG_CLK_EN`"]
pub struct TIMG_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMG_CLK_EN_W<'a> {
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
impl R {
    #[doc = "Bit 31 - Force clock enable for this regfile"]
    #[inline(always)]
    pub fn timg_clk_en(&self) -> TIMG_CLK_EN_R {
        TIMG_CLK_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Force clock enable for this regfile"]
    #[inline(always)]
    pub fn timg_clk_en(&mut self) -> TIMG_CLK_EN_W {
        TIMG_CLK_EN_W { w: self }
    }
}
