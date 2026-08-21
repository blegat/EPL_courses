# Bayesian linear models, MAP, and linear MMSE

## Topics and results

- In a Bayesian linear model $x=H\theta+w$, a Gaussian prior and Gaussian noise
  produce a Gaussian posterior whose mean and covariance follow from linear
  Gaussian conditioning (`SP`, pp. 107–111).
- The maximum-a-posteriori estimator maximizes $p(\theta\mid x)$ and therefore
  combines log likelihood with a log-prior penalty (`SP`, pp. 112–114).
- MAP is the Bayes estimator for a zero–one limiting loss, whereas posterior
  mean is optimal for squared error; the two coincide for a Gaussian posterior.
- MMSE performance equals the expected posterior variance and satisfies an
  orthogonality decomposition against every competing estimator
  (`SP`, pp. 114–116).
- The linear MMSE estimator is the best affine function of the observations:
  $\hat\theta=E\theta+C_{\theta x}C_x^{-1}(x-Ex)$, requiring only first and
  second moments (`SP`, pp. 117–121).
- For jointly Gaussian variables LMMSE equals MMSE. In non-Gaussian problems it
  is optimal only within the affine class (`SP`, pp. 122–124).

## Connections

- General Bayes estimator: [Bayesian risk and minimum-mean-square estimation](Bayesian%20risk%20and%20minimum-mean-square%20estimation.md)
- Recursive linear-Gaussian use: [State-space models and Kalman filtering](State-space%20models%20and%20Kalman%20filtering.md)
- Learning interpretation: [LDACS1110 — MAP estimation and regularization](../../DACS/1110/MAP%20estimation%20and%20regularization.md)
