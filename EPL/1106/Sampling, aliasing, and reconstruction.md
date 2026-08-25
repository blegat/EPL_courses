# Sampling, aliasing, and reconstruction

## Topics and results

- Ideal sampling multiplies a continuous signal by an impulse train. In the
  frequency domain this creates periodically repeated spectra (`FOURIER`,
  pp. 149–152).
- If $x$ is bandlimited to $|\omega|<\omega_m$, sampling above the Nyquist rate
  $\omega_s>2\omega_m$ prevents spectral overlap.
- Under this condition, ideal low-pass filtering reconstructs the signal,
  equivalently through sinc interpolation:
  $$x(t)=\sum_{n\in\mathbb Z}x(nT_s)
  \operatorname{sinc}\!\left(\frac{t-nT_s}{T_s}\right).$$
- Below the Nyquist rate, shifted spectral copies overlap and distinct
  continuous frequencies produce the same samples: aliasing (`FOURIER`,
  pp. 153–158).
- An analog anti-aliasing filter limits bandwidth before sampling; practical
  reconstruction replaces the ideal brick-wall filter by an approximation.
