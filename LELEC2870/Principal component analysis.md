# Principal component analysis

## Topics and results

- PCA is an orthogonal linear change of coordinates that orders components by
  decreasing variance; it supports decorrelation, whitening, compression,
  denoising, visualization, and orthogonal least squares (`PCA`, pp. 3–7).
- The derivation reviews random vectors, covariance/correlation, sample
  centering, and positive-semidefinite covariance matrices (`PCA`, pp. 8–19).
- The spectral theorem diagonalizes the covariance matrix. Principal directions
  are its orthonormal eigenvectors and component variances are the corresponding
  eigenvalues (`PCA`, pp. 20–31; `PCAdraft`, pp. 4–6).
- The leading $q$-dimensional principal subspace simultaneously maximizes
  retained variance and minimizes squared orthogonal reconstruction error
  (`PCA`, pp. 32–38; `PCAdraft`, pp. 5–7).
- Whitening additionally rescales components by inverse square-root eigenvalues;
  zero or tiny eigenvalues require truncation or care (`PCA`, pp. 39–42).
- SVD provides a numerically useful route, and the Gram-matrix formulation helps
  when observations are fewer than dimensions (`PCA`, pp. 43–47;
  `PCAdraft`, pp. 8–9).

## Related courses

- Prerequisite: [LEPL1109 — unsupervised learning](../LEPL1109/Unsupervised%20learning.md)
- Probability foundation: [LEPL1109 — dependence and multivariate probability](../LEPL1109/Dependence%20and%20multivariate%20probability.md)
- Nonlinear extension: [Nonlinear dimensionality reduction and quality assessment](Nonlinear%20dimensionality%20reduction%20and%20quality%20assessment.md)
- Independence-oriented extension: [Independent component analysis](Independent%20component%20analysis.md)
