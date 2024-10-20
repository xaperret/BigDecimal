use crate::decimal::Decimal;

pub struct BigDecimal<T: Decimal> {
    bd: T,
}

impl<T: Decimal> BigDecimal<T> {
    pub fn new(bd: T) -> Self {
        BigDecimal { bd }
    }

    pub fn add(&self, other: &Self) -> Self {}
}
