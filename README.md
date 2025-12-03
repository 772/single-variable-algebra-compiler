[![License: GPL3](https://img.shields.io/badge/License-GNU%20GPL-blue)](https://opensource.org/license/gpl-3-0)
[![Doc](https://docs.rs/single-variable-algebra-compiler/badge.svg)](https://docs.rs/single-variable-algebra-compiler)
[![Crate](https://img.shields.io/crates/v/single-variable-algebra-compiler.svg)](https://crates.io/crates/single-variable-algebra-compiler)

## What is single-variable-algebra-compiler?

A compiler for Single Variable Algebra (SVA) - a minimalist programming language that uses only:
- A single variable (x)
- Numbers
- Basic arithmetic (+, -, *, /, ^)
- Functions built from these elements

__Yet another programming language?__ No. Every valid SVA program is syntactically correct middle school mathematics. This makes it perfect for learning both programming and mathematical concepts.

But don't let the simple syntax fool you. To demonstrate its underlying power, the [online compiler](https://772.github.io/single-variable-algebra-compiler/) can even transform 10-state Turing machines into pure algebraic expressions with a single variable.

SVA uses the concept of [Accelerated Simulators](https://wiki.bbchallenge.org/wiki/Accelerated_simulator). An Accelerated Simulator is a program that simulates Turing machines much faster than traditional step-by-step simulation. However, the Accelerated Simulator in the SVA compiler is still very limited. It only works when all functions follow these precise patterns to enable significant simulation speedups: Copy all example functions from `decimals(x) = g` to `left(x) = right^[g-1](x)` (using a constant in the square brackets, not an expression).

## Usage

Use the [SVA Playground (WebAssembly)](https://772.github.io/single-variable-algebra-compiler/), run locally via `cargo r -- <input>` after cloning this repository or run `single-variable-algebra-compiler <input>` after installing it via `cargo install single-variable-algebra-compiler`).

Example input:

```bash
cargo r -- "
decimals(x) = 50
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
left(x) = right^[49](x)
tm(x) = is0(x)*x+is1(x)*(is0(10*(x-1))*(2+right(x-1+0.1))+is1(10*(x-1))*(2+left(x-1-0.1+0.1)))+is2(x)*(is0(10*(x-2))*(1+left(x-2+0.1))+is1(10*(x-2))*(3+left(x-2-0.1+0.0)))+is3(x)*(is0(10*(x-3))*(0+right(x-3+0.1))+is1(10*(x-3))*(4+left(x-3-0.1+0.1)))+is4(x)*(is0(10*(x-4))*(4+right(x-4+0.1))+is1(10*(x-4))*(1+right(x-4-0.1+0.0)))
f(x) = tm^[10000](x)
f(1)
"
```

Output:

`0.01111111111110000000000000000000000000000000000001`

## Trivia

- This programming language was designed in years of 2019, 2020 and 2024 by the author of this compiler. He wrote his Master Thesis about this.
- There is a lot of room for improvement: Simply renaming functions like left(x) will remove the performance optimization for it. It is planned to have a better expression simplifications in the future.
