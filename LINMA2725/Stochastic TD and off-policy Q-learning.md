# Stochastic TD and off-policy Q-learning

## Topics and results

- For linear value approximation, Bellman error, temporal difference, projected
  residual, and true-value distance are distinct objectives (`S9`, pp. 4–15).
- TD($\lambda$) uses eligibility traces. Under the stated ergodicity, feature,
  and step-size assumptions, its mean linear ODE is stable and the recursion
  converges to a projected solution (`S9`, pp. 16–20).
- Least-squares TD estimates the mean recursion's matrix and vector and acts as
  a stochastic Newton-type method (`S9`, p. 21).
- On-policy Q evaluation inherits the value-function argument, whereas naive
  off-policy TD can lose stability and approximation guarantees (`S9`,
  pp. 23–31).
- Tabular Q-learning converges under sufficient visitation and suitable step
  sizes, but rare state–action pairs cause poor conditioning (`S9`, pp. 32–39).
- General function approximation has fewer guarantees; GQ-learning optimizes a
  squared projected residual to restore a principled off-policy objective
  (`S9`, pp. 40–46).

## Related courses

- Deterministic derivation: [Temporal-difference learning and projected Bellman equations](Temporal-difference%20learning%20and%20projected%20Bellman%20equations.md)
- Sampling assumptions: [Markov systems, invariant measures, and ergodicity](Markov%20systems,%20invariant%20measures,%20and%20ergodicity.md)

