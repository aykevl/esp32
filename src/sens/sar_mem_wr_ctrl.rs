#[doc = "Reader of register SAR_MEM_WR_CTRL"]
pub type R = crate::R<u32, super::SAR_MEM_WR_CTRL>;
#[doc = "Writer for register SAR_MEM_WR_CTRL"]
pub type W = crate::W<u32, super::SAR_MEM_WR_CTRL>;
#[doc = "Register SAR_MEM_WR_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::SAR_MEM_WR_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SENS_RTC_MEM_WR_OFFST_CLR`"]
pub type SENS_RTC_MEM_WR_OFFST_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SENS_RTC_MEM_WR_OFFST_CLR`"]
pub struct SENS_RTC_MEM_WR_OFFST_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_RTC_MEM_WR_OFFST_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `SENS_MEM_WR_ADDR_SIZE`"]
pub type SENS_MEM_WR_ADDR_SIZE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SENS_MEM_WR_ADDR_SIZE`"]
pub struct SENS_MEM_WR_ADDR_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_MEM_WR_ADDR_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 11)) | (((value as u32) & 0x07ff) << 11);
        self.w
    }
}
#[doc = "Reader of field `SENS_MEM_WR_ADDR_INIT`"]
pub type SENS_MEM_WR_ADDR_INIT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SENS_MEM_WR_ADDR_INIT`"]
pub struct SENS_MEM_WR_ADDR_INIT_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_MEM_WR_ADDR_INIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | ((value as u32) & 0x07ff);
        self.w
    }
}
impl R {
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn sens_rtc_mem_wr_offst_clr(&self) -> SENS_RTC_MEM_WR_OFFST_CLR_R {
        SENS_RTC_MEM_WR_OFFST_CLR_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 11:21"]
    #[inline(always)]
    pub fn sens_mem_wr_addr_size(&self) -> SENS_MEM_WR_ADDR_SIZE_R {
        SENS_MEM_WR_ADDR_SIZE_R::new(((self.bits >> 11) & 0x07ff) as u16)
    }
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn sens_mem_wr_addr_init(&self) -> SENS_MEM_WR_ADDR_INIT_R {
        SENS_MEM_WR_ADDR_INIT_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn sens_rtc_mem_wr_offst_clr(&mut self) -> SENS_RTC_MEM_WR_OFFST_CLR_W {
        SENS_RTC_MEM_WR_OFFST_CLR_W { w: self }
    }
    #[doc = "Bits 11:21"]
    #[inline(always)]
    pub fn sens_mem_wr_addr_size(&mut self) -> SENS_MEM_WR_ADDR_SIZE_W {
        SENS_MEM_WR_ADDR_SIZE_W { w: self }
    }
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn sens_mem_wr_addr_init(&mut self) -> SENS_MEM_WR_ADDR_INIT_W {
        SENS_MEM_WR_ADDR_INIT_W { w: self }
    }
}
