use cardano_serialization_lib::{
    fees::LinearFee, plutus::ExUnitPrices, tx_builder::*, utils::*, UnitInterval,
};

pub fn to_bignum(val: u64) -> BigNum {
    BigNum::from_str(&val.to_string()).unwrap()
}

pub fn build_tx_builder() -> TransactionBuilder {
    let cfg = TransactionBuilderConfigBuilder::new()
        .fee_algo(&LinearFee::new(&to_bignum(44), &to_bignum(155381)))
        .pool_deposit(&to_bignum(500000000))
        .key_deposit(&to_bignum(2000000))
        .max_value_size(5000)
        .max_tx_size(16384)
        .coins_per_utxo_byte(&to_bignum(4310))
        .ex_unit_prices(&ExUnitPrices::new(
            &UnitInterval::new(&to_bignum(577), &to_bignum(10000)),
            &UnitInterval::new(&to_bignum(721), &to_bignum(10000000)),
        ))
        .build()
        .unwrap();
    return TransactionBuilder::new(&cfg);
}
