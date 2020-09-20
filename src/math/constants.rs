#![allow(dead_code)]
use sp_arithmetic::{
    traits::{CheckedDiv, Saturating},
    FixedPointNumber, FixedU128,
};

#[derive(Debug)]
pub struct DivErr;

/// Euler's Constant Approximation
/// e = lim (1 + 1/n)^{n} as n approaches infinity
fn euler_c(n: FixedU128) -> Result<FixedU128, DivErr> {
    let exp: f64 = n.to_fraction();
    if let Some(i) = FixedU128::checked_div(&(n + FixedU128::one()), &n) {
        Ok(i.saturating_pow(exp as usize))
    } else {
        Err(DivErr)
    }
}

/// Bernoulli Number Approximation
/// n / (e(n)^{n} - 1) s.t. e(n) is the constant e to n digits of precision
fn bernoulli_num(c: FixedU128) -> Result<FixedU128, DivErr> {
    let exp: f64 = c.to_fraction();
    let e_to_n = euler_c(c)?.saturating_pow(exp as usize);
    let den = e_to_n - FixedU128::one();
    if let Some(i) = c.checked_div(&den) {
        Ok(i)
    } else {
        Err(DivErr)
    }
}
