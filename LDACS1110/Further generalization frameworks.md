# 16. Further generalization frameworks

**Status:** Defer, but legitimate advanced candidates.

### Rademacher complexity

- Empirical Rademacher complexity as data-dependent richness.
- Symmetrization and contraction lemmas.
- Bounds for real-valued losses and norm-constrained linear predictors.
- Advantage: finer than cardinality/VC in many modern classes.

### Algorithmic stability

- Replace-one sensitivity of a learning algorithm.
- Uniform stability implies an expected/high-probability generalization bound.
- Strong convexity plus regularization can create stability.

### PAC-Bayes

- Prior and data-dependent posterior over predictors.
- Bounds involving empirical Gibbs risk and
  `D_KL(posterior || prior)`.
- Attractive synthesis of Bayesian notation and frequentist high-probability
  guarantees, but not ordinary Bayesian posterior inference.

### Structural risk minimization

- Nested classes and complexity penalties.
- Select a model by balancing empirical fit and a uniform-convergence penalty.

### Why defer

Each framework requires a separate proof toolkit. Including several would
prevent adequate treatment of entropy, Bayes, and the crypto bridge.

### LEPL1109 dependency

- Empirical/expected risk, optimization, model selection, and bias-variance:
  [Parts 13-18 of the LEPL1109 map](../LEPL1109/content.md#part-b-data-science-and-machine-learning).

### Bibliography

[B7](content.md#b7), [B8](content.md#b8), [B9](content.md#b9).
