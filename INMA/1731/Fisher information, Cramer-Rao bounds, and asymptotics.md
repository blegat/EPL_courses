# Fisher information, Cramér–Rao bounds, and asymptotics

## Topics and results

- The score $\nabla_\theta\log p(x;\theta)$ measures local sensitivity of the
  data law to its parameter. Under regularity conditions its expectation is
  zero (`SP`, pp. 53–56).
- Fisher information is the score covariance, equivalently the negative
  expected Hessian of the log likelihood under the stated regularity
  assumptions.
- For an unbiased scalar estimator,
  $\operatorname{Var}(\hat\theta)\geq I(\theta)^{-1}$. For vector parameters,
  $\operatorname{Cov}(\hat\theta)-I(\theta)^{-1}\succeq0$ (`SP`, pp. 56–60).
- Equality characterizes an efficient MVUE through an affine relation between
  the score and estimation error. Independent observations add information.
- Convergence in distribution, probability, mean square, and almost surely
  distinguish different stochastic limits (`SP`, pp. 61–63).
- Consistency, asymptotic unbiasedness, and asymptotic normality describe
  large-sample estimator behavior; the errors-in-variables resistance example
  shows that plausible least-squares estimators can remain asymptotically
  biased (`SP`, pp. 63–67).

## Connections

- Estimator criteria: [Estimator performance and minimum-variance unbiased estimation](Estimator%20performance%20and%20minimum-variance%20unbiased%20estimation.md)
- Likelihood estimators: [Maximum likelihood and least squares](Maximum%20likelihood%20and%20least%20squares.md)
