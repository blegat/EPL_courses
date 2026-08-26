# Direct and weighted sampling methods

## Principal results

**Inverse-transform sampling.** LEPL1109 provides this direct-sampling
foundation.

**Rejection sampling.** If $p(x)\\leq Mq(x)$, draw $X\\sim q$ and accept it with
probability $\\frac{p(X)}{Mq(X)}$. The accepted sample has distribution $p$; the mean
acceptance probability is $1/M$ when densities are normalized.

**Importance-sampling identity.** If $p$ is absolutely continuous with respect
to $q$,

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

## Prerequisites from LEPL1109

- Density transformations and common distributions:
  [probability foundations](../../EPL/1109/Probability%20foundations%20and%20random%20variables.md)
  [STAT, pp. 7-23; APP, pp. 4-37].
- Inverse-transform simulation:
  [simulation and bootstrap](../../EPL/1109/Simulation%20and%20bootstrap.md)
  [STAT, pp. 107-110].

## Connection to foundations of cryptography

These methods mainly support statistical computation; their direct connection
to cryptographic foundations is limited.

## References

[B3, Chs. 3-4](README.md#b3), [B4](README.md#b4).

## Related courses

- Sequential application: [LINMA1731 — particle filtering and sequential importance sampling](../../INMA/1731/Particle%20filtering%20and%20sequential%20importance%20sampling.md)
