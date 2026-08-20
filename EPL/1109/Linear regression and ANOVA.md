# 9. Linear regression and ANOVA

### Multiple and simple linear regression

- Multiple Gaussian linear model
  $Y_i=\beta_0+\sum_{j=1}^d\beta_jx_{ij}+\epsilon_i$,
  $\epsilon\sim N(0,\sigma^2I)$, matrix form $Y=X\beta+\epsilon$,
  conditional mean, and response distribution [STAT, pp. 167-172].
- Gaussian MLE is equivalent to least squares. OLS minimizes
  $(y-X\beta)^T(y-X\beta)$, giving normal equations and
  $\hat\beta=(X^TX)^{-1}X^Ty$ when invertible [STAT, pp. 172-173].
- Fitted values, residuals, and hat matrix
  $H=X(X^TX)^{-1}X^T$, including symmetry and idempotence [STAT, p. 174].
- Simple-regression slope/intercept formulas and interpretation [STAT,
  pp. 175-176].
- Total, residual, and regression sums of squares:
  $SST=SSE+SSR$, and $R^2=SSR/SST=1-SSE/SST$ as explained-variance
  proportion [STAT, pp. 177-179].

### Regression inference

- Sampling law
  $\hat\beta\sim N(\beta,\sigma^2(X^TX)^{-1})$, unbiasedness, and residual
  variance estimator $\hat\sigma^2=SSE/[n-(d+1)]$ [STAT, pp. 181-182].
- Residual chi-square law
  $SSE/\sigma^2\sim\chi^2_{n-d-1}$ [STAT, pp. 182-185].
- Variance and covariance of simple-regression coefficients and plug-in
  standard errors [STAT, pp. 186-187].
- Global significance test
  $H_0:\beta_1=\cdots=\beta_d=0$ with
  $F^*=(SSR/d)/(SSE/(n-d-1))\sim F_{d,n-d-1}$ [STAT, pp. 188-190].
- Individual coefficient Student tests and confidence intervals based on the
  diagonal of $(X^TX)^{-1}$ [STAT, pp. 191-197].
- New-response prediction interval in simple regression, including variance
  $\hat\sigma^2[1+1/n+(x_0-\bar x)^2/S_{xx}]$ [STAT, p. 198].
- Statsmodels use and interpretation of estimates, standard errors, t-tests,
  p-values, confidence intervals, and model summaries [STAT, pp. 187, 195-197].

### One-factor ANOVA

- Equality-of-means testing for multiple independent normal populations,
  motivation versus repeated pairwise tests, and expression as categorical
  linear regression [STAT, pp. 199-204].
- Dummy encoding with one reference category; including an intercept and every
  dummy causes rank deficiency [STAT, pp. 202-203].
- Assumptions: normality, equal within-group variances, and independence.
  Equality of group means is tested by the regression global F-test; Bartlett's
  test can assess equal variances [STAT, pp. 204-205].

## Related courses

- Linear-model specification: [LSTAT2120 — regression specification and interpretation](../../STAT/2120/Regression%20specification%20and%20interpretation.md)
- Geometric development: [LSTAT2120 — OLS estimation and projection geometry](../../STAT/2120/OLS%20estimation%20and%20projection%20geometry.md)
- Categorical-model development: [LSTAT2120 — dummy variables, interactions, and ANOVA](../../STAT/2120/Dummy%20variables,%20interactions,%20and%20ANOVA.md)
- Algorithmic continuation: [LELEC2870 — linear regression, optimization, and the perceptron](../../ELEC/2870/Linear%20regression,%20optimization,%20and%20the%20perceptron.md)
- Related LDACS1110 topic: [Causal inference](../../DACS/1110/Causal%20inference.md)
- Related LDACS1110 topic: [MAP estimation and regularization](../../DACS/1110/MAP%20estimation%20and%20regularization.md)
- Control application: [LINMA2725 — linear quadratic regulation and Riccati equations](../../INMA/2725/Linear%20quadratic%20regulation%20and%20Riccati%20equations.md)
