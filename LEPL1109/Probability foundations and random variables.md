# 1. Probability foundations and random variables

### Random variables and distributions

- Experiment, sample space, events, probability, random variable as a map
  `X: Omega -> R`, realization, range/state space, and discrete versus
  continuous random variables [STAT, pp. 7-8].
- PMF, PDF, and CDF:
  $p(x)=P(X=x)$, $P(X\in I)=\sum_{x\in I}p(x)$,
  $P(X\in I)=\int_I f(x)dx$, and $F(x)=P(X\le x)$, with
  $f=F'$ in the continuous case [STAT, pp. 9-12].
- Affine density transformation: if $Y=a+bX$, then
  $f_Y(y)=|b|^{-1}f_X((y-a)/b)$ [STAT, pp. 22-23].

### Expectation, moments, and variability

- Discrete/continuous expectation; empirical mean; center-of-mass
  interpretation; linearity; expectation of a transformed variable; raw
  moments $E[X^k]$ [STAT, pp. 13-15].
- Students are warned that generally $E[h(X)]\ne h(E[X])$ and
  $E[XY]\ne E[X]E[Y]$ [STAT, p. 15].
- Variance and standard deviation:
  $V(X)=E[(X-E[X])^2]=E[X^2]-E[X]^2$, affine-transformation rules, and
  sample variance $S^2=(n-1)^{-1}\sum_i(X_i-\bar X)^2$ [STAT, pp. 16-17].
- Moment-generating functions: $M_X(t)=E[e^{tX}]$, recovery of moments from
  derivatives at zero, $M_{aX+b}(t)=e^{bt}M_X(at)$, products for sums of
  independent variables, and characterization of a distribution [FORM, p. 1].
- Law of large numbers for uncorrelated variables with common finite mean and
  variance: $\bar X_n\to\mu$ in probability, supported by
  $E\bar X_n=\mu$ and $V(\bar X_n)=\sigma^2/n$ [STAT, p. 18].
- Quantiles: in general $q_p=\inf\{x:F(x)\ge p\}$. Under suitable continuity
  and invertibility assumptions, $F(q_p)=p$ and $q_p=F^{-1}(p)$. The course
  develops the upper-tail relation and an exponential-lifetime calculation
  [STAT, pp. 19-21].

### Assumed distribution family

The following distributions, including their support, PMF/PDF, mean, variance,
and listed transformation/additivity properties, are assumed mastered [APP,
p. 2]:

- Discrete uniform [APP, p. 4].
- Bernoulli [APP, p. 5].
- Binomial: independent-trial construction, PMF, sum of Bernoulli variables,
  additivity at common $p$, MGF, mean, and variance [APP, pp. 6-9].
- Geometric: waiting time to first success, PMF, MGF, mean, and variance [APP,
  p. 10].
- Poisson: count interpretation, PMF, MGF, mean/variance $\lambda$, Poisson
  limit, and approximation $Bin(n,p)\approx Po(np)$ for large $n$, small
  $p$ [APP, pp. 11-12].
- Continuous uniform: PDF, CDF, MGF, mean, and variance [APP, p. 14].
- Normal and standard normal: density, shape, parameters, MGF, tail quantiles,
  symmetry, standardization, and affine transformations [APP, pp. 15-23].
- Exponential: nonnegative waiting-time model, density, MGF, mean, and variance
  [APP, p. 24].
- Gamma: density, gamma function, shape/scale parameters, exponential special
  case, MGF, scaling, and additivity at common scale [APP, pp. 25-27].
- Chi-square as a sum of squared independent standard normals and as
  $Gamma(n/2,2)$, with mean $n$ and variance $2n$ [APP, pp. 28-29].
- Student $t_n=Z/\sqrt{Y/n}$ for independent $Z\sim N(0,1)$ and
  $Y\sim\chi_n^2$, including symmetry and convergence to normal [APP,
  pp. 30-31].
- Fisher-Snedecor $F=(X/n_1)/(Y/n_2)$, reciprocal property, and use in
  variance comparison [APP, pp. 32-33].
- Bivariate and multivariate normal distributions, covariance parameterization,
  normal linear combinations, zero-correlation/independence equivalence in the
  jointly normal case, and affine closure [APP, pp. 34-37].
