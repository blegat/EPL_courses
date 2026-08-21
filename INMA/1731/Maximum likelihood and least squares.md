# Maximum likelihood and least squares

## Topics and results

- Maximum likelihood chooses a parameter maximizing $p(x;\theta)$, or
  equivalently the log likelihood. It is invariant under one-to-one
  reparameterization (`SP`, pp. 79–84).
- Likelihood equations set the score to zero, but stationary points, parameter
  constraints, and nonunique maxima require explicit checking.
- Under regularity conditions, maximum-likelihood estimators are consistent,
  asymptotically normal, and asymptotically efficient (`SP`, pp. 82–84).
- Least squares minimizes $\|x-H\theta\|^2$ and gives
  $(H^TH)^{-1}H^Tx$ at full rank. Weighted least squares uses the inverse noise
  covariance (`SP`, pp. 85–88).
- For Gaussian linear noise, ML, weighted least squares, BLUE, and MVUE coincide.
  Without Gaussianity, least squares remains BLUE under Gauss–Markov assumptions
  but need not be ML or globally minimum variance (`SP`, pp. 89–92).

## Connections

- Information and asymptotics: [Fisher information, Cramér–Rao bounds, and asymptotics](Fisher%20information,%20Cramer-Rao%20bounds,%20and%20asymptotics.md)
- Linear estimator: [Linear models and best linear unbiased estimation](Linear%20models%20and%20best%20linear%20unbiased%20estimation.md)
- Introductory likelihood: [LEPL1109 — parametric estimation](../../EPL/1109/Parametric%20estimation.md)
- Gaussian regression: [LSTAT2120 — maximum likelihood in Gaussian linear models](../../STAT/2120/Maximum%20likelihood%20in%20Gaussian%20linear%20models.md)
