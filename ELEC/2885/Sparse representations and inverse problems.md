# Sparse representations and inverse problems

## Topics and results

- A signal is sparse or compressible when few coefficients in a basis or
  dictionary carry most of its energy; redundant dictionaries offer more
  flexible atoms but make coefficients non-unique (`SPA`, pp. 2–28).
- Sparse recovery seeks a coefficient vector fitting the observations with few
  nonzeros. Direct $\ell_0$ minimization is combinatorial, motivating convex
  $\ell_1$ relaxation and greedy or iterative algorithms (`SPA`, pp. 29–47).
- Basis pursuit/Lasso, matching pursuit, orthogonal matching pursuit, and
  iterative hard thresholding express different compromises between optimization
  cost, support selection, and reconstruction quality (`SPA`, pp. 30–47).
- Transform coding keeps large coefficients and quantizes or discards small ones;
  sparsity therefore links approximation quality to compression (`SPA`,
  pp. 48–54).
- Denoising works because structured signal coefficients concentrate while broad
  noise does not; threshold choice controls the bias–variance trade-off (`SPA`,
  pp. 55–64).
- Deblurring is ill-conditioned and amplifies noise. A sparsity prior regularizes
  the data-fidelity inverse problem and stabilizes recovery (`SPA`, pp. 65–79).
- Compressed sensing acquires sparse signals through fewer incoherent linear
  measurements and reconstructs them by sparse recovery; the deck connects this
  principle to coded imaging hardware (`SPA`, pp. 80–97).

## Related courses

- Structured transforms: [Multiscale pyramids and wavelets](Multiscale%20pyramids%20and%20wavelets.md)
- Greedy recovery: [Matching pursuits and adaptive dictionaries](Matching%20pursuits%20and%20adaptive%20dictionaries.md)
- Probabilistic view of penalties: [LDACS1110 — MAP estimation and regularization](../../DACS/1110/MAP%20estimation%20and%20regularization.md)
- Feature sparsity: [LELEC2870 — feature selection](../2870/Feature%20selection.md)
- Fourier-domain representation: [Digital images and Fourier analysis](Digital%20images%20and%20Fourier%20analysis.md)
