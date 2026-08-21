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
  `D_KL(posterior || prior)`.
- Attractive synthesis of Bayesian notation and frequentist high-probability
  guarantees, but not ordinary Bayesian posterior inference.

## Structural risk minimization

- Nested classes and complexity penalties.
- Select a model by balancing empirical fit and a uniform-convergence penalty.

## Prerequisites from LEPL1109

- Empirical/expected risk, optimization, model selection, and bias-variance:
  [Parts 13-18 of the LEPL1109 map](../../EPL/1109/README.md#part-b-data-science-and-machine-learning).

## References

[B7](README.md#b7), [B8](README.md#b8), [B9](README.md#b9).
