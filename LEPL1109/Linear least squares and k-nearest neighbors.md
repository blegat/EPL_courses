# 14. Linear least squares and k-nearest neighbors

### Multivariate linear model and least squares

- Linear model $\hat Y=X^T\hat\beta$, with optional intercept; global,
  parametric, and geometrically a fitted hyperplane [SL-1, pp. 18-19].
- Empirical squared risk
  $N^{-1}\|y-X\beta\|^2$, vertical-distance interpretation, and distinction
  from PCA's orthogonal reconstruction criterion [SL-1, p. 20].
- Assuming $N>p$ and invertible $X^TX$,
  $\hat\beta=(X^TX)^{-1}X^Ty=X^\dagger y$, fitted vector
  $\hat y=Hy$, and hat matrix $H=XX^\dagger$ [SL-1, p. 21].
- Binary LS classifier = continuous LS regression followed by thresholding;
  this yields a linear decision boundary but can produce uninterpretable values
  outside $[0,1]$ and cannot represent nonlinear class geometry [SL-1,
  pp. 22-26].

### k-nearest neighbors

- $N_k(x)$ is the set of the $k$ closest training inputs. k-NN regression
  predicts their average label, making it a local, nonparametric estimator
  [SL-1, p. 27].
- Binary k-NN classification thresholds the local average at 0.5, equivalent to
  majority voting; multiclass prediction chooses the most represented class
  [SL-1, p. 28].
- Small $k$ yields irregular, highly flexible boundaries; $k=1$ induces a
  Voronoi tessellation and zero training error. $k=N$ reduces to global
  majority voting [SL-1, pp. 29-32].
- Classification error
  $N^{-1}\sum_i1_{y_i\ne\hat f(x_i)}$; training error alone cannot select
  $k$ or establish generalization [SL-1, p. 33].
- Effective flexibility/number of parameters is approximately $N/k$, not
  $k$ [SL-1, p. 35].
- k-NN has mild explicit assumptions and nonlinear boundaries but depends on
  the distance and preprocessing, needs hyperparameter selection, and can be
  unstable at high flexibility [SL-1, pp. 34-36].
- Scikit-learn estimator workflow: instantiate a model, call `fit(X,y)`, then
  `predict(X_new)` [SL-1, p. 43].

## Related courses

- Follow-on: [LELEC2870 — linear regression, optimization, and the perceptron](../LELEC2870/Linear%20regression,%20optimization,%20and%20the%20perceptron.md)
- Related LDACS1110 topic: [VC dimension and growth functions](../LDACS1110/VC%20dimension%20and%20growth%20functions.md)
