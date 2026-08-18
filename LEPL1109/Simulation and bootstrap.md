# 6. Simulation and bootstrap

- Pseudorandom-number generators and the linear congruential generator
  $X_{n+1}=(aX_n+c)\bmod m$ [STAT, pp. 107-108].
- Inverse-transform theorem: if $U\sim U[0,1]$, then $F^{-1}(U)$ has CDF
  $F$; simulation procedure and SciPy generation for common laws [STAT,
  pp. 109-110].
- Nonparametric bootstrap: sample with replacement from the empirical
  distribution, recompute $\hat\theta$ over $M$ replications, and estimate
  its mean, variance, sampling distribution, and percentile confidence interval
  [STAT, pp. 111-117].
- Empirical CDF $\hat F_n(x)=n^{-1}\sum_i1_{X_i\le x}$, bootstrap sample and
  replication, bootstrap variance, and percentile interval formulas [STAT,
  pp. 112-115].
- Worked result: reducing the original sample size widens the bootstrap
  interval, illustrating uncertainty reduction with more data [STAT,
  pp. 116-117].
