# 9. Fano's inequality and information-theoretic lower bounds

**Status:** Optional; Bridge if FoC develops impossibility results through
information theory.

### Candidate results

**Fano inequality.** Let `X` take `M` values and let `hat(X)(Y)` estimate it with
error probability `P_e`. Then

$$
H(X\mid Y)\leq h_2(P_e)+P_e\log(M-1).
$$

For uniform `X`, a common consequence is

$$
P_e\geq1-\frac{I(X;Y)+\log 2}{\log M},
$$

with constants adjusted to the logarithm convention.

**Lower-bound template.** Select a finite set of well-separated hypotheses,
bound the information conveyed by observations, then use Fano to lower-bound
the probability of identification error.

### LEPL1109 dependency

- Conditional probability, classification error, and Bayes risk:
  [statistical decision theory](../LEPL1109/Statistical%20decision%20theory%20and%20Bayes%20optimality.md)
  [SL-3, pp. 7-19].
- Entropy and mutual information must first be taught in FoL.

### What is new beyond LEPL1109

- Information-theoretic impossibility and minimax lower-bound methodology.

### FoC reuse

- Reliability/secrecy tradeoffs and information-theoretic impossibility.
- Optional: FoC may instead prove Shannon-style impossibility directly, making
  Fano unnecessary in the 15-hour core.

### Bibliography

[B5, Ch. 2](content.md#b5), [B7](content.md#b7).
