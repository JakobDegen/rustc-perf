#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 4usize],
    #[doc = "0x04 - Cluster CH%s, containing ?CR1, ?CR2, ?FRCR, ?SLOTR, ?IM, ?SR, ?CLRFR, ?DR"]
    pub cha: CH,
    #[doc = "0x24 - Cluster CH%s, containing ?CR1, ?CR2, ?FRCR, ?SLOTR, ?IM, ?SR, ?CLRFR, ?DR"]
    pub chb: CH,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct CH {
    #[doc = "0x00 - SAI AConfiguration register 1"]
    pub cr1: self::ch::CR1,
    #[doc = "0x04 - SAI AConfiguration register 2"]
    pub cr2: self::ch::CR2,
    #[doc = "0x08 - SAI AFrame configuration register"]
    pub frcr: self::ch::FRCR,
    #[doc = "0x0c - SAI ASlot register"]
    pub slotr: self::ch::SLOTR,
    #[doc = "0x10 - SAI AInterrupt mask register2"]
    pub im: self::ch::IM,
    #[doc = "0x14 - SAI AStatus register"]
    pub sr: self::ch::SR,
    #[doc = "0x18 - SAI AClear flag register"]
    pub clrfr: self::ch::CLRFR,
    #[doc = "0x1c - SAI AData register"]
    pub dr: self::ch::DR,
}
#[doc = r"Register block"]
#[doc = "Cluster CH%s, containing ?CR1, ?CR2, ?FRCR, ?SLOTR, ?IM, ?SR, ?CLRFR, ?DR"]
pub mod ch;
