# Continuous and discrete Fourier transforms

## Topics and results

- The continuous-time Fourier transform pair is
  $$X(\omega)=\int_{-\infty}^{\infty}x(t)e^{-j\omega t}\,dt,
  \qquad x(t)=\frac1{2\pi}\int_{-\infty}^{\infty}X(\omega)e^{j\omega t}\,d\omega.$$
  (`FOURIER`, pp. 17–27, 138–145).
- The DTFT $X(e^{j\omega})=\sum_nx[n]e^{-j\omega n}$ is periodic with period
  $2\pi$ (`FOURIER`, pp. 29–41).
- Linearity, time/frequency shifts, scaling, conjugation and differentiation
  translate signal operations into spectral operations (`FOURIER`, pp. 66–108).
- Convolution in time becomes multiplication in frequency; multiplication in
  time becomes frequency-domain convolution.
- Parseval/Plancherel relates signal energy to spectral energy. Duality links
  transform pairs.
- Generalized transforms include impulses, allowing constants, sinusoids and
  periodic signals to be represented spectrally (`FOURIER`, pp. 109–137).

## Related courses

- Two-dimensional application: [LELEC2885 — digital images and Fourier analysis](../../ELEC/2885/Digital%20images%20and%20Fourier%20analysis.md)
