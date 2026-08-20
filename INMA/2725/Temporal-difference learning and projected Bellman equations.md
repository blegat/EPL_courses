# Temporal-difference learning and projected Bellman equations

## Topics and results

- Temporal difference replaces a conditional Bellman residual by a sampled
  one-step transition under a fixed policy (`CH3B`, pp. 20–23).
- Linear least-squares TD estimates parameters from trajectory features without
  separately estimating every conditional expectation (`CH3B`, pp. 23–25).
- Projected Bellman equations seek a fixed point after projection onto the
  approximation class; Galerkin orthogonality provides the finite equations
  (`CH3B`, pp. 26–30).
- Eligibility traces in TD($\lambda$) interpolate between one-step bootstrapping
  and longer returns (`CH3B`, pp. 29–34).
- On-policy Q evaluation extends the same construction to state–action values;
  stability and approximation quality depend on sampling and features
  (`CH3B`, pp. 31–34).

## Related courses

- Foundation: [Bellman equations, value iteration, and policy iteration](Bellman%20equations,%20value%20iteration,%20and%20policy%20iteration.md)
- Statistical regression precursor: [LEPL1109 — linear least squares and k-nearest neighbors](../../EPL/1109/Linear%20least%20squares%20and%20k-nearest%20neighbors.md)
- Stochastic and off-policy extension: [Stochastic TD and off-policy Q-learning](Stochastic%20TD%20and%20off-policy%20Q-learning.md)
