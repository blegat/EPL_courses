# Feature selection

## Topics and results

- High-dimensional data aggravate norm/distance concentration, sample scarcity,
  computation, and interpretability; feature selection retains original
  variables, unlike feature extraction (`FS`, pp. 2–16).
- Filter criteria assess relevance independently of a predictor. The course
  studies linear correlation and mutual information, including the difference
  between linear association and general dependence (`FS`, pp. 17–38).
- Wrapper methods score subsets through a learning algorithm and validation;
  they capture interactions but cost more and can overfit the selection process
  (`FS`, pp. 39–46).
- Forward selection, backward elimination, and floating/greedy searches replace
  infeasible exhaustive enumeration, without guaranteeing the globally best
  subset (`FS`, pp. 47–61).
- Embedded selection incorporates sparsity or importance into fitting; examples
  connect regularization and model-specific selection (`FS`, pp. 62–70).
- Case studies illustrate selection for regression, business-plan data,
  spectroscopy, and lagged time-series inputs (`FS`, pp. 71–84).

## Related courses

- Motivation: [LEPL1109 — bias–variance and dimensionality](../../EPL/1109/Bias-variance%20tradeoff%20and%20dimensionality.md)
- Information criterion: [LDACS1110 — KL divergence and mutual information](../../DACS/1110/KL%20divergence,%20cross-entropy,%20and%20mutual%20information.md)
- Regularization: [LDACS1110 — MAP estimation and regularization](../../DACS/1110/MAP%20estimation%20and%20regularization.md)
