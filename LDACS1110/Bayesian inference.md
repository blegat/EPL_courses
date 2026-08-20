# 10. Bayesian inference

**Status:** Core.

### Concepts

- Unknown parameter as a random variable.
- Prior, likelihood, evidence/marginal likelihood, and posterior.
- Sequential updating under conditional independence.
- Posterior summaries: mean, variance, credible intervals, and MAP.
- Posterior predictive distribution and Bayesian decision rules.
- Conjugacy as a tractable example, not as a general requirement.
- Prior sensitivity and distinction between epistemic and observation
  uncertainty.
- Credible intervals versus frequentist confidence intervals.

### Candidate results

**Bayes formula for parameters.**

$$
p(\theta\mid x)
=\frac{p(x\mid\theta)p(\theta)}{p(x)},
\qquad
p(x)=\int p(x\mid\theta)p(\theta)d\theta.
$$

**Posterior prediction.**

$$
p(y_{\mathrm{new}}\mid x)
=\int p(y_{\mathrm{new}}\mid\theta,x)
p(\theta\mid x)d\theta.
$$

This reduces to `integral p(y_new|theta) p(theta|x) dtheta` when the new
observation is conditionally independent of the observed data given `theta`.

**Beta-Bernoulli conjugacy.** If
`theta ~ Beta(alpha,beta)` and `s` successes occur in `n` Bernoulli trials,

$$
\theta\mid x_{1:n}
\sim\operatorname{Beta}(\alpha+s,\beta+n-s).
$$

The posterior predictive success probability is

$$
P(X_{n+1}=1\mid x_{1:n})
=\frac{\alpha+s}{\alpha+\beta+n}.
$$

**Bayes actions.** Posterior mean minimizes posterior expected squared loss;
posterior median minimizes absolute loss; posterior mode is a MAP point estimate
under regularity/uniqueness qualifications.

### LEPL1109 dependency

- Conditional distributions and Bayes' rule:
  [random vectors and conditioning](../LEPL1109/Dependence%20and%20multivariate%20probability.md#random-vectors-and-conditioning)
  [STAT, pp. 60-71].
- Parametric models, likelihood, and MLE:
  [parametric estimation](../LEPL1109/Parametric%20estimation.md)
  [STAT, pp. 83-105].
- Bayes predictor and conditional risk:
  [statistical decision theory](../LEPL1109/Statistical%20decision%20theory%20and%20Bayes%20optimality.md)
  [SL-3, pp. 7-19].
- Confidence intervals:
  [sampling distributions](../LEPL1109/Sampling%20distributions%20and%20confidence%20intervals.md)
  [STAT, pp. 119-136].

### What is new beyond LEPL1109

LEPL1109's Bayes predictor assumes a known joint population distribution. FoL
adds uncertainty over unknown parameters, prior/posterior updating, credible
sets, and posterior prediction.

### FoC reuse

- Adversarial inference and updating beliefs from observations.
- Important boundary: cryptographic security must not rely on a favorable
  subjective prior over adversaries.

### Bibliography

[B4, Chs. 1-5](README.md#b4), [B6](README.md#b6), [B8](README.md#b8).

### Related courses

- Classification application: [LINFO2262 — Bayesian decision theory, MAP, and ROC analysis](../LINFO2262/Bayesian%20decision%20theory,%20MAP,%20and%20ROC%20analysis.md)
