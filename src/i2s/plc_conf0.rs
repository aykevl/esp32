#[doc = "Reader of register PLC_CONF0"]
pub type R = crate::R<u32, super::PLC_CONF0>;
#[doc = "Writer for register PLC_CONF0"]
pub type W = crate::W<u32, super::PLC_CONF0>;
#[doc = "Register PLC_CONF0 `reset()`'s with value 0"]
impl crate::ResetValue for super::PLC_CONF0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2S_N_MIN_ERR`"]
pub type I2S_N_MIN_ERR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2S_N_MIN_ERR`"]
pub struct I2S_N_MIN_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_N_MIN_ERR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 25)) | (((value as u32) & 0x07) << 25);
        self.w
    }
}
#[doc = "Reader of field `I2S_PACK_LEN_8K`"]
pub type I2S_PACK_LEN_8K_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2S_PACK_LEN_8K`"]
pub struct I2S_PACK_LEN_8K_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_PACK_LEN_8K_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 20)) | (((value as u32) & 0x1f) << 20);
        self.w
    }
}
#[doc = "Reader of field `I2S_MAX_SLIDE_SAMPLE`"]
pub type I2S_MAX_SLIDE_SAMPLE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2S_MAX_SLIDE_SAMPLE`"]
pub struct I2S_MAX_SLIDE_SAMPLE_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_MAX_SLIDE_SAMPLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 12)) | (((value as u32) & 0xff) << 12);
        self.w
    }
}
#[doc = "Reader of field `I2S_SHIFT_RATE`"]
pub type I2S_SHIFT_RATE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2S_SHIFT_RATE`"]
pub struct I2S_SHIFT_RATE_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_SHIFT_RATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 9)) | (((value as u32) & 0x07) << 9);
        self.w
    }
}
#[doc = "Reader of field `I2S_N_ERR_SEG`"]
pub type I2S_N_ERR_SEG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2S_N_ERR_SEG`"]
pub struct I2S_N_ERR_SEG_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_N_ERR_SEG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 6)) | (((value as u32) & 0x07) << 6);
        self.w
    }
}
#[doc = "Reader of field `I2S_GOOD_PACK_MAX`"]
pub type I2S_GOOD_PACK_MAX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2S_GOOD_PACK_MAX`"]
pub struct I2S_GOOD_PACK_MAX_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_GOOD_PACK_MAX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 25:27"]
    #[inline(always)]
    pub fn i2s_n_min_err(&self) -> I2S_N_MIN_ERR_R {
        I2S_N_MIN_ERR_R::new(((self.bits >> 25) & 0x07) as u8)
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    pub fn i2s_pack_len_8k(&self) -> I2S_PACK_LEN_8K_R {
        I2S_PACK_LEN_8K_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 12:19"]
    #[inline(always)]
    pub fn i2s_max_slide_sample(&self) -> I2S_MAX_SLIDE_SAMPLE_R {
        I2S_MAX_SLIDE_SAMPLE_R::new(((self.bits >> 12) & 0xff) as u8)
    }
    #[doc = "Bits 9:11"]
    #[inline(always)]
    pub fn i2s_shift_rate(&self) -> I2S_SHIFT_RATE_R {
        I2S_SHIFT_RATE_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    #[doc = "Bits 6:8"]
    #[inline(always)]
    pub fn i2s_n_err_seg(&self) -> I2S_N_ERR_SEG_R {
        I2S_N_ERR_SEG_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn i2s_good_pack_max(&self) -> I2S_GOOD_PACK_MAX_R {
        I2S_GOOD_PACK_MAX_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 25:27"]
    #[inline(always)]
    pub fn i2s_n_min_err(&mut self) -> I2S_N_MIN_ERR_W {
        I2S_N_MIN_ERR_W { w: self }
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    pub fn i2s_pack_len_8k(&mut self) -> I2S_PACK_LEN_8K_W {
        I2S_PACK_LEN_8K_W { w: self }
    }
    #[doc = "Bits 12:19"]
    #[inline(always)]
    pub fn i2s_max_slide_sample(&mut self) -> I2S_MAX_SLIDE_SAMPLE_W {
        I2S_MAX_SLIDE_SAMPLE_W { w: self }
    }
    #[doc = "Bits 9:11"]
    #[inline(always)]
    pub fn i2s_shift_rate(&mut self) -> I2S_SHIFT_RATE_W {
        I2S_SHIFT_RATE_W { w: self }
    }
    #[doc = "Bits 6:8"]
    #[inline(always)]
    pub fn i2s_n_err_seg(&mut self) -> I2S_N_ERR_SEG_W {
        I2S_N_ERR_SEG_W { w: self }
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn i2s_good_pack_max(&mut self) -> I2S_GOOD_PACK_MAX_W {
        I2S_GOOD_PACK_MAX_W { w: self }
    }
}
