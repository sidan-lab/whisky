use pallas::ledger::primitives::conway::RationalNumber as PallasRationalNumber;

pub fn parse_rational_number(tuple: (u64, u64)) -> PallasRationalNumber {
    PallasRationalNumber {
        numerator: tuple.0,
        denominator: tuple.1,
    }
}
