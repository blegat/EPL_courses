# Multiscale pyramids and wavelets

## Topics and results

- Gaussian pyramids repeatedly smooth and subsample an image; smoothing before
  decimation prevents aliasing and makes each level a coarser scale-space
  representation (`REP`, pp. 65–75).
- A Laplacian pyramid stores prediction residuals between adjacent Gaussian
  levels. The residual bands plus the coarsest approximation permit progressive
  and exact reconstruction under the stated operators (`REP`, pp. 72–90).
- Haar scaling functions produce piecewise-constant approximations, while Haar
  wavelets encode the complementary details lost between resolutions (`WAV`,
  pp. 2–32).
- Multiresolution analysis organizes nested approximation spaces and orthogonal
  detail spaces. Scaling and wavelet coefficients are inner products with
  translated and dilated atoms (`WAV`, pp. 33–42).
- The fast wavelet transform implements analysis and synthesis through conjugate
  low-pass/high-pass filter banks with downsampling and upsampling (`WAV`,
  pp. 43–56).
- Beyond Haar, regularity, compact support, symmetry, and vanishing moments trade
  off against one another. Applications include JPEG2000-style compression,
  denoising, deblurring, and inverse problems (`WAV`, pp. 57–63).

## Related courses

- Frequency-domain prerequisite: [Digital images and Fourier analysis](Digital%20images%20and%20Fourier%20analysis.md)
- Sparse use of transform coefficients: [Sparse representations and inverse problems](Sparse%20representations%20and%20inverse%20problems.md)
- Adaptive alternative: [Matching pursuits and adaptive dictionaries](Matching%20pursuits%20and%20adaptive%20dictionaries.md)
