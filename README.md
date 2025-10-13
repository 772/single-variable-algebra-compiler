[![License: GPL3](https://img.shields.io/badge/License-GNU%20GPL-blue)](https://opensource.org/license/gpl-3-0)
[![Doc](https://docs.rs/single-variable-algebra-compiler/badge.svg)](https://docs.rs/single-variable-algebra-compiler)
[![Crate](https://img.shields.io/crates/v/single-variable-algebra-compiler.svg)](https://crates.io/crates/single-variable-algebra-compiler)

## What is single-variable-algebra-compiler?

A compiler for Single Variable Algebra (SVA) - a minimalist programming language that uses only:
- A single variable (x)
- Numbers
- Basic arithmetic (+, -, *, /, ^)
- Functions built from these elements

__Yet another programming language?__ No. While not every math expression qualifies as SVA, every valid SVA program is syntactically correct middle school mathematics. This is making it perfect for learning both programming and math concepts.

## Usage

Use https://772.github.io/single-variable-algebra-compiler/ to test this!

> [!WARNING]
> Known limitations: The WebAssembly version is limited to 25 decimal places due to the lack of a pure Rust (WASM-compatible) crate that supports arbitrary-precision arithmetic with a `powf` function.

Run locally via `cargo r -- <input>` after cloning this repository or run `single-variable-algebra-compiler <input>` after installing it via `cargo install single-variable-algebra-compiler`).

Example input:

```bash
cargo r -- "
decimals(x) = 25
abs(x) = (x^2)^(1/2)
H(x) = (x+abs(x))/(2*x)
tiny(x) = 10^(-decimals(x))
ge0(x) = H(x+tiny(x)/10)
lt1(x) = 1-ge0(x-1)
is0(x) = ge0(x)*lt1(x)
is1(x) = is0(x-1)
is2(x) = is0(x-2)
is3(x) = is0(x-3)
is4(x) = is0(x-4)
is5(x) = is0(x-5)
is6(x) = is0(x-6)
is7(x) = is0(x-7)
is8(x) = is0(x-8)
is9(x) = is0(x-9)
floor1(x) = is1(x)+2*is2(x)+3*is3(x)+4*is4(x)+5*is5(x)+6*is6(x)+7*is7(x)+8*is8(x)+9*is9(x)
right(x) = x*10-floor1(x*10)+floor1(x*10)*tiny(x)
left(x) = right(right(right(right(right(right(right(right(right(right(right(right(right(right(right(right(right(right(right(right(right(right(right(right(x))))))))))))))))))))))))
left(0.3000000000000000000000012)
"
```

Output: `0.2300000000000000000000001`

Note that some of the functions above contain performance optimizations. So using them is recommended.

## Prooving Turing completeness

Algebraic functions for the simulation of a Turing machine with d memory cells:

$$
abs(x) = (x^2)^\frac{1}{2}
$$
$$
H(x) = \frac{x+abs(x)}{2 \cdot x}
$$
$$
tiny(x) = 10^{-d}
$$
$$
ge0(x) = H(x + \frac{tiny(x)}{10})
$$
$$
lt1(x) = 1-ge0(x-1)
$$
$$
\boldsymbol{is0}(x) = ge0(x) \cdot lt1(x)
$$
$$
\boldsymbol{is1}(x) = is0(x - 1)
$$
$$
\boldsymbol{is2}(x) = is0(x - 2)
$$
$$
\boldsymbol{is3}(x) = is0(x - 3)
$$
$$
\boldsymbol{is4}(x) = is0(x - 4)
$$
$$
\boldsymbol{is5}(x) = is0(x - 5)
$$
$$
\boldsymbol{is6}(x) = is0(x - 6)
$$
$$
\boldsymbol{is7}(x) = is0(x - 7)
$$
$$
\boldsymbol{is8}(x) = is0(x - 8)
$$
$$
\boldsymbol{is9}(x) = is0(x - 9)
$$
$$
\boldsymbol{floor1}(x) = is1(x) + is2(x) + is3(x) + is4(x) + is5(x) + is6(x) + is7(x) + is8(x) + is9(x)
$$
$$
right_2(x) = floor1(x \cdot 10)
$$
$$
\boldsymbol{right}(x) = x \cdot 10 - right_2(x) + right_2(x) \cdot tiny(x)
$$
$$
left_2(x) = right(right(x))
$$
$$
left_3(x) = right(left_2(x))
$$
$$
left_4(x) = right(left_3(x))
$$
$$
...
$$
$$
left_{d-1}(x) = right(left_{d-2}(x))
$$
$$
\boldsymbol{left}(x) = left_{d-1}(x)
$$

For each function command_1 to command_m, the following holds:  
Let F be the set of all previously bolded functions and loop(x).  
f_1, f_2, ..., f_n are arbitrary functions from F or other algebraic functions.  
The functions can then be chained together.

$$
\boldsymbol{command_1}(x) = f_n( f_{n-1}( ... f_2(f_1(x)) ... ))
$$
$$
\boldsymbol{command_2}(x) = f_n( f_{n-1}( ... f_2(f_1(command_1(x))) ... ))
$$
$$
\boldsymbol{command_3}(x) = f_n( f_{n-1}( ... f_2(f_1(command_2(x))) ... ))
$$
$$
...
$$
$$
\boldsymbol{command_m}(x) = f_n( f_{n-1}( ... f_2(f_1(command_{m-1}(x))) ... ))
$$

Let k be any number from 1 to including m

$$
repeat_1(x) = command_k(command_k(command_k(x)))
$$
$$
repeat_2(x) = repeat_1(repeat_1(repeat_1(x)))
$$
$$
repeat_3(x) = repeat_2(repeat_2(repeat_2(x)))
$$
$$
...
$$
$$
repeat_{167}(x) = repeat_{166}(repeat_{166}(repeat_{166}(x)))
$$
$$
\boldsymbol{loop_k}(x) = repeat_{167}(repeat_{167}(repeat_{167}(x)))
$$

## Code examples without using the `dec` crate

floor8(x) is able to round down bigger numbers:

```rust
fn h(x: f64) -> f64 {
    (1.0 + x / (x.powf(2.0)).powf(0.5)) / 2.0
}

fn ge0(x: f64) -> f64 {
    h(x + (10.0_f64).powf(-9.0))
}

fn lt1(x: f64) -> f64 {
    1.0 - ge0(x - 1.0)
}

fn is0(x: f64) -> f64 {
    ge0(x) * lt1(x)
}

fn is1(x: f64) -> f64 {
    is0(x - 1.0)
}

fn is2(x: f64) -> f64 {
    is0(x - 2.0)
}

fn is3(x: f64) -> f64 {
    is0(x - 3.0)
}

fn is4(x: f64) -> f64 {
    is0(x - 4.0)
}

fn is5(x: f64) -> f64 {
    is0(x - 5.0)
}

fn is6(x: f64) -> f64 {
    is0(x - 6.0)
}

fn is7(x: f64) -> f64 {
    is0(x - 7.0)
}

fn is8(x: f64) -> f64 {
    is0(x - 8.0)
}

fn is9(x: f64) -> f64 {
    is0(x - 9.0)
}

fn floor1(x: f64) -> f64 {
    is1(x)
        + is2(x) * 2.0
        + is3(x) * 3.0
        + is4(x) * 4.0
        + is5(x) * 5.0
        + is6(x) * 6.0
        + is7(x) * 7.0
        + is8(x) * 8.0
        + is9(x) * 9.0
}

fn floor2(x: f64) -> f64 {
    floor1(x / 10.0) * 10.0 + floor1(x - floor1(x / 10.0) * 10.0)
}

fn floor4(x: f64) -> f64 {
    floor2(x / 10.0_f64.powf(2.0)) * 10.0_f64.powf(2.0)
        + floor2(x - floor2(x / 10.0_f64.powf(2.0)) * 10.0_f64.powf(2.0))
}

fn floor8(x: f64) -> f64 {
    floor4(x / 10.0_f64.powf(4.0)) * 10.0_f64.powf(4.0)
        + floor4(x - floor4(x / 10.0_f64.powf(4.0)) * 10.0_f64.powf(4.0))
}

fn main() {
    for x in [1.45000001, 34.0, 99887766.12378, 50000.1] {
        println!("x = {:<20}floor8(x) = {:<20}", x, floor8(x),);
    }
}
```

Result:

```
x = 1.45000001          floor8(x) = 1                   
x = 34                  floor8(x) = 34                  
x = 99887766.12378      floor8(x) = 99887766            
x = 50000.1             floor8(x) = 50000  
```

Turing machines allow infinite loops. The identity function can be used as a termination condition to mimic loops:

```rust
fn h(x: f64) -> f64 {
    (1.0 + x / (x.powf(2.0)).powf(0.5)) / 2.0
}

fn ge0(x: f64) -> f64 {
    h(x + (10.0_f64).powf(-9.0))
}

fn lt1(x: f64) -> f64 {
    1.0 - ge0(x - 1.0)
}

fn decimal_places(x: f64) -> f64 {
    lt1(x) * x + (1.0 - lt1(x)) * (x - 1.0)
}

fn true_floor(x: f64) -> f64 {
    x - loop_a_function(x, decimal_places)
}

/// This function is basically just f(f(f(f(f(f(f(f(...f(x)...)))))))). But it
/// will stop when f(x) = x.
fn loop_a_function(mut x: f64, function: fn(f64) -> f64) -> f64 {
    // u128 can’t handle numbers like 10^80, hence we are using u128::MAX.
    for _ in 0..u128::MAX {
        let new_x = function(x);
        if x == new_x {
            break;
        } else {
            x = new_x;
        }
    }
    x
}

fn main() {
    let x = 772.5530;
    println!("true_floor^[{}]({x}) = {}", u128::MAX, true_floor(x));
}
```

Result:

```
true_floor^[340282366920938463463374607431768211455](772.553) = 772
```

## Trivia

This programming language was designed in years of 2019, 2020 and 2024 by the author of this compiler. He wrote his Master Thesis about this.
