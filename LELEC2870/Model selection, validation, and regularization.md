# Model selection, validation, and regularization

## Topics and results

- Parameters are learned within a model structure; hyperparameters select the
  structure or learning procedure (`MS`, pp. 2–9).
- AIC and BIC penalize empirical fit by complexity, but the examples emphasize
  that no selector is immune to finite-sample failure (`MS`, pp. 10–13).
- Holdout validation, repeated validation, $K$-fold cross-validation, and
  leave-one-out estimate generalization with different bias, variance, and
  computational costs (`MS`, pp. 14–22).
- Bootstrap optimism correction and the .632 bootstrap estimate performance by
  resampling with replacement and accounting for in-bag/out-of-bag behavior
  (`MS`, pp. 23–32).
- Training, validation, and final test sets have distinct roles; using the test
  set for hyperparameter selection invalidates its role as an unbiased final
  assessment (`MS`, pp. 33–39).
- Classification assessment uses confusion matrices and error consequences,
  not accuracy alone (`MS`, pp. 40–43).
- Complexity control includes weight-decay regularization and post-training
  pruning, including Optimal Brain Damage and Optimal Brain Surgeon
  approximations (`MS`, pp. 44–50).

## Related courses

- Prerequisite: [LEPL1109 — resampling, model assessment, and model selection](../LEPL1109/Resampling,%20model%20assessment,%20and%20model%20selection.md)
- Prerequisite: [LEPL1109 — bias–variance tradeoff](../LEPL1109/Bias-variance%20tradeoff%20and%20dimensionality.md)
- Theoretical extension: [LDACS1110 — MAP estimation and regularization](../LDACS1110/MAP%20estimation%20and%20regularization.md)

