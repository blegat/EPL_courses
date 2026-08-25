# Z-transform and discrete-time systems

## Topics and results

- The bilateral Z-transform is the Laurent series
  $$X(z)=\sum_{n=-\infty}^{\infty}x[n]z^{-n};$$
  evaluating it on the unit circle gives the DTFT when the ROC contains that
  circle (`Z`, pp. 4–22).
- The ROC distinguishes right-sided, left-sided and two-sided sequences and is
  essential for inversion (`Z`, pp. 23–30).
- Linearity, shifts, exponential weighting, convolution and multiplication by
  $n$ have algebraic Z-domain rules (`Z`, pp. 31–41).
- Rational transforms are inverted using partial fractions and the ROC.
- A discrete LTI transfer function satisfies $H(z)=Y(z)/U(z)$ under zero-state
  conditions, and state space gives $H(z)=C(zI-A)^{-1}B+D$ (`Z`, pp. 42–59).
- Poles describe discrete modes $\lambda^n$; causality places the ROC outside
  the outermost pole, while BIBO stability requires the unit circle in the ROC.
- The unilateral Z-transform incorporates initial conditions in difference
  equations (`Z`, pp. 60–75).

## Internal connections

- [Continuous and discrete Fourier transforms](Continuous%20and%20discrete%20Fourier%20transforms.md)
- [BIBO and internal stability](BIBO%20and%20internal%20stability.md)
- [Digital FIR and IIR filter design](Digital%20FIR%20and%20IIR%20filter%20design.md)
