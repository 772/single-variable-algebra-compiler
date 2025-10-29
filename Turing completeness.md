# Turing completeness

Is Single Variable Algebra (SVA) Turing complete?

TLDR:

- If we use __Very strongly zero__ or even __Iverson Brackets__ -> Yes, SVA is Turing complete.
- Otherwise -> No, SVA is Turing incomplete.

## Very strongly zero

__Very strongly zero__ is a convention where `0 * 1/0` = `0 * undefined` = `0`. In normal mathematics you would write `0 * 1/0` = `0 * undefined` = `undefined`.

That allows using functions like ge0(x) to stop recursive functions since a strong zero allows you to write `0*(0+0+0+0+0...) = 0` instead of `0*(0+0+0+0+0...) = undefined`.

## Bounded loops

If we don't use "very strongly zero" we can only use iterated functions like f^[10000000000000....000](x).

It's important to note that no physical computer can perform, for example, 10^80 iterations, as this exceeds the number of atoms in the observable universe. However, the theoretical definition of Turing completeness requires the capability for unbounded loops, not their practical execution.

On the other hand: When it comes to space, the general opinion is that Laptops and modern hardware are Turing comlete because there is so much RAM to work with.
