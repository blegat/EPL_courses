# Continuous and discrete Fourier series

## Topics and results

- A periodic continuous-time signal of period $T$ is represented by harmonics
  $e^{jk\omega_0t}$ with $\omega_0=2\pi/T$:
  $$x(t)=\sum_{k\in\mathbb Z}c_ke^{jk\omega_0t},\qquad
  c_k=\frac1T\int_Tx(t)e^{-jk\omega_0t}\,dt.$$
  (`FOURIER`, pp. 43–54).
- Orthogonality extracts coefficients; real signals impose conjugate symmetry.
- A periodic discrete-time sequence of period $N$ uses only $N$ distinct
  harmonics, since discrete frequencies differing by $2\pi$ coincide
  (`FOURIER`, pp. 57–64).
- Parseval's identity equates average signal power with the squared Fourier
  coefficients.
- LTI response to a periodic input multiplies each harmonic coefficient by the
  system frequency response at that harmonic.

## Internal connections

- [Continuous and discrete Fourier transforms](Continuous%20and%20discrete%20Fourier%20transforms.md)
