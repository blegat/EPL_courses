# 18. Bias-variance tradeoff and dimensionality

### Bias-variance decomposition

- Overfitting means excessive model flexibility; underfitting means too little.
  Model parameters, feature choices, and hyperparameters all affect flexibility
  [SL-3, p. 22].
- For \(Y=f(X)+\epsilon\), \(E\epsilon=0\), \(V\epsilon=\sigma_\epsilon^2\),
  and a fitted predictor random through its training dataset, expected test
  error at \(x_0\) decomposes as
  \[
  EPE(x_0)=\sigma_\epsilon^2+
  (f(x_0)-E_T\hat f(x_0))^2+V_T(\hat f(x_0)).
  \]
  The terms are irreducible noise, squared bias, and variance [SL-3,
  pp. 23-26].
- Typically bias decreases and variance increases with flexibility, producing a
  U-shaped test error and motivating model selection [SL-3, p. 26].
- For k-NN, flexibility is \(N/k\); under fixed-neighbor simplifications the
  variance is \(\sigma_\epsilon^2/k\), while increasing \(k\) raises local
  averaging bias [SL-3, pp. 27-29].

### Curse of dimensionality (explicitly extra material)

- In high dimension, most volume lies near a shell/boundary and fixed local
  neighborhoods become large in each coordinate [SL-3, pp. 31-34].
- For uniform points in \([0,1]^p\), a cube of side \(r\) contains about
  \(r^pN\) points, so obtaining \(k\) neighbors requires
  \(r\approx(k/N)^{1/p}\to1\) as \(p\) grows [SL-3, p. 33].
- Maintaining fixed sampling density requires sample size exponential in
  dimension; local nonparametric methods such as k-NN are especially affected
  [SL-3, pp. 32-34].
- This section is tagged **Extra Material** [SL-3, pp. 2, 30-34], so it should
  not be treated as firm examinable prerequisite knowledge.
