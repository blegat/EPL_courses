# Recursive appearance-based tracking

## Topics and results

- Track-to-detect methods initialize each frame from the previous estimate,
  providing low latency when appearance and motion change gradually but risking
  drift after a bad update (`T2D`, pp. 2–6).
- Lucas–Kanade template matching linearizes image motion, uses spatial gradients
  to solve a local least-squares update, and iterates from a sufficiently close
  initialization (`T2D`, pp. 7–10).
- Kernel-based tracking represents appearance with a weighted color histogram.
  Mean shift follows the gradient of a nonparametric similarity/density objective
  toward the target mode (`T2D`, pp. 11–30).
- State-space models separate uncertain dynamics from noisy observations.
  Bayesian recursive filtering alternates prediction and measurement update of
  the posterior state distribution (`T2D`, pp. 31–41).
- Linear Gaussian models yield the Kalman filter, whose mean/covariance recursion
  is exact under those assumptions (`T2D`, pp. 42–46).
- Particle filters approximate general nonlinear or non-Gaussian posteriors with
  weighted samples; propagation, weighting, normalization, and resampling form
  each tracking step (`T2D`, pp. 47–54).

## Related courses

- Monte Carlo foundation: [LDACS1110 — Monte Carlo estimation](../../DACS/1110/Monte%20Carlo%20estimation.md)
- Geometric correspondences: [Stereo vision and view interpolation](Stereo%20vision%20and%20view%20interpolation.md)
- Alternative tracking paradigm: [Detection-based multi-object tracking](Detection-based%20multi-object%20tracking.md)
