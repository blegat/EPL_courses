# Robustness, sensitivity, and distribution shift

## Topics and results

- Distinguish robustness to sampling variation, outliers/contamination,
  covariate shift, label noise, and adversarial perturbations.
- Use the LEPL1109 bias-variance decomposition to explain sensitivity to the
  sampled training set, but not as a complete theory of distribution shift.
- A bounded-loss Hoeffding guarantee is distribution-specific: it controls
  generalization under i.i.d. train/test sampling from the same population.
- If training and test distributions differ, for any loss bounded in `[0,1]`,
  total variation gives

$$
|E_P[\ell]-E_Q[\ell]|\leq\Delta(P,Q).
$$

- Regularization can reduce parameter sensitivity and variance, but does not by
  itself guarantee robustness to arbitrary distribution shift or adversarial
  examples.
- Robust summaries or losses can limit outlier influence; one practical may
  compare squared loss with absolute or Huber loss under contamination.
- Model-selection uncertainty should be acknowledged: repeated reuse of a
  validation set can overfit that set, and the untouched test set remains an
  estimate rather than a proof under future shift.

## Prerequisites from LEPL1109

- Bias-variance and model selection:
  [bias-variance](../../EPL/1109/Bias-variance%20tradeoff%20and%20dimensionality.md)
  and [resampling](../../EPL/1109/Resampling,%20model%20assessment,%20and%20model%20selection.md).
- Outliers and preprocessing:
  [descriptive statistics](../../EPL/1109/Descriptive%20statistics%20and%20exploratory%20data%20analysis.md).
- LEPL1109 does not develop distribution shift, robust statistics, or
  adversarial robustness; those qualifications are new.

## Connection to foundations of cryptography

The distinction between empirical robustness and worst-case security is
important: robustness experiments do not establish a cryptographic security
property quantified over efficient adversaries.

## References

[B10](README.md#b10), [B7](README.md#b7), [B12](README.md#b12).

## Related courses

- Applied framing: [LELEC2870 — machine-learning framing and dimensionality](../../ELEC/2870/Machine-learning%20framing%20and%20dimensionality.md)
- Modern-model context: [LELEC2870 — deep learning architectures and training](../../ELEC/2870/Deep%20learning%20architectures%20and%20training.md)
