# Further generalization frameworks

## Rademacher complexity

- Empirical Rademacher complexity as data-dependent richness.
- Symmetrization and contraction lemmas.
- Bounds for real-valued losses and norm-constrained linear predictors.
- Advantage: finer than cardinality/VC in many modern classes.

## Algorithmic stability

- Replace-one sensitivity of a learning algorithm.
- Uniform stability implies an expected/high-probability generalization bound.
- Strong convexity plus regularization can create stability.

## PAC-Bayes

- Prior and data-dependent posterior over predictors.
- Bounds involving empirical Gibbs risk and
  $D_{\\mathrm{KL}}(\\text{posterior}\\,\\|\\,\\text{prior})$.
- Attractive synthesis of Bayesian notation and frequentist high-probability
  guarantees, but not ordinary Bayesian posterior inference.

## Structural risk minimization

- Nested classes and complexity penalties.
- Select a model by balancing empirical fit and a uniform-convergence penalty.

## Prerequisites from LEPL1109

- Empirical and expected risk and optimization:
  [LEPL1109 — supervised-learning formulation](../../EPL/1109/Supervised-learning%20formulation.md).
- Validation and model selection:
  [LEPL1109 — resampling, model assessment, and model selection](../../EPL/1109/Resampling,%20model%20assessment,%20and%20model%20selection.md).
- Bias-variance and dimensionality:
  [LEPL1109 — bias-variance tradeoff and dimensionality](../../EPL/1109/Bias-variance%20tradeoff%20and%20dimensionality.md).

## References

[B7](README.md#b7), [B8](README.md#b8), [B9](README.md#b9).
