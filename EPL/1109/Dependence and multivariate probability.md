# 3. Dependence and multivariate probability

### Independence, covariance, and correlation

- Independence as factorization of joint probabilities/densities [STAT, p. 38].
- Covariance
  $C(X,Y)=E[(X-E[X])(Y-E[Y])]=E[XY]-E[X]E[Y]$, empirical covariance,
  bilinearity, symmetry, and
  $V(aX+bY)=a^2V(X)+b^2V(Y)+2abC(X,Y)$ [STAT, pp. 39-42].
- Independence implies zero covariance, but zero covariance need not imply
  independence; the course uses $Y=X^2$ as a counterexample [STAT, p. 42].
- Correlation $\rho=C(X,Y)/(\sigma_X\sigma_Y)$, empirical correlation,
  Cauchy-Schwarz bound $|\rho|\le1$, scale invariance, affine equality case,
  and the limitation to linear dependence [STAT, pp. 43-45].

### Random vectors and conditioning

- Random vectors, joint continuous/discrete distributions, marginalization by
  integration/summation, and conditional PDF/PMF [STAT, pp. 60-66].
- Mean vector, expectation of multivariate functions, vector linearity, and
  conditional expectation [STAT, pp. 67-68].
- Tower property $E[X_A]=E[E(X_A\mid X_B)]$ and MMSE result
  $E[X_A\mid X_B]=\arg\min_gE[\|X_A-g(X_B)\|_2^2]$, with an ordinary square
  in the scalar case [STAT, pp. 69-71].
- Covariance matrix
  $\Sigma=E[(X-\mu)(X-\mu)^T]$, entries as variances/covariances, and affine
  transformation $\Sigma_{MX+m}=M\Sigma_XM^T$ [STAT, pp. 71-73].
- Mutual independence by factorization; consequences for conditioning and a
  diagonal covariance matrix [STAT, pp. 73-75].
- Multivariate normal density, affine closure, marginal normals, whitening, and
  diagonal-covariance/independence equivalence [STAT, pp. 76-78].
- Conditional multivariate-normal theorem. For a partition into $A,B$,
  $X_B\mid X_A=x_A$ has mean
  $\mu_B+\Sigma_{BA}\Sigma_{AA}^{-1}(x_A-\mu_A)$ and covariance
  $\Sigma_{BB}-\Sigma_{BA}\Sigma_{AA}^{-1}\Sigma_{AB}$ [STAT, pp. 79-81].

## Related courses

- Classification application: [LINFO2262 — naive Bayes and Gaussian classifiers](../../INFO/2262/Naive%20Bayes%20and%20Gaussian%20classifiers.md)
- Follow-on decorrelation: [LELEC2870 — principal component analysis](../../ELEC/2870/Principal%20component%20analysis.md)
- Follow-on independence modeling: [LELEC2870 — independent component analysis](../../ELEC/2870/Independent%20component%20analysis.md)
- Related LDACS1110 topic: [Bayesian inference](../../DACS/1110/Bayesian%20inference.md)
- Related LDACS1110 topic: [Causal inference](../../DACS/1110/Causal%20inference.md)
- Related LDACS1110 topic: [KL divergence, cross-entropy, and mutual information](../../DACS/1110/KL%20divergence,%20cross-entropy,%20and%20mutual%20information.md)
- Related LDACS1110 topic: [Markov-chain Monte Carlo](../../DACS/1110/Markov-chain%20Monte%20Carlo.md)
- Related LDACS1110 topic: [Min-entropy and randomness extraction](../../DACS/1110/Min-entropy%20and%20randomness%20extraction.md)
- Related LDACS1110 topic: [Randomized algorithms and probabilistic analysis](../../DACS/1110/Randomized%20algorithms%20and%20probabilistic%20analysis.md)
- Related LDACS1110 topic: [Shannon entropy and conditional entropy](../../DACS/1110/Shannon%20entropy%20and%20conditional%20entropy.md)
- Control application: [LINMA2725 — linear quadratic Gaussian control](../../INMA/2725/Linear%20quadratic%20Gaussian%20control.md)
- Sequential extension: [LINMA2725 — Markov systems, invariant measures, and ergodicity](../../INMA/2725/Markov%20systems,%20invariant%20measures,%20and%20ergodicity.md)
