# 17. Gaussian-process regression as Bayesian learning

**Status:** Optional synthesis; do not reteach the LEPL1109 derivation.

### Already covered by LEPL1109

- GP prior as finite-dimensional jointly Gaussian function values.
- Mean and covariance kernel.
- Noisy observation model.
- Posterior predictive mean and variance.
- RBF, Matern, and rational-quadratic kernels.
- Marginal-likelihood hyperparameter fitting.

See [Gaussian-process regression in LEPL1109](../../EPL/1109/Gaussian-process%20regression.md)
[STAT, pp. 222-232; ERR, p. 1].

### Candidate new connections

**GP as a function prior.** Kernel choice expresses prior assumptions about
smoothness, scale, and structure.

**Posterior conditioning.** GP regression is a direct application of Bayesian
conditioning for a multivariate Gaussian model.

**Kernel ridge equivalence.** Under matching conventions, the GP posterior mean
at the training/prediction points agrees with kernel ridge regression, with the
noise/prior scale setting the regularization parameter.

**Marginal likelihood.** Hyperparameter fitting balances data fit and a
log-determinant complexity term, often called an Occam factor.

**Calibration caveat.** Posterior intervals are conditional on the kernel,
noise model, and hyperparameters; misspecification can invalidate uncertainty
claims.

### FoC reuse

Low. This is useful for the Bayesian/regularization learning outcome but should
not displace shared crypto foundations.

### Bibliography

[B16, Chs. 2, 4, and 5](README.md#b16), [B6](README.md#b6).
