#[cfg(target_arch = "wasm32")]
use web_sys::window;

#[cfg(not(target_arch = "wasm32"))]
pub const MAX_DECIMAL_PLACES: usize = 100;
#[cfg(target_arch = "wasm32")]
pub const MAX_DECIMAL_PLACES: usize = 25;

#[cfg(not(target_arch = "wasm32"))]
pub type Dec = dec::Decimal<MAX_DECIMAL_PLACES>;
#[cfg(target_arch = "wasm32")]
pub type Dec = fastnum::decimal::Decimal<MAX_DECIMAL_PLACES>;

#[cfg(not(target_arch = "wasm32"))]
pub fn zero() -> Dec {
    Dec::zero()
}
#[cfg(target_arch = "wasm32")]
pub fn zero() -> Dec {
    Dec::ZERO
}

#[cfg(not(target_arch = "wasm32"))]
pub fn pow(mut x: Dec, exp: Dec) -> Dec {
    let mut ctx = dec::Context::<Dec>::default();
    ctx.set_min_exponent(-1000).unwrap();
    ctx.set_max_exponent(1000).unwrap();
    ctx.pow(&mut x, &exp);
    x
}
#[cfg(target_arch = "wasm32")]
pub fn pow(x: Dec, exp: Dec) -> Dec {
    x.pow(exp)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn normal_string(x: Dec) -> String {
    x.to_standard_notation_string()
}
#[cfg(target_arch = "wasm32")]
pub fn normal_string(x: Dec) -> String {
    x.to_string()
}
