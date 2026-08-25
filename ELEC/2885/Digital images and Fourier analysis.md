# Digital images and Fourier analysis

## Topics and results

- A digital image is a sampled, quantized multidimensional signal represented by
  a matrix; resolution, bit depth, dynamic range, and color system determine its
  basic representation (`REP`, pp. 5–14).
- Pointwise transformations and histograms describe or modify intensity without
  using spatial arrangement; contrast stretching and histogram operations expose
  the distinction between radiometry and geometry (`REP`, pp. 15–25).
- The two-dimensional DFT expands an image in complex sinusoidal basis functions.
  Conjugate symmetry, DC/AC coefficients, magnitude, and phase explain how real
  images appear in the frequency domain (`REP`, pp. 26–42).
- Spatial orientation is perpendicular to the orientation of Fourier energy;
  translations primarily affect phase, while periodic structures create localized
  spectral peaks (`REP`, pp. 43–55).
- Multiplication by a transfer function in frequency corresponds to convolution
  in space. Low-pass filtering smooths and supports anti-aliasing, whereas
  high-pass and directional filters emphasize details (`REP`, pp. 56–64).

## Related courses

- One-dimensional foundation: [LEPL1106 — continuous and discrete Fourier transforms](../../EPL/1106/Continuous%20and%20discrete%20Fourier%20transforms.md)
- Multiresolution continuation: [Multiscale pyramids and wavelets](Multiscale%20pyramids%20and%20wavelets.md)
- Sparse spectral models: [Sparse representations and inverse problems](Sparse%20representations%20and%20inverse%20problems.md)
- Statistical image summaries: [LEPL1109 — descriptive statistics and exploratory data analysis](../../EPL/1109/Descriptive%20statistics%20and%20exploratory%20data%20analysis.md)
