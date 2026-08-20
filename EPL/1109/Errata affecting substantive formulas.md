# 22. Errata affecting substantive formulas

The supplied errata page corrects three formulas [ERR, p. 1]:

1. The conditional-density example associated with `STAT`, p. 66 must use the
   corrected marginal density in its denominator.
2. The Gaussian-process posterior mean associated with `STAT`, p. 227 must
   include the cross-covariance vector and noisy Gram matrix:
   $k(x_*,X)^T[K+\sigma_\epsilon^2I]^{-1}y$.
3. The noisy GP observation distribution associated with `STAT`, p. 232 is
   $N(0,K+\sigma_\epsilon^2I)$.

Other apparent slide issues relevant when reusing results:

- A linear combination is guaranteed normal from **joint** normality, not merely
  from normal marginals [STAT, p. 49; APP, p. 22].
- The multivariate-normal density uses $\Sigma^{-1}$ in its quadratic form
  [STAT, p. 76; APP, p. 36].
- The p-value is not the probability that the null hypothesis is true [STAT,
  p. 152].
- PCA itself is a linear projection after fitting, despite a takeaway slide
  describing it as nonlinear [UL, p. 35]. The learned axes depend on the data,
  but the forward map is linear for centered data.
