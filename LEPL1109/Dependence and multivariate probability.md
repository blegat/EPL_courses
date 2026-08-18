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

- Follow-on decorrelation: [LELEC2870 — principal component analysis](../LELEC2870/Principal%20component%20analysis.md)
- Follow-on independence modeling: [LELEC2870 — independent component analysis](../LELEC2870/Independent%20component%20analysis.md)
