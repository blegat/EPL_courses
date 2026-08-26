# Markov-chain Monte Carlo

## Concepts

- Sampling from a target distribution known up to a normalizing constant.
- Markov transition kernel, stationary distribution, irreducibility,
  aperiodicity, recurrence, and ergodic averages.
- Burn-in, autocorrelation, effective sample size, and limitations of visual
  convergence diagnostics.
- Metropolis-Hastings proposal and accept/reject correction.
- Gibbs sampling from full conditional distributions.

## Principal results

**Detailed balance implies stationarity.** If

$$
\pi(x)P(x,y)=\pi(y)P(y,x)
$$

for all states, then $\\pi$ is stationary for $P$.

**Metropolis-Hastings acceptance probability.** For proposal $q(y\\mid x)$,

$$
\alpha(x,y)=
\min\left\{1,
\frac{\pi(y)q(x\mid y)}{\pi(x)q(y\mid x)}
\right\}.
$$

The resulting transition satisfies detailed balance with the target under the
usual support conditions.

**Gibbs invariance.** Updating one coordinate from its full conditional leaves
the joint target distribution invariant.

**Ergodic theorem, stated with assumptions.** Under appropriate irreducibility,
aperiodicity, and positive-recurrence conditions,

$$
\frac1n\sum_{t=1}^n f(X_t)\to E_\pi[f(X)]
$$

almost surely for integrable $f$.

**MCMC variance warning.** Dependent draws do not have variance
$\\sigma^2/n$; integrated autocorrelation inflates Monte Carlo variance.

## Prerequisites from LEPL1109

- Conditional and multivariate distributions:
  [dependence and multivariate probability](../../EPL/1109/Dependence%20and%20multivariate%20probability.md)
  [STAT, pp. 60-81].
- Time-series autocorrelation:
  [time series and autoregressive models](../../EPL/1109/Time%20series%20and%20autoregressive%20models.md)
  [STAT, pp. 207-220].
- Bayesian posterior distributions provide the target distributions for MCMC.

## Developments beyond LEPL1109

- Markov-chain stationarity and detailed balance.
- Dependent sampling and ergodic convergence.
- Sampling from unnormalized posterior distributions.

## Connection to foundations of cryptography

Random walks and sampling also appear in specialized areas of cryptography;
the direct connection to the course's cryptographic foundations is limited.

## References

[B3, Chs. 7-9](README.md#b3), [B4, Chs. 10-12](README.md#b4).

## Related courses

- Sequential extension: [LINMA2725 — Markov systems, invariant measures, and ergodicity](../../INMA/2725/Markov%20systems,%20invariant%20measures,%20and%20ergodicity.md)
