> [!WARNING]
> The compiler is work in progress.

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

Use the [SVA Playground (WebAssembly)](https://772.github.io/single-variable-algebra-compiler/), run locally via `cargo r -- <input>` after cloning this repository or run `single-variable-algebra-compiler <input>` after installing it via `cargo install single-variable-algebra-compiler`).

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

Output:

`0.2300000000000000000000001`

It is recommended to use the functions mentioned above, as they trigger performance optimizations during runtime. The value of `decimals(x)` is adjustable. The core concept of SVA is that these functions enable Turing-complete programming.

## Code examples without using any third-party crate

The following Rust code implements a function called floor8(x) and can be run on the [Rust Playground](https://play.rust-lang.org/):

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

## Trivia

This programming language was designed in years of 2019, 2020 and 2024 by the author of this compiler. He wrote his Master Thesis about this.
