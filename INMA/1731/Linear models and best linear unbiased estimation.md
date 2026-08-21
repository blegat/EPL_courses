# Linear models and best linear unbiased estimation

## Topics and results

- In the Gaussian linear model $x=H\theta+w$, full column rank makes the
  parameter identifiable and yields
  $\hat\theta=(H^TC^{-1}H)^{-1}H^TC^{-1}x$ (`SP`, pp. 68–73).
- With Gaussian noise this estimator attains the Cramér–Rao bound and is the
  MVUE; polynomial fitting and impulse-response identification become choices
  of the design matrix $H$.
- BLUE restricts attention to estimators linear in the data and minimizes
  variance subject to unbiasedness. It needs only the first two noise moments,
  not a complete density (`SP`, pp. 73–78).
- For $x=H\theta+w$ with zero-mean noise of arbitrary distribution and
  covariance $C$, the Gauss–Markov theorem gives the generalized least-squares
  BLUE and covariance $(H^TC^{-1}H)^{-1}$ (`SP`, pp. 78–79).
- A linear estimator can be inadequate when the estimand is intrinsically
  nonlinear in the observations, as for estimating a variance from zero-mean
  samples.

## Connections

- Optimality criterion: [Estimator performance and minimum-variance unbiased estimation](Estimator%20performance%20and%20minimum-variance%20unbiased%20estimation.md)
- Likelihood and residual fitting: [Maximum likelihood and least squares](Maximum%20likelihood%20and%20least%20squares.md)
- Regression geometry: [LSTAT2120 — OLS estimation and projection geometry](../../STAT/2120/OLS%20estimation%20and%20projection%20geometry.md)
