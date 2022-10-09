use colored::Colorize;
use std::fmt::Display;
use std::ops;

/// Sign MASK
const SMASK: u8 = 0x80; // 0b1000_0000;
const SOFFSET: u8 = 7;
/// Exp MASK
const EMASK: u8 = 0x78; // 0b0111_1000;
const EOFFSET: u8 = 3;
const EBIAS: u8 = 7;
/// Mantissa MASK
const MMASK: u8 = 0x07; // 0b0000_0111;
const MOFFSET: u8 = 0;

const ZERO: u8 = 0x00; // 0b0000_0000;
const NEG_ZERO: u8 = 0x80; // 0b1000_0000;
const NAN: u8 = 0xFF; // 0b1111_1111;
const NEG_NAN: u8 = 0x7F; // 0b0111_1111;

/// float 8 for inference, which has sign, exp, mantissa with 1, 4, 3.
pub struct E4M3 {
    data: u8,
}

impl E4M3 {
    /// Create a new fp8 e4m3 from u8
    pub fn new(val: u8) -> Self {
        E4M3 { data: val }
    }
    /// Create a zero fp8 e4m3
    pub fn zero() -> Self {
        E4M3 { data: ZERO }
    }
    /// Create a negative zero fp8 from e4m3
    pub fn neg_zero() -> Self {
        E4M3 { data: NEG_ZERO }
    }
    pub fn nan() -> Self {
        E4M3 { data: NAN }
    }
    /// Get the decimal representation of fp8 e4m3
    pub fn value(&self) -> f32 {
        let sign = if self.sign() == 1 { -1 } else { 1 } as f32;
        // subnormal: all exp == 0;
        let exp = if self.is_subnormal() {
            -6.0_f32
        } else {
            self.exp() as f32 - EBIAS as f32
        };
        let man = (self.mantissa() as f32 / 8.0) + if self.is_subnormal() { 0.0 } else { 1.0 };
        sign * man * 2.0_f32.powf(exp)
    }

    pub const fn is_subnormal(&self) -> bool {
        self.exp() == 0 && self.mantissa() != 0
    }
    pub const fn is_zero(&self) -> bool {
        self.data == ZERO
    }
    pub const fn is_neg_zero(&self) -> bool {
        self.data == NEG_ZERO
    }
    pub const fn is_nan(&self) -> bool {
        self.data == NAN || self.data == NEG_NAN
    }
    pub const fn sign(&self) -> u8 {
        (self.data & SMASK) >> SOFFSET
    }
    pub const fn exp(&self) -> u8 {
        (self.data & EMASK) >> EOFFSET
    }
    pub const fn mantissa(&self) -> u8 {
        (self.data & MMASK) >> MOFFSET
    }
}

impl ops::Add<E4M3> for E4M3 {
    type Output = E4M3;

    fn add(self, rhs: E4M3) -> Self::Output {
        if self.is_nan() || rhs.is_nan() {
            E4M3::nan()
        } else {
            let exp_diff = self.exp().abs_diff(rhs.exp());
            
            E4M3::nan()
        }
    }
}

impl Display for E4M3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let whole = format!("whole binary: {:08b}", self.data);
        let sign = format!("sign: {:01b}", self.sign());
        let exp = format!("exp: {:04b}", self.exp());
        let mantissa = format!("mantissa: {:03b}", self.mantissa());
        if self.is_nan() {
            write!(
                f,
                "{}\n{} {} {}\nNAN",
                whole.blue(),
                sign.red(),
                exp.yellow(),
                mantissa.purple()
            )
        } else if self.is_zero() {
            write!(
                f,
                "{}\n{} {} {}\nZERO",
                whole.blue(),
                sign.red(),
                exp.yellow(),
                mantissa.purple()
            )
        } else if self.is_neg_zero() {
            write!(
                f,
                "{}\n{} {} {}\nNEG_ZERO",
                whole.blue(),
                sign.red(),
                exp.yellow(),
                mantissa.purple()
            )
        } else {
            write!(
                f,
                "{}\n{} {} {}\n{}",
                whole.blue(),
                sign.red(),
                exp.yellow(),
                mantissa.purple(),
                self.value().to_string().green()
            )
        }
    }
}

impl From<E4M3> for f32 {
    fn from(val: E4M3) -> Self {
        val.value()
    }
}
