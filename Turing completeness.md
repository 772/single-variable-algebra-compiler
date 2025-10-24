# Turing completeness

Is Single Variable Algebra (SVA) Turing complete?

TLDR:

- If we use __Very strongly zero__ or even __Iverson Brackets__ -> Yes, SVA is Turing complete.
- Otherwise -> No, SVA is Turing incomplete.

## Very strongly zero

__Very strongly zero__ is a convention where `0 * 1/0` = `0 * undefined` = `0`. In normal mathematics you would write `0 * 1/0` = `0 * undefined` = `undefined`.

## Bounded loops

It's important to note that no physical computer can perform, for example, 10^80 iterations, as this exceeds the number of atoms in the observable universe. However, the theoretical definition of Turing completeness requires the capability for unbounded loops, not their practical execution.
