#[doc = "Reader of register GEN2_A"]
pub type R = crate::R<u32, super::GEN2_A>;
#[doc = "Writer for register GEN2_A"]
pub type W = crate::W<u32, super::GEN2_A>;
#[doc = "Register GEN2_A `reset()`'s with value 0"]
impl crate::ResetValue for super::GEN2_A {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MCPWM_GEN2_A_DT1`"]
pub type MCPWM_GEN2_A_DT1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MCPWM_GEN2_A_DT1`"]
pub struct MCPWM_GEN2_A_DT1_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_GEN2_A_DT1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Reader of field `MCPWM_GEN2_A_DT0`"]
pub type MCPWM_GEN2_A_DT0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MCPWM_GEN2_A_DT0`"]
pub struct MCPWM_GEN2_A_DT0_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_GEN2_A_DT0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Reader of field `MCPWM_GEN2_A_DTEB`"]
pub type MCPWM_GEN2_A_DTEB_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MCPWM_GEN2_A_DTEB`"]
pub struct MCPWM_GEN2_A_DTEB_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_GEN2_A_DTEB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Reader of field `MCPWM_GEN2_A_DTEA`"]
pub type MCPWM_GEN2_A_DTEA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MCPWM_GEN2_A_DTEA`"]
pub struct MCPWM_GEN2_A_DTEA_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_GEN2_A_DTEA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `MCPWM_GEN2_A_DTEP`"]
pub type MCPWM_GEN2_A_DTEP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MCPWM_GEN2_A_DTEP`"]
pub struct MCPWM_GEN2_A_DTEP_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_GEN2_A_DTEP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `MCPWM_GEN2_A_DTEZ`"]
pub type MCPWM_GEN2_A_DTEZ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MCPWM_GEN2_A_DTEZ`"]
pub struct MCPWM_GEN2_A_DTEZ_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_GEN2_A_DTEZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `MCPWM_GEN2_A_UT1`"]
pub type MCPWM_GEN2_A_UT1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MCPWM_GEN2_A_UT1`"]
pub struct MCPWM_GEN2_A_UT1_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_GEN2_A_UT1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `MCPWM_GEN2_A_UT0`"]
pub type MCPWM_GEN2_A_UT0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MCPWM_GEN2_A_UT0`"]
pub struct MCPWM_GEN2_A_UT0_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_GEN2_A_UT0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `MCPWM_GEN2_A_UTEB`"]
pub type MCPWM_GEN2_A_UTEB_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MCPWM_GEN2_A_UTEB`"]
pub struct MCPWM_GEN2_A_UTEB_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_GEN2_A_UTEB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `MCPWM_GEN2_A_UTEA`"]
pub type MCPWM_GEN2_A_UTEA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MCPWM_GEN2_A_UTEA`"]
pub struct MCPWM_GEN2_A_UTEA_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_GEN2_A_UTEA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `MCPWM_GEN2_A_UTEP`"]
pub type MCPWM_GEN2_A_UTEP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MCPWM_GEN2_A_UTEP`"]
pub struct MCPWM_GEN2_A_UTEP_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_GEN2_A_UTEP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `MCPWM_GEN2_A_UTEZ`"]
pub type MCPWM_GEN2_A_UTEZ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MCPWM_GEN2_A_UTEZ`"]
pub struct MCPWM_GEN2_A_UTEZ_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_GEN2_A_UTEZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 22:23 - Action on PWM2A triggered by event_t1 when timer decreasing. 0: no change 1: low 2: high 3: toggle"]
    #[inline(always)]
    pub fn mcpwm_gen2_a_dt1(&self) -> MCPWM_GEN2_A_DT1_R {
        MCPWM_GEN2_A_DT1_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Action on PWM2A triggered by event_t0 when timer decreasing"]
    #[inline(always)]
    pub fn mcpwm_gen2_a_dt0(&self) -> MCPWM_GEN2_A_DT0_R {
        MCPWM_GEN2_A_DT0_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Action on PWM2A triggered by event TEB when timer decreasing"]
    #[inline(always)]
    pub fn mcpwm_gen2_a_dteb(&self) -> MCPWM_GEN2_A_DTEB_R {
        MCPWM_GEN2_A_DTEB_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Action on PWM2A triggered by event TEA when timer decreasing"]
    #[inline(always)]
    pub fn mcpwm_gen2_a_dtea(&self) -> MCPWM_GEN2_A_DTEA_R {
        MCPWM_GEN2_A_DTEA_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Action on PWM2A triggered by event TEP when timer decreasing"]
    #[inline(always)]
    pub fn mcpwm_gen2_a_dtep(&self) -> MCPWM_GEN2_A_DTEP_R {
        MCPWM_GEN2_A_DTEP_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Action on PWM2A triggered by event TEZ when timer decreasing"]
    #[inline(always)]
    pub fn mcpwm_gen2_a_dtez(&self) -> MCPWM_GEN2_A_DTEZ_R {
        MCPWM_GEN2_A_DTEZ_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Action on PWM2A triggered by event_t1 when timer increasing"]
    #[inline(always)]
    pub fn mcpwm_gen2_a_ut1(&self) -> MCPWM_GEN2_A_UT1_R {
        MCPWM_GEN2_A_UT1_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Action on PWM2A triggered by event_t0 when timer increasing"]
    #[inline(always)]
    pub fn mcpwm_gen2_a_ut0(&self) -> MCPWM_GEN2_A_UT0_R {
        MCPWM_GEN2_A_UT0_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Action on PWM2A triggered by event TEB when timer increasing"]
    #[inline(always)]
    pub fn mcpwm_gen2_a_uteb(&self) -> MCPWM_GEN2_A_UTEB_R {
        MCPWM_GEN2_A_UTEB_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Action on PWM2A triggered by event TEA when timer increasing"]
    #[inline(always)]
    pub fn mcpwm_gen2_a_utea(&self) -> MCPWM_GEN2_A_UTEA_R {
        MCPWM_GEN2_A_UTEA_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Action on PWM2A triggered by event TEP when timer increasing"]
    #[inline(always)]
    pub fn mcpwm_gen2_a_utep(&self) -> MCPWM_GEN2_A_UTEP_R {
        MCPWM_GEN2_A_UTEP_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - Action on PWM2A triggered by event TEZ when timer increasing"]
    #[inline(always)]
    pub fn mcpwm_gen2_a_utez(&self) -> MCPWM_GEN2_A_UTEZ_R {
        MCPWM_GEN2_A_UTEZ_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 22:23 - Action on PWM2A triggered by event_t1 when timer decreasing. 0: no change 1: low 2: high 3: toggle"]
    #[inline(always)]
    pub fn mcpwm_gen2_a_dt1(&mut self) -> MCPWM_GEN2_A_DT1_W {
        MCPWM_GEN2_A_DT1_W { w: self }
    }
    #[doc = "Bits 20:21 - Action on PWM2A triggered by event_t0 when timer decreasing"]
    #[inline(always)]
    pub fn mcpwm_gen2_a_dt0(&mut self) -> MCPWM_GEN2_A_DT0_W {
        MCPWM_GEN2_A_DT0_W { w: self }
    }
    #[doc = "Bits 18:19 - Action on PWM2A triggered by event TEB when timer decreasing"]
    #[inline(always)]
    pub fn mcpwm_gen2_a_dteb(&mut self) -> MCPWM_GEN2_A_DTEB_W {
        MCPWM_GEN2_A_DTEB_W { w: self }
    }
    #[doc = "Bits 16:17 - Action on PWM2A triggered by event TEA when timer decreasing"]
    #[inline(always)]
    pub fn mcpwm_gen2_a_dtea(&mut self) -> MCPWM_GEN2_A_DTEA_W {
        MCPWM_GEN2_A_DTEA_W { w: self }
    }
    #[doc = "Bits 14:15 - Action on PWM2A triggered by event TEP when timer decreasing"]
    #[inline(always)]
    pub fn mcpwm_gen2_a_dtep(&mut self) -> MCPWM_GEN2_A_DTEP_W {
        MCPWM_GEN2_A_DTEP_W { w: self }
    }
    #[doc = "Bits 12:13 - Action on PWM2A triggered by event TEZ when timer decreasing"]
    #[inline(always)]
    pub fn mcpwm_gen2_a_dtez(&mut self) -> MCPWM_GEN2_A_DTEZ_W {
        MCPWM_GEN2_A_DTEZ_W { w: self }
    }
    #[doc = "Bits 10:11 - Action on PWM2A triggered by event_t1 when timer increasing"]
    #[inline(always)]
    pub fn mcpwm_gen2_a_ut1(&mut self) -> MCPWM_GEN2_A_UT1_W {
        MCPWM_GEN2_A_UT1_W { w: self }
    }
    #[doc = "Bits 8:9 - Action on PWM2A triggered by event_t0 when timer increasing"]
    #[inline(always)]
    pub fn mcpwm_gen2_a_ut0(&mut self) -> MCPWM_GEN2_A_UT0_W {
        MCPWM_GEN2_A_UT0_W { w: self }
    }
    #[doc = "Bits 6:7 - Action on PWM2A triggered by event TEB when timer increasing"]
    #[inline(always)]
    pub fn mcpwm_gen2_a_uteb(&mut self) -> MCPWM_GEN2_A_UTEB_W {
        MCPWM_GEN2_A_UTEB_W { w: self }
    }
    #[doc = "Bits 4:5 - Action on PWM2A triggered by event TEA when timer increasing"]
    #[inline(always)]
    pub fn mcpwm_gen2_a_utea(&mut self) -> MCPWM_GEN2_A_UTEA_W {
        MCPWM_GEN2_A_UTEA_W { w: self }
    }
    #[doc = "Bits 2:3 - Action on PWM2A triggered by event TEP when timer increasing"]
    #[inline(always)]
    pub fn mcpwm_gen2_a_utep(&mut self) -> MCPWM_GEN2_A_UTEP_W {
        MCPWM_GEN2_A_UTEP_W { w: self }
    }
    #[doc = "Bits 0:1 - Action on PWM2A triggered by event TEZ when timer increasing"]
    #[inline(always)]
    pub fn mcpwm_gen2_a_utez(&mut self) -> MCPWM_GEN2_A_UTEZ_W {
        MCPWM_GEN2_A_UTEZ_W { w: self }
    }
}
