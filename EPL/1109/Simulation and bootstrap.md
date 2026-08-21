# Simulation and bootstrap

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

## Related courses

- Related LDACS1110 topic: [Computational pseudorandomness and reductions](../../DACS/1110/Computational%20pseudorandomness%20and%20reductions.md)
- Related LDACS1110 topic: [Direct and weighted sampling methods](../../DACS/1110/Direct%20and%20weighted%20sampling%20methods.md)
- Related LDACS1110 topic: [Monte Carlo estimation](../../DACS/1110/Monte%20Carlo%20estimation.md)
- Related LDACS1110 topic: [Randomized algorithms and probabilistic analysis](../../DACS/1110/Randomized%20algorithms%20and%20probabilistic%20analysis.md)
