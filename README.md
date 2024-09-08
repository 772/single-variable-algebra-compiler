# clay-tablet-computer

## Download

TBD.

## Background

The following functions simulate a Turing machine with a memory tape of length 27. The functions should be defined in the given order. Additionally, the identity function should be used as the termination condition for the function LOOP(x). The initial adjustment of the tape size is done by adjusting the functions TINY(x) and LEFT(x). Currently, the function LEFT(x) calls RIGHT(x) 26 times (tape length minus one).

```math
ABS(x) = (x^2)^\frac{1}{2} \\
H(x) = \frac{x+ABS(x)}{2 \cdot x} \
TINY(x) = 10^{-27} \\
GE0_2(x) = \frac{x+TINY(x)}{10} \\
LT1(x) = H(1-GE0(x-1)) \\
IS0(x) = GE0(x) \cdot LT1(x) \\
IS1(x) = IS0(x-1) \\
IS2(x) = IS0(x-2) \\
IS3(x) = IS0(x-3) \\
IS4(x) = IS0(x-4) \\
IS5(x) = IS0(x-5) \\
IS6(x) = IS0(x-6) \\
IS7(x) = IS0(x-7) \\
IS8(x) = IS0(x-8) \\
IS9(x) = IS0(x-9) \\
FLOOR1_5(x) = 8 \cdot IS8(x)+9 \cdot IS9(x) \\
FLOOR1_4(x) = 6 \cdot IS6(x)+7 \cdot IS7(x) \\
FLOOR1_3(x) = 4 \cdot IS4(x)+5 \cdot IS5(x) \\
FLOOR1_2(x) = 2 \cdot IS2(x)+3 \cdot IS3(x) \\
FLOOR1(x) = IS1(x) + FLOOR1_2(x) + FLOOR1_3(x)+FLOOR1_4(x) + FLOOR1_5(x) \\
RIGHT_2(x) = FLOOR1(x \cdot 10) \\
RIGHT(x) = x \cdot 10-RIGHT_2(x)+RIGHT_2(x) \cdot TINY(x) \\
LEFT_4(x) = RIGHT(RIGHT(x)) \\
LEFT_3(x) = LEFT_4(LEFT_4(LEFT_4(x))) \\
LEFT_2(x) = LEFT_4(LEFT_4(LEFT_3(x))) \\
LEFT(x) = LEFT_3(LEFT_2(LEFT_2(x))) \\
COMMAND_1(x) = ... \\
COMMAND_2(x) = ... COMMAND_1(x) ... \\
COMMAND_3(x) = ... COMMAND_2(x) ... \\
COMMAND_4(x) = ... COMMAND_3(x) ... \\
... \\
COMMAND_n(x) = ... COMMAND_{n-1}(x) ... \\
REPEAT_1(x) = COMMAND_n(COMMAND_n(COMMAND_n(x))) \\
REPEAT_2(x) = REPEAT_1(REPEAT_1(REPEAT_1(x))) \\
REPEAT_3(x) = REPEAT_2(REPEAT_2(REPEAT_2(x))) \\
... \\
REPEAT_167(x) = REPEAT_166(REPEAT_166(REPEAT_166(x))) \\
LOOP(x) = REPEAT_167(REPEAT_167(REPEAT_167(x))) \\
```

## Copyright

Copyright &copy; 2024 Armin Schäfer
