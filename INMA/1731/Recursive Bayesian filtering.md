# Recursive Bayesian filtering

## Topics and results

- A hidden Markov state model assumes the next state depends on the past only
  through the current state and that a measurement is conditionally independent
  of other variables given its state (`SP`, pp. 143–146).
- The prediction step applies Chapman–Kolmogorov:
  $p(s_n\mid x_{0:n-1})=\int p(s_n\mid s_{n-1})
  p(s_{n-1}\mid x_{0:n-1})\,ds_{n-1}$.
- The update multiplies the prediction by the measurement likelihood and
  normalizes it, producing $p(s_n\mid x_{0:n})$ (`SP`, pp. 147–150).
- This Bayes filter is exact in principle for nonlinear and non-Gaussian models,
  but its integrals and full density representations are usually intractable.
- The Kalman filter is the closed-form linear-Gaussian specialization; finite
  state hidden Markov filters are the discrete specialization (`SP`, p. 151).

## Connections

- Linear-Gaussian specialization: [State-space models and Kalman filtering](State-space%20models%20and%20Kalman%20filtering.md)
- Sampling approximation: [Particle filtering and sequential importance sampling](Particle%20filtering%20and%20sequential%20importance%20sampling.md)
