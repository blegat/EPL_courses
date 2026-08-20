# 13. Supervised-learning formulation

### Data, objectives, and preprocessing

- Notation: dataset size $N$, feature dimension $p$, design matrix
  $X\in R^{N\times p}$, random variables versus observations, indicators,
  i.i.d. samples, and basic set notation [SL-1, pp. 4-7].
- Regression predicts ordered/quantitative values; classification predicts
  qualitative categories. The boundary is application-dependent [SL-1,
  pp. 8-9].
- One-hot/dummy encoding maps $K$ categories to canonical vectors in
  $R^K$ [SL-1, p. 10].
- Features/input/predictors/covariates $X$ and outcome/label/response $Y$;
  noisy functional relation for regression and noisy labels for binary
  classification [SL-1, p. 11].
- Goal: learn a reliable $\hat f$ from representative samples. Training
  consists of obtaining data, preprocessing, and fitting; prediction must apply
  **exactly the same preprocessing** to independent data [SL-1, pp. 12-13].
- Preprocessing examples: missing-value imputation, categorical encoding,
  outlier handling, normalization/standardization, and dimensionality reduction
  [SL-1, p. 13].

### Models, losses, and accuracy

- Learning algorithm $A$ selects $\hat f\in\mathcal F_\gamma$ from a
  restricted function class; assumptions are unavoidable. Parameters $\beta$
  are fitted, while hyperparameters $\gamma$ characterize the model class
  [SL-1, pp. 38-39; SL-2, p. 3].
- Parametric models: explicit finite parameterization, loss minimization,
  feature maps $\phi(X)$, linear and logistic examples, optimization and
  overfitting/interpretability tradeoffs [SL-1, pp. 39-40].
- Nonparametric models: no explicit finite functional form but implicit
  assumptions, e.g. piecewise-constant behavior for k-NN; more flexibility and
  data demand, hyperparameter selection, and weaker interpretability [SL-1,
  pp. 41-42].
- Squared-loss risk for fixed $\hat f,X$ decomposes into reducible model error
  $[f(X)-\hat f(X)]^2$ and irreducible noise variance [SL-1, p. 44].
- Empirical MSE for regression and 0/1 misclassification rate for
  classification are training empirical risks [SL-1, p. 45].
- Training error may be zero while unseen-data risk is not; this is overfitting.
  Test error typically has a U-shape versus flexibility, motivating validation
  [SL-1, pp. 46-47].

## Related courses

- Classification survey: [LINFO2262 — learning problems and classification workflow](../../INFO/2262/Learning%20problems%20and%20classification%20workflow.md)
- Follow-on: [LELEC2870 — machine-learning framing and dimensionality](../../ELEC/2870/Machine-learning%20framing%20and%20dimensionality.md)
- Related LDACS1110 topic: [MAP estimation and regularization](../../DACS/1110/MAP%20estimation%20and%20regularization.md)
- Related LDACS1110 topic: [PAC learning and finite-class sample complexity](../../DACS/1110/PAC%20learning%20and%20finite-class%20sample%20complexity.md)
- Related LDACS1110 topic: [Sample compression and description length](../../DACS/1110/Sample%20compression%20and%20description%20length.md)
- Related LDACS1110 topic: [VC dimension and growth functions](../../DACS/1110/VC%20dimension%20and%20growth%20functions.md)
