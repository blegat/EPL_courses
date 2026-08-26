# Fano's inequality and information-theoretic lower bounds

## Principal results

**Fano inequality.** Let $X$ take $M$ values and let $\\widehat X(Y)$ estimate it with
error probability $P_e$. Then

$$
H(X\mid Y)\leq h_2(P_e)+P_e\log(M-1).
$$

For uniform $X$, a common consequence is

$$
P_e\geq1-\frac{I(X;Y)+\log 2}{\log M},
$$

with constants adjusted to the logarithm convention.

**Lower-bound template.** Select a finite set of well-separated hypotheses,
bound the information conveyed by observations, then use Fano to lower-bound
the probability of identification error.

## Prerequisites from LEPL1109

- Conditional probability, classification error, and Bayes risk:
  [statistical decision theory](../../EPL/1109/Statistical%20decision%20theory%20and%20Bayes%20optimality.md)
  [SL-3, pp. 7-19].
- Entropy and mutual information are prerequisites for Fano's inequality.

## Developments beyond LEPL1109

- Information-theoretic impossibility and minimax lower-bound methodology.

## Connection to foundations of cryptography

- Reliability/secrecy tradeoffs and information-theoretic impossibility.

## References

[B5, Ch. 2](README.md#b5), [B7](README.md#b7).
