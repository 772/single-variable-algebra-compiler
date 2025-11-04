// lib.rs should not contain any Decimal-crate-specific code.
#[allow(unused_imports)]
#[cfg(target_arch = "wasm32")]
use web_sys::window;

pub const MAX_DECIMAL_PLACES: usize = 450; // RUST_BIGDECIMAL_DEFAULT_PRECISION is MAX_DECIMAL_PLACES * 2 + 2.
pub type Dec = bigdecimal::BigDecimal;

pub fn zero() -> Dec {
    bigdecimal::Zero::zero()
}

pub fn dec_to_string(x: Dec) -> String {
    x.to_plain_string()
}

pub fn pow(x: Dec, exp: Dec) -> Dec {
    let x_str = x.to_string();
    let exp_str = exp.to_string();
    if x_str == "0" || x_str == "1" {
        x
    } else if exp_str == "0.5" {
        x.sqrt().unwrap()
    } else if exp_str.contains('.') {
        let result: f64 = x_str
            .parse::<f64>()
            .unwrap()
            .powf(exp_str.parse::<f64>().unwrap());
        bigdecimal::FromPrimitive::from_f64(result).unwrap()
    } else {
        x.powi(exp_str.parse::<i64>().unwrap())
    }
}
