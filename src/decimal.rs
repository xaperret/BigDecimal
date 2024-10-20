// src/decimal/decimal.rs

pub trait Decimal {
    fn sign(&self) -> bool;
    fn exponent(&self) -> i32;
    fn significand(&self) -> u32;
    fn new(sign: bool, exponent: i32, significand: u32) -> Self;
}
