# Bochner's theorem and random Fourier features

## Topics and results

- Kernel matrices require $O(N^2)$ storage and kernel prediction may scale with
  the number of training points; explicit low-dimensional features replace this
  dependence by ordinary linear methods (`RFF`, pp. 1–6).
- Bochner's theorem characterizes continuous shift-invariant positive-definite
  kernels as Fourier transforms of nonnegative measures (`RFF`, pp. 7–22).
- Sampling frequencies from the normalized spectral measure and averaging
  complex exponentials gives a Monte Carlo kernel approximation. Equivalent
  real constructions use sine/cosine pairs or randomly shifted cosines
  (`RFF`, pp. 23–35).
- The resulting map $z:\mathbb R^n\to\mathbb R^D$ satisfies
  $z(x)^\top z(y)\approx k(x,y)$ and permits a standard linear estimator whose
  evaluation cost depends on $D$, not $N$.
- Hoeffding bounds control a fixed pair, while covering and Lipschitz arguments
  yield uniform high-probability approximation on compact domains
  (`RFF`, pp. 36–45).
- Gaussian and Laplace kernels correspond to Gaussian and Cauchy spectral
  sampling respectively; experiments illustrate the accuracy–cost trade-off
  as $D$ grows (`RFF`, pp. 46–52).

## Connections

- Kernel characterization: [Reproducing-kernel Hilbert spaces and kernel construction](Reproducing-kernel%20Hilbert%20spaces%20and%20kernel%20construction.md)
- Approximate classifier: [Support-vector machines and kernel classification](Support-vector%20machines%20and%20kernel%20classification.md)
- Probability tools: [LDACS1110 — Monte Carlo estimation](../../DACS/1110/Monte%20Carlo%20estimation.md)
