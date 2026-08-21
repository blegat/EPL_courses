# 2. Monte Carlo estimation

**Status:** Core.

### Concepts

- Express a target quantity as an expectation `mu = E_P[f(X)]`.
- Draw independent samples and use
  `hat(mu)_n = n^{-1} sum_i f(X_i)`.
- Separate sampling error, model error, approximation error, and floating-point
  error.
- Estimate integrals, probabilities, risks, and expected algorithmic costs.
- Confidence based on asymptotic normality versus finite-sample concentration.
- Variance reduction as the central determinant of Monte Carlo efficiency.

### Candidate results

**Unbiasedness and variance.** For i.i.d. samples with finite variance,

$$
E[\widehat\mu_n]=\mu,
\qquad
\operatorname{Var}(\widehat\mu_n)
=\frac{\operatorname{Var}(f(X))}{n}.
$$

**Weak consistency.** The law of large numbers yields
`hat(mu)_n -> mu` in probability.

**CLT uncertainty.** Under a finite-variance assumption,

$$
\sqrt n\,\frac{\widehat\mu_n-\mu}{\sigma}
\Rightarrow N(0,1).
$$

**Root-n rate.** Halving the standard Monte Carlo error requires approximately
four times as many independent samples.

**Control variate, optional.** If `E[g(X)]` is known, then

$$
\widehat\mu_c
=\frac1n\sum_i\bigl(f(X_i)-c(g(X_i)-E[g(X)])\bigr)
$$

is unbiased, with an optimal coefficient determined by covariance.

### LEPL1109 dependency

- LLN, CLT, means, and variance of averages:
  [probability foundations](../../EPL/1109/Probability%20foundations%20and%20random%20variables.md)
  and [normal approximations](../../EPL/1109/Normal%20approximations%20and%20reference%20laws.md)
  [STAT, pp. 13-18, 50-52].
- Simulation and inverse transforms:
  [simulation and bootstrap](../../EPL/1109/Simulation%20and%20bootstrap.md)
  [STAT, pp. 107-117].
- Risk and empirical risk:
  [statistical decision theory](../../EPL/1109/Statistical%20decision%20theory%20and%20Bayes%20optimality.md)
  [SL-3, pp. 5-6].

### What is new beyond LEPL1109

- Monte Carlo as a general algorithmic design pattern.
- Explicit computational accuracy/cost tradeoffs.
- Finite-sample guarantees supplied by the next section.

### FoC reuse

- Estimation of adversarial success rates in experiments.
- Correct warning: empirical testing can reveal failures but cannot establish
  computational security against all efficient adversaries.

### Bibliography

[B3, Chs. 3-4](README.md#b3), [B4, Chs. 10-11](README.md#b4), [B2](README.md#b2).

### Related courses

- Sequential application: [LINMA2470 — bandits and Monte Carlo reinforcement learning](../../INMA/2470/Bandits%20and%20Monte%20Carlo%20reinforcement%20learning.md)
- Sequential Monte Carlo application: [LELEC2885 — recursive appearance-based tracking](../../ELEC/2885/Recursive%20appearance-based%20tracking.md)
- Kernel approximation application: [LINMA2472 — Bochner's theorem and random Fourier features](../../INMA/2472/Bochner's%20theorem%20and%20random%20Fourier%20features.md)
