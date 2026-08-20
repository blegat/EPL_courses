# Fluid models, policy sensitivity, and score functions

## Topics and results

- A fluid model removes or averages stochastic disturbances to obtain a
  deterministic approximation useful for intuition, stability, and candidate
  controls (`S67`, pp. 48–51).
- The approximation can suggest optimal behavior, but transferring optimality
  to the stochastic system requires explicit error/stability arguments (`S67`,
  pp. 49–51).
- Parameterized randomized policies induce parameterized closed-loop kernels
  and invariant distributions (`S67`, pp. 52–53).
- The likelihood-ratio/score function differentiates expectations without
  differentiating sampled outcomes directly (`S67`, p. 53).
- The sensitivity theorem expresses the gradient of average cost using the
  score and a Q/relative-value quantity, motivating policy-gradient methods
  (`S67`, pp. 54–55).

## Related courses

- Estimation precursor: [LEPL1109 — parametric estimation](../LEPL1109/Parametric%20estimation.md)
- Follow-on: [Advantage functions and actor-critic methods](Advantage%20functions%20and%20actor-critic%20methods.md)

