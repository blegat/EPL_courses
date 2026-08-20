# 18. Bias-variance tradeoff and dimensionality

### Bias-variance decomposition

- Overfitting means excessive model flexibility; underfitting means too little.
  Model parameters, feature choices, and hyperparameters all affect flexibility
  [SL-3, p. 22].
- For $Y=f(X)+\epsilon$, $E\epsilon=0$, $V\epsilon=\sigma_\epsilon^2$,
  and a fitted predictor random through its training dataset, expected test
  error at $x_0$ decomposes as

$$
  EPE(x_0)=\sigma_\epsilon^2+
  (f(x_0)-E_T\hat f(x_0))^2+V_T(\hat f(x_0)).
$$

  The terms are irreducible noise, squared bias, and variance [SL-3,
  pp. 23-26].
- Typically bias decreases and variance increases with flexibility, producing a
  U-shaped test error and motivating model selection [SL-3, p. 26].
- For k-NN, flexibility is $N/k$; under fixed-neighbor simplifications the
  variance is $\sigma_\epsilon^2/k$, while increasing $k$ raises local
  averaging bias [SL-3, pp. 27-29].

### Curse of dimensionality (explicitly extra material)

- In high dimension, most volume lies near a shell/boundary and fixed local
  neighborhoods become large in each coordinate [SL-3, pp. 31-34].
- For uniform points in $[0,1]^p$, a cube of side $r$ contains about
  $r^pN$ points, so obtaining $k$ neighbors requires
  $r\approx(k/N)^{1/p}\to1$ as $p$ grows [SL-3, p. 33].
- Maintaining fixed sampling density requires sample size exponential in
  dimension; local nonparametric methods such as k-NN are especially affected
  [SL-3, pp. 32-34].
- This section is tagged **Extra Material** [SL-3, pp. 2, 30-34], so it should
  not be treated as firm examinable prerequisite knowledge.

## Related courses

- Follow-on: [LELEC2870 — machine-learning framing and dimensionality](../../ELEC/2870/Machine-learning%20framing%20and%20dimensionality.md)
- Follow-on: [LELEC2870 — feature selection](../../ELEC/2870/Feature%20selection.md)
- Follow-on: [LELEC2870 — model selection, validation, and regularization](../../ELEC/2870/Model%20selection,%20validation,%20and%20regularization.md)
- Related LDACS1110 topic: [Integrated learning applications](../../DACS/1110/Integrated%20learning%20applications.md)
- Related LDACS1110 topic: [MAP estimation and regularization](../../DACS/1110/MAP%20estimation%20and%20regularization.md)
- Related LDACS1110 topic: [PAC learning and finite-class sample complexity](../../DACS/1110/PAC%20learning%20and%20finite-class%20sample%20complexity.md)
- Related LDACS1110 topic: [Robustness, sensitivity, and distribution shift](../../DACS/1110/Robustness,%20sensitivity,%20and%20distribution%20shift.md)
