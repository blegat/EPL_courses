# 4. Direct and weighted sampling methods

**Status:** Optional.

### Candidate results

**Inverse-transform sampling.** Already covered by LEPL1109; use only as a
starting point.

**Rejection sampling.** If `p(x) <= M q(x)`, draw `X ~ q` and accept it with
probability `p(X)/(M q(X))`. The accepted sample has distribution `p`; the mean
acceptance probability is `1/M` when densities are normalized.

**Importance-sampling identity.** If `p` is absolutely continuous with respect
to `q`,

$$
E_p[f(X)]
=E_q\left[f(X)\frac{p(X)}{q(X)}\right].
$$

This gives an unbiased estimator when the normalizing constants and support
conditions permit it.

**Self-normalized importance sampling.** Useful when only unnormalized target
weights are known, but generally biased at finite sample size.

**Importance-weight variance.** A poor proposal with heavy or highly variable
weights can make the estimator unusable; effective sample size is only a
diagnostic approximation.

### LEPL1109 dependency

- Density transformations and common distributions:
  [probability foundations](../LEPL1109/Probability%20foundations%20and%20random%20variables.md)
  [STAT, pp. 7-23; APP, pp. 4-37].
- Inverse-transform simulation:
  [simulation and bootstrap](../LEPL1109/Simulation%20and%20bootstrap.md)
  [STAT, pp. 107-110].

### FoC reuse

Low direct value for the listed cryptography syllabus.

### Bibliography

[B3, Chs. 3-4](content.md#b3), [B4](content.md#b4).
