# Finite fields and Reed-Solomon codes

## Topics and results

- Extension fields $\mathbb F_{2^m}$ are represented using polynomials modulo
  an irreducible degree-$m$ polynomial (`LINEAR`, pp. 24–32).
- Frobenius conjugates and minimal polynomials connect extension-field elements
  to binary polynomial descriptions (`LINEAR`, pp. 33–40).
- A Reed–Solomon code evaluates a polynomial of degree below $k$ at $n$ distinct
  field points. It has parameters $[n,k,n-k+1]$ (`RS`, pp. 3–8).
- The Singleton bound gives $d\leq n-k+1$; Reed–Solomon codes attain it and are
  maximum-distance separable (`RS`, pp. 9–12).
- Syndromes determine an error-locator/evaluator key equation; solving it finds
  error positions and values (`RS`, pp. 17–23).
- Symbol-level distance makes these codes effective against burst errors, and
  concatenation combines them with inner binary codes.

## Related courses

- Algebraic prerequisite: [LEPL1108 — modular arithmetic, finite fields, and coding](../../EPL/1108/Modular%20arithmetic,%20finite%20fields,%20and%20coding.md)

## Internal connections

- [Linear codes, syndromes, and Hamming codes](Linear%20codes%2C%20syndromes%2C%20and%20Hamming%20codes.md)
- [Code selection and parameter design](Code%20selection%20and%20parameter%20design.md)
- [Secret sharing and secure function evaluation](Secret%20sharing%20and%20secure%20function%20evaluation.md)
