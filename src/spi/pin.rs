#[doc = "Reader of register PIN"]
pub type R = crate::R<u32, super::PIN>;
#[doc = "Writer for register PIN"]
pub type W = crate::W<u32, super::PIN>;
#[doc = "Register PIN `reset()`'s with value 0"]
impl crate::ResetValue for super::PIN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI_CS_KEEP_ACTIVE`"]
pub type SPI_CS_KEEP_ACTIVE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_CS_KEEP_ACTIVE`"]
pub struct SPI_CS_KEEP_ACTIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_CS_KEEP_ACTIVE_W<'a> {
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
#[doc = "Reader of field `SPI_CK_IDLE_EDGE`"]
pub type SPI_CK_IDLE_EDGE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_CK_IDLE_EDGE`"]
pub struct SPI_CK_IDLE_EDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_CK_IDLE_EDGE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `SPI_MASTER_CK_SEL`"]
pub type SPI_MASTER_CK_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI_MASTER_CK_SEL`"]
pub struct SPI_MASTER_CK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MASTER_CK_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | (((value as u32) & 0x07) << 11);
        self.w
    }
}
#[doc = "Reader of field `SPI_MASTER_CS_POL`"]
pub type SPI_MASTER_CS_POL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI_MASTER_CS_POL`"]
pub struct SPI_MASTER_CS_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MASTER_CS_POL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 6)) | (((value as u32) & 0x07) << 6);
        self.w
    }
}
#[doc = "Reader of field `SPI_CK_DIS`"]
pub type SPI_CK_DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_CK_DIS`"]
pub struct SPI_CK_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_CK_DIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `SPI_CS2_DIS`"]
pub type SPI_CS2_DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_CS2_DIS`"]
pub struct SPI_CS2_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_CS2_DIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `SPI_CS1_DIS`"]
pub type SPI_CS1_DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_CS1_DIS`"]
pub struct SPI_CS1_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_CS1_DIS_W<'a> {
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
#[doc = "Reader of field `SPI_CS0_DIS`"]
pub type SPI_CS0_DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_CS0_DIS`"]
pub struct SPI_CS0_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_CS0_DIS_W<'a> {
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
    #[doc = "Bit 30 - spi cs line keep low when the bit is set."]
    #[inline(always)]
    pub fn spi_cs_keep_active(&self) -> SPI_CS_KEEP_ACTIVE_R {
        SPI_CS_KEEP_ACTIVE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - 1: spi clk line is high when idle 0: spi clk line is low when idle"]
    #[inline(always)]
    pub fn spi_ck_idle_edge(&self) -> SPI_CK_IDLE_EDGE_R {
        SPI_CK_IDLE_EDGE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bits 11:13 - In the master mode spi cs line is enable as spi clk it is combined with spi_cs0_dis spi_cs1_dis spi_cs2_dis."]
    #[inline(always)]
    pub fn spi_master_ck_sel(&self) -> SPI_MASTER_CK_SEL_R {
        SPI_MASTER_CK_SEL_R::new(((self.bits >> 11) & 0x07) as u8)
    }
    #[doc = "Bits 6:8 - In the master mode the bits are the polarity of spi cs line the value is equivalent to spi_cs ^ spi_master_cs_pol."]
    #[inline(always)]
    pub fn spi_master_cs_pol(&self) -> SPI_MASTER_CS_POL_R {
        SPI_MASTER_CS_POL_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    #[doc = "Bit 5 - 1: spi clk out disable 0: spi clk out enable"]
    #[inline(always)]
    pub fn spi_ck_dis(&self) -> SPI_CK_DIS_R {
        SPI_CK_DIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SPI CS2 pin enable, 1: disable CS2, 0: spi_cs2 signal is from/to CS2 pin"]
    #[inline(always)]
    pub fn spi_cs2_dis(&self) -> SPI_CS2_DIS_R {
        SPI_CS2_DIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - SPI CS1 pin enable, 1: disable CS1, 0: spi_cs1 signal is from/to CS1 pin"]
    #[inline(always)]
    pub fn spi_cs1_dis(&self) -> SPI_CS1_DIS_R {
        SPI_CS1_DIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - SPI CS0 pin enable, 1: disable CS0, 0: spi_cs0 signal is from/to CS0 pin"]
    #[inline(always)]
    pub fn spi_cs0_dis(&self) -> SPI_CS0_DIS_R {
        SPI_CS0_DIS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 30 - spi cs line keep low when the bit is set."]
    #[inline(always)]
    pub fn spi_cs_keep_active(&mut self) -> SPI_CS_KEEP_ACTIVE_W {
        SPI_CS_KEEP_ACTIVE_W { w: self }
    }
    #[doc = "Bit 29 - 1: spi clk line is high when idle 0: spi clk line is low when idle"]
    #[inline(always)]
    pub fn spi_ck_idle_edge(&mut self) -> SPI_CK_IDLE_EDGE_W {
        SPI_CK_IDLE_EDGE_W { w: self }
    }
    #[doc = "Bits 11:13 - In the master mode spi cs line is enable as spi clk it is combined with spi_cs0_dis spi_cs1_dis spi_cs2_dis."]
    #[inline(always)]
    pub fn spi_master_ck_sel(&mut self) -> SPI_MASTER_CK_SEL_W {
        SPI_MASTER_CK_SEL_W { w: self }
    }
    #[doc = "Bits 6:8 - In the master mode the bits are the polarity of spi cs line the value is equivalent to spi_cs ^ spi_master_cs_pol."]
    #[inline(always)]
    pub fn spi_master_cs_pol(&mut self) -> SPI_MASTER_CS_POL_W {
        SPI_MASTER_CS_POL_W { w: self }
    }
    #[doc = "Bit 5 - 1: spi clk out disable 0: spi clk out enable"]
    #[inline(always)]
    pub fn spi_ck_dis(&mut self) -> SPI_CK_DIS_W {
        SPI_CK_DIS_W { w: self }
    }
    #[doc = "Bit 2 - SPI CS2 pin enable, 1: disable CS2, 0: spi_cs2 signal is from/to CS2 pin"]
    #[inline(always)]
    pub fn spi_cs2_dis(&mut self) -> SPI_CS2_DIS_W {
        SPI_CS2_DIS_W { w: self }
    }
    #[doc = "Bit 1 - SPI CS1 pin enable, 1: disable CS1, 0: spi_cs1 signal is from/to CS1 pin"]
    #[inline(always)]
    pub fn spi_cs1_dis(&mut self) -> SPI_CS1_DIS_W {
        SPI_CS1_DIS_W { w: self }
    }
    #[doc = "Bit 0 - SPI CS0 pin enable, 1: disable CS0, 0: spi_cs0 signal is from/to CS0 pin"]
    #[inline(always)]
    pub fn spi_cs0_dis(&mut self) -> SPI_CS0_DIS_W {
        SPI_CS0_DIS_W { w: self }
    }
}
