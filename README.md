# Only-Algebraic-Functions

## Functions

Algebraic functions for the simulation of a Turing machine with d memory cells:

$$
ABS(x) = (x^2)^\frac{1}{2}
$$
$$
H(x) = \frac{x+ABS(x)}{2 \cdot x}
$$
$$
TINY(x) = 10^{-d}
$$
$$
GE0(x) = H(x + \frac{TINY(x)}{10})
$$
$$
LT1(x) = 1-GE0(x-1)
$$
$$
\boldsymbol{IS0}(x) = GE0(x) \cdot LT1(x)
$$
$$
\boldsymbol{IS1}(x) = IS0(x - 1)
$$
$$
\boldsymbol{IS2}(x) = IS0(x - 2)
$$
$$
\boldsymbol{IS3}(x) = IS0(x - 3)
$$
$$
\boldsymbol{IS4}(x) = IS0(x - 4)
$$
$$
\boldsymbol{IS5}(x) = IS0(x - 5)
$$
$$
\boldsymbol{IS6}(x) = IS0(x - 6)
$$
$$
\boldsymbol{IS7}(x) = IS0(x - 7)
$$
$$
\boldsymbol{IS8}(x) = IS0(x - 8)
$$
$$
\boldsymbol{IS9}(x) = IS0(x - 9)
$$
$$
\boldsymbol{FLOOR1}(x) = IS1(x) + IS2(x) + IS3(x) + IS4(x) + IS5(x) + IS6(x) + IS7(x) + IS8(x) + IS9(x)
$$
$$
RIGHT_2(x) = FLOOR1(x \cdot 10)
$$
$$
\boldsymbol{RIGHT}(x) = x \cdot 10 - RIGHT_2(x) + RIGHT_2(x) \cdot TINY(x)
$$
$$
LEFT_2(x) = RIGHT(RIGHT(x))
$$
$$
LEFT_3(x) = RIGHT(LEFT_2(x))
$$
$$
LEFT_4(x) = RIGHT(LEFT_3(x))
$$
$$
...
$$
$$
LEFT_{d-1}(x) = RIGHT(LEFT_{d-2}(x))
$$
$$
\boldsymbol{LEFT}(x) = LEFT_{d-1}(x)
$$

For each function COMMAND_1 to COMMAND_m, the following holds:  
Let F be the set of all previously bolded functions and LOOP(x).  
f_1, f_2, ..., f_n are arbitrary functions from F or other algebraic functions.  
The functions can then be chained together.

$$
\boldsymbol{COMMAND_1}(x) = f_n( f_{n-1}( ... f_2(f_1(x)) ... ))
$$
$$
\boldsymbol{COMMAND_2}(x) = f_n( f_{n-1}( ... f_2(f_1(COMMAND_1(x))) ... ))
$$
$$
\boldsymbol{COMMAND_3}(x) = f_n( f_{n-1}( ... f_2(f_1(COMMAND_2(x))) ... ))
$$
$$
...
$$
$$
\boldsymbol{COMMAND_m}(x) = f_n( f_{n-1}( ... f_2(f_1(COMMAND_{m-1}(x))) ... ))
$$

Let k be any number from 1 to including m

$$
REPEAT_1(x) = COMMAND_k(COMMAND_k(COMMAND_k(x)))
$$
$$
REPEAT_2(x) = REPEAT_1(REPEAT_1(REPEAT_1(x)))
$$
$$
REPEAT_3(x) = REPEAT_2(REPEAT_2(REPEAT_2(x)))
$$
$$
...
$$
$$
REPEAT_{167}(x) = REPEAT_{166}(REPEAT_{166}(REPEAT_{166}(x)))
$$
$$
\boldsymbol{LOOP_k}(x) = REPEAT_{167}(REPEAT_{167}(REPEAT_{167}(x)))
$$

## Round down more than one digit before the decimal point.

$$
FLOOR2(x) = FLOOR1(\frac{x}{10}) \cdot 10 + FLOOR1(x - FLOOR1(\frac{x}{10}) \cdot 10)
$$
$$
FLOOR4(x) = FLOOR2(\frac{x}{10^2}) \cdot 10^2 + FLOOR2(x - FLOOR2(\frac{x}{10^2}) \cdot 10^2)
$$
$$
FLOOR8(x) = FLOOR4(\frac{x}{10^4}) \cdot 10^4 + FLOOR4(x - FLOOR4(\frac{x}{10^4}) \cdot 10^4)
$$
$$
...
$$

## Author

Copyright &copy; 2024 Armin Schäfer
