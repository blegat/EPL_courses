# 12. Causal inference

**Status:** Optional; Defer a complete treatment.

### Concepts

- Association, prediction, and causation are different questions.
- Confounders, colliders, mediators, and causal DAGs.
- Observational conditioning `P(Y|X=x)` versus intervention
  `P(Y|do(X=x))`.
- Randomized experiments and exchangeability.
- Identifiability depends on causal assumptions not recoverable from the joint
  observational distribution alone.
- Distribution shift can invalidate associational predictors.

### Candidate results

**Back-door adjustment.** Under the back-door criterion for an adjustment set
`Z`,

$$
P(Y\mid do(X=x))
=\sum_z P(Y\mid X=x,Z=z)P(Z=z).
$$

**Randomization.** In an ideal randomized experiment, treatment assignment is
independent of potential outcomes, identifying average treatment effects from
group contrasts under consistency and positivity assumptions.

**Observational non-identifiability.** Different causal graphs can induce the
same observational distribution while predicting different intervention
effects.

### LEPL1109 dependency

- Conditional distributions and regression:
  [dependence and multivariate probability](../../EPL/1109/Dependence%20and%20multivariate%20probability.md)
  and [linear regression](../../EPL/1109/Linear%20regression%20and%20ANOVA.md).
- Correlation captures linear association, not causation:
  [independence covariance correlation](../../EPL/1109/Dependence%20and%20multivariate%20probability.md#independence-covariance-and-correlation)
  [STAT, pp. 38-45].

### What is new beyond LEPL1109

- Intervention semantics, causal assumptions, DAGs, and identification.

### FoC reuse

Negligible for the listed cryptography topics. A single motivating example is
realistic; a responsible causal-inference module requires more time.

### Bibliography

[B10, Chs. 9-10](README.md#b10), [B13](README.md#b13), [B14](README.md#b14).
