# Kernel ridge regression and the representer theorem

## Topics and results

- Kernel ridge regression minimizes empirical squared loss plus an RKHS norm
  penalty and generalizes ordinary ridge regression (`APPLICATIONS`, pp. 11–13).
- The representer theorem states that, for losses depending on finitely many
  evaluations and a strictly increasing function of the RKHS norm, an optimizer
  has the form $f^*=\sum_{i=1}^N\alpha_i k(x_i,\cdot)$ (`APPLICATIONS`,
  pp. 14–19).
- Orthogonally decomposing any candidate into the span of the training feature
  vectors and its complement proves the theorem: the complement changes no
  fitted value and can only increase regularization.
- Substitution reduces an infinite-dimensional variational problem to a finite
  problem involving the kernel matrix; for squared loss this yields the usual
  regularized linear system (`APPLICATIONS`, pp. 20–23).

## Connections

- Required geometry: [Reproducing-kernel Hilbert spaces and kernel construction](Reproducing-kernel%20Hilbert%20spaces%20and%20kernel%20construction.md)
- Bayesian equivalence: [LEPL1109 — Gaussian-process regression](../../EPL/1109/Gaussian-process%20regression.md)
- Bayesian synthesis: [LDACS1110 — Gaussian-process regression as Bayesian learning](../../DACS/1110/Gaussian-process%20regression%20as%20Bayesian%20learning.md)
- Control application: [LINMA2725 — value-function approximation architectures](../2725/Value-function%20approximation%20architectures.md)
