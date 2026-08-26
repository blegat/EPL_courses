# Shannon entropy and conditional entropy

## Concepts

- Self-information $-\\log p(x)$.
- Shannon entropy in bits when logarithms have base 2.
- Joint entropy, conditional entropy, and entropy rate only if needed.
- Entropy as average uncertainty, distinct from variance and from worst-case
  unpredictability.
- Compression as an operational motivation; coding theorems can be stated but
  need not be proved.

## Principal results

**Shannon entropy.** For a discrete random variable,

$$
H(X)=-\sum_x p(x)\log p(x).
$$

**Bounds and equality cases.** If $X$ takes values in a finite alphabet,

$$
0\leq H(X)\leq\log|\mathcal X|,
$$

with maximum entropy at the uniform distribution.

**Chain rule.**

$$
H(X,Y)=H(X)+H(Y\mid X).
$$

**Conditioning reduces entropy.**

$$
H(X\mid Y)\leq H(X).
$$

**Independent additivity.** If $X$ and $Y$ are independent,
$H(X,Y)=H(X)+H(Y)$.

**Entropy of a function.** For deterministic $g$,
$H(g(X))\\leq H(X)$.

## Prerequisites from LEPL1109

- Discrete distributions and expectation of functions:
  [probability foundations](../../EPL/1109/Probability%20foundations%20and%20random%20variables.md)
  [STAT, pp. 7-15].
- Joint and conditional distributions:
  [random vectors and conditioning](../../EPL/1109/Dependence%20and%20multivariate%20probability.md#random-vectors-and-conditioning)
  [STAT, pp. 60-71].
- Logistic cross-entropy is known as a loss, but entropy theory is not:
  [logistic regression](../../EPL/1109/Logistic%20regression%20and%20classification%20assessment.md#logistic-regression)
  [SL-2, pp. 33-35].

## Developments beyond LEPL1109

- Entropy and its chain rules.
- Quantitative uncertainty accounting.
- Operational link to compression and information.

## Connection to foundations of cryptography

- Perfect secrecy and key uncertainty.
- Leakage and information-theoretic security.
- Impossibility and key-length lower bounds.

## References

[B5, Ch. 2](README.md#b5), [B6, Parts I-II](README.md#b6).

## Related courses

- Coding-theory treatment: [LELEC2348 — entropy, conditional entropy, and mutual information](../../ELEC/2348/Entropy,%20conditional%20entropy,%20and%20mutual%20information.md)
- Perfect-secrecy application: [LELEC2348 — perfect secrecy, one-time pads, and wiretap channels](../../ELEC/2348/Perfect%20secrecy,%20one-time%20pads,%20and%20wiretap%20channels.md)
