# Monte Carlo algorithms and error amplification

## Topics and results

- Monte Carlo integration estimates an expectation from independent samples.
  Its root-mean-square error decreases as $O(n^{-1/2})$, independently of the
  integration dimension, though the variance still matters (`S8`, pp. 2–5).
- Schwartz–Zippel bounds the probability that a nonzero multivariate polynomial
  vanishes at a uniformly sampled point; it yields randomized polynomial
  identity testing (`S8`, pp. 6–10).
- Freivalds' algorithm checks a matrix product faster than recomputing it by
  testing equality on a random vector (`S8`, pp. 11–12).
- Independent repetition multiplies one-sided failure probabilities. Majority
  voting amplifies a bounded two-sided advantage (`S8`, pp. 13–15).
- Random Max-Cut illustrates how expectation can prove the existence and
  expected quality of a randomized solution (`S8`, pp. 16–17).

## Related courses

- Statistical continuation: [LDACS1110 — Monte Carlo estimation](../../DACS/1110/Monte%20Carlo%20estimation.md)
