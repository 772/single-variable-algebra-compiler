# Only-Algebraic-Functions

## Functions

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

## Round down more than one digit before the decimal point

$$
floor2(x) = floor1(\frac{x}{10}) \cdot 10 + floor1(x - floor1(\frac{x}{10}) \cdot 10)
$$
$$
floor4(x) = floor2(\frac{x}{10^2}) \cdot 10^2 + floor2(x - floor2(\frac{x}{10^2}) \cdot 10^2)
$$
$$
floor8(x) = floor4(\frac{x}{10^4}) \cdot 10^4 + floor4(x - floor4(\frac{x}{10^4}) \cdot 10^4)
$$
$$
...
$$

## Author

Copyright &copy; 2024 Armin Schäfer
