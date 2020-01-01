#[doc = "Reader of register U4_CONF0"]
pub type R = crate::R<u32, super::U4_CONF0>;
#[doc = "Writer for register U4_CONF0"]
pub type W = crate::W<u32, super::U4_CONF0>;
#[doc = "Register U4_CONF0 `reset()`'s with value 0"]
impl crate::ResetValue for super::U4_CONF0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PCNT_CH1_LCTRL_MODE_U4`"]
pub type PCNT_CH1_LCTRL_MODE_U4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PCNT_CH1_LCTRL_MODE_U4`"]
pub struct PCNT_CH1_LCTRL_MODE_U4_W<'a> {
    w: &'a mut W,
}
impl<'a> PCNT_CH1_LCTRL_MODE_U4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
#[doc = "Reader of field `PCNT_CH1_HCTRL_MODE_U4`"]
pub type PCNT_CH1_HCTRL_MODE_U4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PCNT_CH1_HCTRL_MODE_U4`"]
pub struct PCNT_CH1_HCTRL_MODE_U4_W<'a> {
    w: &'a mut W,
}
impl<'a> PCNT_CH1_HCTRL_MODE_U4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Reader of field `PCNT_CH1_POS_MODE_U4`"]
pub type PCNT_CH1_POS_MODE_U4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PCNT_CH1_POS_MODE_U4`"]
pub struct PCNT_CH1_POS_MODE_U4_W<'a> {
    w: &'a mut W,
}
impl<'a> PCNT_CH1_POS_MODE_U4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "Reader of field `PCNT_CH1_NEG_MODE_U4`"]
pub type PCNT_CH1_NEG_MODE_U4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PCNT_CH1_NEG_MODE_U4`"]
pub struct PCNT_CH1_NEG_MODE_U4_W<'a> {
    w: &'a mut W,
}
impl<'a> PCNT_CH1_NEG_MODE_U4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `PCNT_CH0_LCTRL_MODE_U4`"]
pub type PCNT_CH0_LCTRL_MODE_U4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PCNT_CH0_LCTRL_MODE_U4`"]
pub struct PCNT_CH0_LCTRL_MODE_U4_W<'a> {
    w: &'a mut W,
}
impl<'a> PCNT_CH0_LCTRL_MODE_U4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Reader of field `PCNT_CH0_HCTRL_MODE_U4`"]
pub type PCNT_CH0_HCTRL_MODE_U4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PCNT_CH0_HCTRL_MODE_U4`"]
pub struct PCNT_CH0_HCTRL_MODE_U4_W<'a> {
    w: &'a mut W,
}
impl<'a> PCNT_CH0_HCTRL_MODE_U4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Reader of field `PCNT_CH0_POS_MODE_U4`"]
pub type PCNT_CH0_POS_MODE_U4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PCNT_CH0_POS_MODE_U4`"]
pub struct PCNT_CH0_POS_MODE_U4_W<'a> {
    w: &'a mut W,
}
impl<'a> PCNT_CH0_POS_MODE_U4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Reader of field `PCNT_CH0_NEG_MODE_U4`"]
pub type PCNT_CH0_NEG_MODE_U4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PCNT_CH0_NEG_MODE_U4`"]
pub struct PCNT_CH0_NEG_MODE_U4_W<'a> {
    w: &'a mut W,
}
impl<'a> PCNT_CH0_NEG_MODE_U4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `PCNT_THR_THRES1_EN_U4`"]
pub type PCNT_THR_THRES1_EN_U4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCNT_THR_THRES1_EN_U4`"]
pub struct PCNT_THR_THRES1_EN_U4_W<'a> {
    w: &'a mut W,
}
impl<'a> PCNT_THR_THRES1_EN_U4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `PCNT_THR_THRES0_EN_U4`"]
pub type PCNT_THR_THRES0_EN_U4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCNT_THR_THRES0_EN_U4`"]
pub struct PCNT_THR_THRES0_EN_U4_W<'a> {
    w: &'a mut W,
}
impl<'a> PCNT_THR_THRES0_EN_U4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `PCNT_THR_L_LIM_EN_U4`"]
pub type PCNT_THR_L_LIM_EN_U4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCNT_THR_L_LIM_EN_U4`"]
pub struct PCNT_THR_L_LIM_EN_U4_W<'a> {
    w: &'a mut W,
}
impl<'a> PCNT_THR_L_LIM_EN_U4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `PCNT_THR_H_LIM_EN_U4`"]
pub type PCNT_THR_H_LIM_EN_U4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCNT_THR_H_LIM_EN_U4`"]
pub struct PCNT_THR_H_LIM_EN_U4_W<'a> {
    w: &'a mut W,
}
impl<'a> PCNT_THR_H_LIM_EN_U4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `PCNT_THR_ZERO_EN_U4`"]
pub type PCNT_THR_ZERO_EN_U4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCNT_THR_ZERO_EN_U4`"]
pub struct PCNT_THR_ZERO_EN_U4_W<'a> {
    w: &'a mut W,
}
impl<'a> PCNT_THR_ZERO_EN_U4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `PCNT_FILTER_EN_U4`"]
pub type PCNT_FILTER_EN_U4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCNT_FILTER_EN_U4`"]
pub struct PCNT_FILTER_EN_U4_W<'a> {
    w: &'a mut W,
}
impl<'a> PCNT_FILTER_EN_U4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `PCNT_FILTER_THRES_U4`"]
pub type PCNT_FILTER_THRES_U4_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PCNT_FILTER_THRES_U4`"]
pub struct PCNT_FILTER_THRES_U4_W<'a> {
    w: &'a mut W,
}
impl<'a> PCNT_FILTER_THRES_U4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31 - This register is used to control the mode of channel1's low control signal for unit4. 2'd0:increase when control signal is low 2'd1: decrease when control signal is high others:forbidden"]
    #[inline(always)]
    pub fn pcnt_ch1_lctrl_mode_u4(&self) -> PCNT_CH1_LCTRL_MODE_U4_R {
        PCNT_CH1_LCTRL_MODE_U4_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - This register is used to control the mode of channel1's high control signal for unit4. 2'd0:increase when control signal is low 2'd1: decrease when control signal is high others:forbidden"]
    #[inline(always)]
    pub fn pcnt_ch1_hctrl_mode_u4(&self) -> PCNT_CH1_HCTRL_MODE_U4_R {
        PCNT_CH1_HCTRL_MODE_U4_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - This register is used to control the mode of channel1's input posedge signal for unit4. 2'd1: increase at the posedge of input signal 2'd2:decrease at the posedge of input signal others:forbidden"]
    #[inline(always)]
    pub fn pcnt_ch1_pos_mode_u4(&self) -> PCNT_CH1_POS_MODE_U4_R {
        PCNT_CH1_POS_MODE_U4_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - This register is used to control the mode of channel1's input negedge signal for unit4. 2'd1: increase at the negedge of input signal 2'd2:decrease at the negedge of input signal others:forbidden"]
    #[inline(always)]
    pub fn pcnt_ch1_neg_mode_u4(&self) -> PCNT_CH1_NEG_MODE_U4_R {
        PCNT_CH1_NEG_MODE_U4_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - This register is used to control the mode of channel0's low control signal for unit4. 2'd0:increase when control signal is low 2'd1: decrease when control signal is high others:forbidden"]
    #[inline(always)]
    pub fn pcnt_ch0_lctrl_mode_u4(&self) -> PCNT_CH0_LCTRL_MODE_U4_R {
        PCNT_CH0_LCTRL_MODE_U4_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - This register is used to control the mode of channel0's high control signal for unit4. 2'd0:increase when control signal is low 2'd1: decrease when control signal is high others:forbidden"]
    #[inline(always)]
    pub fn pcnt_ch0_hctrl_mode_u4(&self) -> PCNT_CH0_HCTRL_MODE_U4_R {
        PCNT_CH0_HCTRL_MODE_U4_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - This register is used to control the mode of channel0's input posedge signal for unit4. 2'd1: increase at the posedge of input signal 2'd2:decrease at the posedge of input signal others:forbidden"]
    #[inline(always)]
    pub fn pcnt_ch0_pos_mode_u4(&self) -> PCNT_CH0_POS_MODE_U4_R {
        PCNT_CH0_POS_MODE_U4_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - This register is used to control the mode of channel0's input negedge signal for unit4. 2'd1: increase at the negedge of input signal 2'd2:decrease at the negedge of input signal others:forbidden"]
    #[inline(always)]
    pub fn pcnt_ch0_neg_mode_u4(&self) -> PCNT_CH0_NEG_MODE_U4_R {
        PCNT_CH0_NEG_MODE_U4_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 15 - This is the enable bit for comparing unit4's count with thres1 value ."]
    #[inline(always)]
    pub fn pcnt_thr_thres1_en_u4(&self) -> PCNT_THR_THRES1_EN_U4_R {
        PCNT_THR_THRES1_EN_U4_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - This is the enable bit for comparing unit4's count with thres0 value."]
    #[inline(always)]
    pub fn pcnt_thr_thres0_en_u4(&self) -> PCNT_THR_THRES0_EN_U4_R {
        PCNT_THR_THRES0_EN_U4_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - This is the enable bit for comparing unit4's count with thr_l_lim value."]
    #[inline(always)]
    pub fn pcnt_thr_l_lim_en_u4(&self) -> PCNT_THR_L_LIM_EN_U4_R {
        PCNT_THR_L_LIM_EN_U4_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - This is the enable bit for comparing unit4's count with thr_h_lim value."]
    #[inline(always)]
    pub fn pcnt_thr_h_lim_en_u4(&self) -> PCNT_THR_H_LIM_EN_U4_R {
        PCNT_THR_H_LIM_EN_U4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - This is the enable bit for comparing unit4's count with 0 value."]
    #[inline(always)]
    pub fn pcnt_thr_zero_en_u4(&self) -> PCNT_THR_ZERO_EN_U4_R {
        PCNT_THR_ZERO_EN_U4_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - This is the enable bit for filtering input signals for unit4."]
    #[inline(always)]
    pub fn pcnt_filter_en_u4(&self) -> PCNT_FILTER_EN_U4_R {
        PCNT_FILTER_EN_U4_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 0:9 - This register is used to filter pluse whose width is smaller than this value for unit4."]
    #[inline(always)]
    pub fn pcnt_filter_thres_u4(&self) -> PCNT_FILTER_THRES_U4_R {
        PCNT_FILTER_THRES_U4_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 30:31 - This register is used to control the mode of channel1's low control signal for unit4. 2'd0:increase when control signal is low 2'd1: decrease when control signal is high others:forbidden"]
    #[inline(always)]
    pub fn pcnt_ch1_lctrl_mode_u4(&mut self) -> PCNT_CH1_LCTRL_MODE_U4_W {
        PCNT_CH1_LCTRL_MODE_U4_W { w: self }
    }
    #[doc = "Bits 28:29 - This register is used to control the mode of channel1's high control signal for unit4. 2'd0:increase when control signal is low 2'd1: decrease when control signal is high others:forbidden"]
    #[inline(always)]
    pub fn pcnt_ch1_hctrl_mode_u4(&mut self) -> PCNT_CH1_HCTRL_MODE_U4_W {
        PCNT_CH1_HCTRL_MODE_U4_W { w: self }
    }
    #[doc = "Bits 26:27 - This register is used to control the mode of channel1's input posedge signal for unit4. 2'd1: increase at the posedge of input signal 2'd2:decrease at the posedge of input signal others:forbidden"]
    #[inline(always)]
    pub fn pcnt_ch1_pos_mode_u4(&mut self) -> PCNT_CH1_POS_MODE_U4_W {
        PCNT_CH1_POS_MODE_U4_W { w: self }
    }
    #[doc = "Bits 24:25 - This register is used to control the mode of channel1's input negedge signal for unit4. 2'd1: increase at the negedge of input signal 2'd2:decrease at the negedge of input signal others:forbidden"]
    #[inline(always)]
    pub fn pcnt_ch1_neg_mode_u4(&mut self) -> PCNT_CH1_NEG_MODE_U4_W {
        PCNT_CH1_NEG_MODE_U4_W { w: self }
    }
    #[doc = "Bits 22:23 - This register is used to control the mode of channel0's low control signal for unit4. 2'd0:increase when control signal is low 2'd1: decrease when control signal is high others:forbidden"]
    #[inline(always)]
    pub fn pcnt_ch0_lctrl_mode_u4(&mut self) -> PCNT_CH0_LCTRL_MODE_U4_W {
        PCNT_CH0_LCTRL_MODE_U4_W { w: self }
    }
    #[doc = "Bits 20:21 - This register is used to control the mode of channel0's high control signal for unit4. 2'd0:increase when control signal is low 2'd1: decrease when control signal is high others:forbidden"]
    #[inline(always)]
    pub fn pcnt_ch0_hctrl_mode_u4(&mut self) -> PCNT_CH0_HCTRL_MODE_U4_W {
        PCNT_CH0_HCTRL_MODE_U4_W { w: self }
    }
    #[doc = "Bits 18:19 - This register is used to control the mode of channel0's input posedge signal for unit4. 2'd1: increase at the posedge of input signal 2'd2:decrease at the posedge of input signal others:forbidden"]
    #[inline(always)]
    pub fn pcnt_ch0_pos_mode_u4(&mut self) -> PCNT_CH0_POS_MODE_U4_W {
        PCNT_CH0_POS_MODE_U4_W { w: self }
    }
    #[doc = "Bits 16:17 - This register is used to control the mode of channel0's input negedge signal for unit4. 2'd1: increase at the negedge of input signal 2'd2:decrease at the negedge of input signal others:forbidden"]
    #[inline(always)]
    pub fn pcnt_ch0_neg_mode_u4(&mut self) -> PCNT_CH0_NEG_MODE_U4_W {
        PCNT_CH0_NEG_MODE_U4_W { w: self }
    }
    #[doc = "Bit 15 - This is the enable bit for comparing unit4's count with thres1 value ."]
    #[inline(always)]
    pub fn pcnt_thr_thres1_en_u4(&mut self) -> PCNT_THR_THRES1_EN_U4_W {
        PCNT_THR_THRES1_EN_U4_W { w: self }
    }
    #[doc = "Bit 14 - This is the enable bit for comparing unit4's count with thres0 value."]
    #[inline(always)]
    pub fn pcnt_thr_thres0_en_u4(&mut self) -> PCNT_THR_THRES0_EN_U4_W {
        PCNT_THR_THRES0_EN_U4_W { w: self }
    }
    #[doc = "Bit 13 - This is the enable bit for comparing unit4's count with thr_l_lim value."]
    #[inline(always)]
    pub fn pcnt_thr_l_lim_en_u4(&mut self) -> PCNT_THR_L_LIM_EN_U4_W {
        PCNT_THR_L_LIM_EN_U4_W { w: self }
    }
    #[doc = "Bit 12 - This is the enable bit for comparing unit4's count with thr_h_lim value."]
    #[inline(always)]
    pub fn pcnt_thr_h_lim_en_u4(&mut self) -> PCNT_THR_H_LIM_EN_U4_W {
        PCNT_THR_H_LIM_EN_U4_W { w: self }
    }
    #[doc = "Bit 11 - This is the enable bit for comparing unit4's count with 0 value."]
    #[inline(always)]
    pub fn pcnt_thr_zero_en_u4(&mut self) -> PCNT_THR_ZERO_EN_U4_W {
        PCNT_THR_ZERO_EN_U4_W { w: self }
    }
    #[doc = "Bit 10 - This is the enable bit for filtering input signals for unit4."]
    #[inline(always)]
    pub fn pcnt_filter_en_u4(&mut self) -> PCNT_FILTER_EN_U4_W {
        PCNT_FILTER_EN_U4_W { w: self }
    }
    #[doc = "Bits 0:9 - This register is used to filter pluse whose width is smaller than this value for unit4."]
    #[inline(always)]
    pub fn pcnt_filter_thres_u4(&mut self) -> PCNT_FILTER_THRES_U4_W {
        PCNT_FILTER_THRES_U4_W { w: self }
    }
}
