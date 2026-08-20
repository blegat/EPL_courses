# 11. MAP estimation and regularization

**Status:** Core.

### Candidate results

**MAP objective.**

$$
\widehat\theta_{MAP}
=\arg\min_\theta
\{-\log p(x\mid\theta)-\log p(\theta)\}.
$$

**Gaussian prior to ridge penalty.** If
`theta ~ N(0,tau^2 I)`, then the negative log-prior contributes
`||theta||_2^2/(2 tau^2)`.

**Laplace prior to lasso penalty.** An independent Laplace prior contributes an
`L1` penalty proportional to `||theta||_1`.

**Gaussian linear model.** Gaussian likelihood plus Gaussian prior yields a
quadratic posterior and a regularized least-squares posterior mean/MAP.

**MAP limitation.** MAP is not invariant under arbitrary reparameterization and
does not retain posterior uncertainty; it should not be equated with full
Bayesian prediction.

### LEPL1109 dependency

- MLE and negative log-likelihood:
  [maximum likelihood](../../EPL/1109/Parametric%20estimation.md#maximum-likelihood)
  [STAT, pp. 97-105].
- OLS and model flexibility:
  [linear regression and ANOVA](../../EPL/1109/Linear%20regression%20and%20ANOVA.md)
  and [supervised-learning formulation](../../EPL/1109/Supervised-learning%20formulation.md).
- Bias-variance tradeoff:
  [bias-variance](../../EPL/1109/Bias-variance%20tradeoff%20and%20dimensionality.md)
  [SL-3, pp. 22-29].

### What is new beyond LEPL1109

- Prior-penalty equivalence and an explicit probabilistic view of
  regularization.
- Distinction between posterior uncertainty, MAP, and regularized ERM.

### FoC reuse

Low direct dependence, but this topic is central to the official learning
outcome linking uncertainty, regularization, and generalization.

### Bibliography

[B6](README.md#b6), [B8](README.md#b8), [B9](README.md#b9).

### Related courses

- Applied continuation: [LELEC2870 — model selection, validation, and regularization](../../ELEC/2870/Model%20selection,%20validation,%20and%20regularization.md)
- Applied continuation: [LELEC2870 — feature selection](../../ELEC/2870/Feature%20selection.md)
- Imaging application: [LELEC2885 — sparse representations and inverse problems](../../ELEC/2885/Sparse%20representations%20and%20inverse%20problems.md)
