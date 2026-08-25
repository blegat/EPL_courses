# Discrete Fourier transform and fast multiplication

## Topics and results

- A polynomial may be represented by coefficients or by values at sufficiently
  many distinct points; evaluation and interpolation convert between them
  (`S3`, pp. 7–10).
- Polynomial multiplication is coefficient convolution, but becomes pointwise
  multiplication in a value representation.
- The DFT evaluates at roots of unity. The inverse transform uses the conjugate
  Fourier matrix scaled by $1/n$ (`S4`, pp. 3–4).
- The FFT separates even and odd coefficients and reuses half-size transforms,
  giving $T(n)=2T(n/2)+O(n)=O(n\log n)$ (`S4`, pp. 5–7).
- Evaluation, pointwise multiplication and interpolation therefore multiply
  degree-$n$ polynomials quasi-linearly; Schönhage–Strassen transfers this idea
  to large integers (`S4`, pp. 8–9).
