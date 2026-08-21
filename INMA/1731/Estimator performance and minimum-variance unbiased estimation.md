# Estimator performance and minimum-variance unbiased estimation

## Topics and results

- A parametric model $p(x;\theta)$ treats the observations as random and the
  Fisher parameter as deterministic; an estimator $\hat\theta=g(x)$ is itself
  a random variable (`SP`, pp. 37–43).
- Bias, covariance, and mean-square error assess an estimator over repeated
  data realizations rather than from one observed error (`SP`, pp. 44–49).
- The decomposition
  $\operatorname{MSE}(\hat\theta)=\operatorname{Var}(\hat\theta)+
  \operatorname{bias}(\hat\theta)^2$ separates dispersion and systematic error.
- An estimator is unbiased when $E_\theta[\hat\theta]=\theta$ for every
  parameter value. The usual variance estimator requires the $N-1$ correction
  to be unbiased when the mean is estimated (`SP`, pp. 47–49).
- A minimum-variance unbiased estimator (MVUE) has the smallest variance among
  all unbiased estimators, although an MVUE need not exist and there is no
  general construction procedure (`SP`, pp. 50–52).

## Connections

- Probability model: [Probability and random-process foundations](Probability%20and%20random-process%20foundations.md)
- Performance lower bound: [Fisher information, Cramér–Rao bounds, and asymptotics](Fisher%20information,%20Cramer-Rao%20bounds,%20and%20asymptotics.md)
- Linear estimator class: [Linear models and best linear unbiased estimation](Linear%20models%20and%20best%20linear%20unbiased%20estimation.md)
- Introductory counterpart: [LEPL1109 — parametric estimation](../../EPL/1109/Parametric%20estimation.md)
