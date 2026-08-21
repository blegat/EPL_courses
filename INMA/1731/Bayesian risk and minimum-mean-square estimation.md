# Bayesian risk and minimum-mean-square estimation

## Topics and results

- Bayesian estimation models the unknown parameter as random with a prior;
  Bayes' rule combines it with the likelihood to form the posterior
  $p(\theta\mid x)$ (`SP`, pp. 93–99).
- A loss function quantifies decision error and posterior expected loss defines
  conditional Bayes risk. Minimizing it pointwise in the data produces a Bayes
  estimator.
- Under squared-error loss the posterior mean
  $E[\theta\mid x]$ is the minimum-mean-square-error (MMSE) estimator
  (`SP`, pp. 100–105).
- The orthogonality principle states that the MMSE error is orthogonal to every
  square-integrable function of the observations; conditional expectation is
  an $L^2$ projection.
- For jointly Gaussian parameter and observations, the conditional mean is
  affine and its error covariance is the Gaussian conditional covariance
  (`SP`, p. 106).

## Connections

- Parametric Bayesian formulas: [Bayesian linear models, MAP, and linear MMSE](Bayesian%20linear%20models,%20MAP,%20and%20linear%20MMSE.md)
- Learning-theory treatment: [LDACS1110 — Bayesian inference](../../DACS/1110/Bayesian%20inference.md)
