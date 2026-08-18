# 1. Randomized algorithms and probabilistic analysis

**Status:** Core, Bridge.

### Concepts

- Probability space attached to an algorithm's internal random choices.
- Random seed and random bits; deterministic behavior conditional on the seed.
- Monte Carlo algorithms: bounded running time with a controlled probability of
  an incorrect answer.
- Las Vegas algorithms: always correct, with random running time.
- One-sided versus two-sided error.
- Expected running time, failure probability, and adversarial versus average
  inputs.
- Failure-probability amplification by independent repetition and majority vote.
- Indicator variables and linearity of expectation for algorithm analysis.
- Universal hashing as the principal randomized-data-structure example.

### Candidate results

**Union bound.** For events `A_1, ..., A_m`,

\[
P\left(\bigcup_{i=1}^m A_i\right)\leq \sum_{i=1}^m P(A_i).
\]

**Amplification.** If independent runs fail with probability `p < 1/2`, a
majority of `r` runs has exponentially decreasing failure probability; a
Hoeffding or Chernoff bound makes this quantitative.

**Linearity of expectation.** Independence is unnecessary for
`E[sum_i X_i] = sum_i E[X_i]`, which supports collision, occupancy, and running
time analyses.

**Birthday bound.** For `q` independent uniform samples from a set of size `N`,
the collision probability satisfies

\[
P(\text{collision})
=1-\prod_{i=0}^{q-1}\left(1-\frac{i}{N}\right)
\leq \frac{q(q-1)}{2N}.
\]

The transition from unlikely to likely collision occurs around `q = Theta(sqrt(N))`.

### LEPL1109 dependency

- Random variables, expectation, indicators, and variance:
  [probability foundations](../LEPL1109/Probability%20foundations%20and%20random%20variables.md)
  [STAT, pp. 7-18; SL-1, pp. 4-7 for indicator notation].
- Independence and covariance:
  [dependence and multivariate probability](../LEPL1109/Dependence%20and%20multivariate%20probability.md)
  [STAT, pp. 38-45].
- Numerical pseudorandom generators and seeds:
  [simulation and bootstrap](../LEPL1109/Simulation%20and%20bootstrap.md)
  [STAT, pp. 107-110].
- Algorithm/data-structure prerequisites mainly come from LEPL1402, not
  LEPL1109.

### What is new beyond LEPL1109

- Algorithms as probability experiments.
- Explicit failure probabilities and amplification.
- Randomness as a computational resource.
- Birthday-scale collision behavior.

### FoC reuse

- Security games, probabilistic adversaries, and reduction success probability.
- Hash collisions and random-oracle query bounds.
- PRF/PRP switching bounds and block-cipher analysis.
- Amplification and accounting for multiple bad events.

### Bibliography

[B1, Chs. 5 and 11](content.md#b1), [B2, Chs. 3-5](content.md#b2), [B11](content.md#b11).
