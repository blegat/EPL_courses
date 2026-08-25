# Analog Butterworth and Chebyshev filter design

## Topics and results

- An analog rational filter must place poles for causality and stability while
  meeting passband and stopband tolerances.
- Butterworth filters are maximally flat at zero frequency, with squared
  magnitude $|H(j\Omega)|^2=1/(1+(\Omega/\Omega_c)^{2N})$ after normalization
  (`FILTER`, pp. 13–20).
- The specifications determine a minimum order; stable poles are selected from
  the Butterworth pole circle (`FILTER`, pp. 20–24).
- Chebyshev filters trade passband or stopband ripple for a sharper transition
  at a given order (`FILTER`, pp. 25–31).
- Frequency substitutions convert a normalized low-pass prototype into
  high-pass and band-pass filters (`FILTER`, pp. 32–38).
- Pole-zero geometry, realizability and numerical conditioning complement the
  magnitude-only design calculation.

## Internal connections

- [Frequency response and filtering specifications](Frequency%20response%20and%20filtering%20specifications.md)
