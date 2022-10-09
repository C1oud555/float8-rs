use colored::Colorize;
use std::fmt::Display;

/// Sign MASK
const SMASK: u8 = 0x80; // 0b1000_0000;
const SOFFSET: u8 = 7;
/// Exp MASK
const EMASK: u8 = 0x7C; // 0b0111_1100;
const EOFFSET: u8 = 2;
const ESHIFT: u8 = 15;
/// Mantissa MASK
const MMASK: u8 = 0x03; // 0b0000_0011;
const MOFFSET: u8 = 0;

/// float 8 for inference, which has sign, exp, mantissa with 1, 5, 2.
pub struct E5M2 {
    data: u8,
}

impl E5M2 {
    pub fn new(val: u8) -> Self {
        E5M2 { data: val }
    }
    pub fn value(&self) -> f32 {
        let n1: i8 = -1;
        let e2: u8 = 2;
        n1.pow(self.sign().into()) as f32
            * e2.pow((self.exp() - EOFFSET).into()) as f32
            * (1 + (self.mantissa() / 4)) as f32
    }
    pub fn sign(&self) -> u8 {
        (self.data & SMASK) >> SOFFSET
    }

    pub fn exp(&self) -> u8 {
        (self.data & EMASK) >> EOFFSET
    }

    pub fn mantissa(&self) -> u8 {
        (self.data & MMASK) >> MOFFSET
    }
}

impl Display for E5M2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let whole = format!("whole binary: {:08b}", self.data);
        let sign = format!("sign: {:01b}", self.sign());
        let exp = format!("exp: {:05b}", self.exp());
        let mantissa = format!("mantissa: {:02b}", self.mantissa());
        write!(
            f,
            "{}\n{} {} {}",
            whole.blue(),
            sign.red(),
            exp.yellow(),
            mantissa.purple()
        )
    }
}
