# Particle filtering and sequential importance sampling

## Topics and results

- A weighted empirical measure
  $\sum_{i=1}^Nw^{(i)}\delta(s-s^{(i)})$ approximates a posterior density and
  its expectations by Monte Carlo sums (`SP`, pp. 151–153).
- Importance sampling draws from an accessible proposal $\pi$ and corrects the
  mismatch to a target $p$ with weights proportional to $p(s)/\pi(s)$; proposal
  support must contain target support (`SP`, pp. 154–158).
- Normalized weights avoid an unknown target normalizing constant and produce a
  discrete posterior approximation.
- Sequential importance sampling factorizes the proposal over time and updates
  particle weights recursively using the new likelihood and transition density
  (`SP`, pp. 158–160).
- Repeated weighting causes degeneracy as mass concentrates on few particles;
  effective sample size diagnoses this and resampling replaces low-weight
  particles by copies of high-weight ones (`SP`, pp. 160–162).
- Particle filtering approximates the nonlinear/non-Gaussian Bayes filter; the
  number of particles controls computation and Monte Carlo error.

## Connections

- Target recursion: [Recursive Bayesian filtering](Recursive%20Bayesian%20filtering.md)
- Monte Carlo foundation: [LDACS1110 — Monte Carlo estimation](../../DACS/1110/Monte%20Carlo%20estimation.md)
- Importance weights: [LDACS1110 — direct and weighted sampling methods](../../DACS/1110/Direct%20and%20weighted%20sampling%20methods.md)
- Vision application: [LELEC2885 — recursive appearance-based tracking](../../ELEC/2885/Recursive%20appearance-based%20tracking.md)
