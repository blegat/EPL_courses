# 5. Parametric estimation

### Estimators and their quality

- Parametric family $f(x\mid\theta)$, i.i.d. random sample, estimator as a
  statistic, and estimate as its observed value [STAT, pp. 83-85].
- Unbiasedness, bias, estimator variance, MSE, and
  $MSE(\hat\theta)=Bias(\hat\theta)^2+V(\hat\theta)$ [STAT, pp. 86-87].
- Model-family selection is illustrated by comparing empirical histograms to
  candidate distributions and respecting support [STAT, p. 84].

### Method of moments

- Match $d$ theoretical moments $E[X^k]$ to empirical moments
  $n^{-1}\sum_iX_i^k$ and solve for a $d$-parameter model [STAT,
  pp. 88-89].
- Worked estimators for exponential, normal, gamma, Bernoulli, and bivariate
  normal parameters [STAT, pp. 90-93].
- Sample mean unbiasedness and CLT sampling approximation; consistency of
  moment estimators; ease versus lower statistical efficiency than MLE [STAT,
  pp. 94-95].

### Maximum likelihood

- Likelihood $L(\theta)=\prod_i f(x_i\mid\theta)$, log-likelihood as a sum,
  score equations, and comparison of candidate fits by likelihood [STAT,
  pp. 97-99].
- MLE asymptotics: under suitable regularity conditions, asymptotic
  unbiasedness, normality, and efficiency among the relevant regular
  asymptotically unbiased estimators. No general finite-sample minimum-variance
  result is established [STAT, p. 100].
- MLEs for exponential, Bernoulli, Poisson, and normal models [STAT,
  pp. 99, 101-105].
- Normal variance MLE uses denominator $n$ and has expectation
  $(n-1)\sigma^2/n$; replacing it by denominator $n-1$ gives the unbiased
  sample variance [STAT, pp. 104-105].
