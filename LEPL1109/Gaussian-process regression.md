# 11. Gaussian-process regression

- Motivation: flexible nonlinear, nonparametric regression with a predictive
  distribution and uncertainty [STAT, p. 222].
- Gaussian process definition: every finite function-value vector is jointly
  normal; mean function $m(x)$, covariance kernel $k(x,x')$, and Gram
  matrix [STAT, pp. 223-224].
- Observation model $y_i=f(x_i)+\epsilon_i$,
  $\epsilon_i\sim N(0,\sigma_\epsilon^2)$, and
  $Y\sim N(0,K+\sigma_\epsilon^2I)$ [STAT, p. 225; ERR, p. 1, correction
  for slide 232].
- Joint Gaussian conditioning gives posterior mean
  $k(x_*,X)^T[K(X,X)+\sigma_\epsilon^2I]^{-1}y$ and posterior variance
  $k(x_*,x_*)-k(x_*,X)^T[K+\sigma_\epsilon^2I]^{-1}k(X,x_*)$
  [STAT, pp. 226-227; ERR, p. 1, correction for slide 227].
- The noise term both models observation noise and regularizes Gram-matrix
  inversion [STAT, p. 227].
- Valid kernels are symmetric and produce positive-semidefinite Gram matrices;
  kernels encode similarity, smoothness, and periodicity [STAT, p. 229].
- RBF, Matern, and rational-quadratic kernels and their hyperparameters [STAT,
  pp. 230-231].
- Kernel hyperparameters are fitted by maximizing Gaussian marginal likelihood;
  scikit-learn's `GaussianProcessRegressor` is demonstrated [STAT, p. 232].
