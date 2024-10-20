use crate::{decimal::Decimal, utilities::is_number};

const STORAGE_WIDTH_IN_BITS: i32 = 32;
const DECIMAL32_BIAS: i32 = 101; // Exponent bias for Decimal32
const MAX_EXPONENT: i32 = 96; // Maximum exponent (emax)
const MIN_EXPONENT: i32 = -95; // Minimum exponent (emin)
const SIGN_MASK: u32 = 0x80000000; // Mask for the sign bit
const EXPONENT_MASK: u32 = 0x7F800000; // Mask for the exponent field
const SIGNIFICAND_MASK: u32 = 0x007FFFFF; // Mask for the significand (23 bits)

pub struct Decimal32 {
    // 1 bit: for the sign.
    // 5 bits: for the combination field, which encodes part of the exponent and the most significant decimal digit.
    // 20 bits: for the trailing significand, which stores the rest of the digits using DPD encoding.
    // 6 bits: for the exponent continuation (bias + additional exponent bits).
    bits: u32,
}

impl Decimal for Decimal32 {
    fn sign(&self) -> bool {
        (self.bits & SIGN_MASK) != 0
    }

    fn exponent(&self) -> i32 {
        let raw_exponent = (self.bits & EXPONENT_MASK) >> 23;
        raw_exponent as i32 - DECIMAL32_BIAS
    }

    fn significand(&self) -> u32 {
        self.bits & SIGNIFICAND_MASK
    }

    fn new(sign: bool, exponent: i32, significand: u32) -> Self {
        // Adjust the exponent by adding the bias
        let biased_exponent = (exponent + DECIMAL32_BIAS) as u32;
        let sign_bit = if sign { SIGN_MASK } else { 0 };

        Decimal32 {
            bits: sign_bit | (biased_exponent << 23) | (significand & SIGNIFICAND_MASK),
        }
    }
}

impl From<String> for Decimal32 {
    fn from(item: String) -> Self {
        let is_number = is_number(&item);
        if !is_number {
            panic!("Invalid number format");
        }

        // Assuming the string is a valid number, parse it and create a Decimal32 instance
        // This is a placeholder implementation; you need to replace it with actual parsing logic
        Decimal32::new(false, 0, 0)
    }
}

impl From<&str> for Decimal32 {
    fn from(item: &str) -> Self {
        let dec = Decimal32::try_from(item);
        match dec {
            Ok(decimal) => decimal,
            Err(error) => panic!("{}", error),
        }
    }
}

impl TryFrom<&String> for Decimal32 {
    type Error = String;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        if !is_number(value) {
            return Err(String::from("Invalid number format"));
        }

        // Assuming the string is a valid number, parse it and create a Decimal32 instance
        // This is a placeholder implementation; actual parsing logic is required
        Ok(Decimal32::new(false, 0, 0)) // Replace with actual logic
    }
}
