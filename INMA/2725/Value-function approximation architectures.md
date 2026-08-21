# Value-function approximation architectures

## Topics and results

- Approximate dynamic programming replaces an unrestricted value or Q-function
  by a parameterized class fitted on sampled state/action points (`CH3B`,
  pp. 7–9).
- Regularized empirical curve fitting balances sample fit against complexity;
  excessive regularization underfits (`CH3B`, pp. 8–10).
- Linear architectures reduce fitting to least squares in fixed basis features
  (`CH3B`, pp. 10–11).
- Galerkin relaxation enforces residual orthogonality to selected test functions
  rather than pointwise equality (`CH3B`, pp. 12–13).
- Neural networks provide nonlinear finite-dimensional classes (`CH3B`, p. 13).
- Reproducing-kernel methods use a rich Hilbert space; the representer theorem
  reduces regularized fitting to a finite expansion over sampled points
  (`CH3B`, pp. 14–15).

## Related courses

- Linear fitting: [LELEC2870 — linear regression, optimization, and the perceptron](../../ELEC/2870/Linear%20regression,%20optimization,%20and%20the%20perceptron.md)
- Neural approximation: [LELEC2870 — multilayer perceptrons and backpropagation](../../ELEC/2870/Multilayer%20perceptrons%20and%20backpropagation.md)
- Kernel methods: [LELEC2870 — support-vector machines and kernels](../../ELEC/2870/Support-vector%20machines%20and%20kernels.md)
- Representer theorem: [LINMA2472 — kernel ridge regression and the representer theorem](../2472/Kernel%20ridge%20regression%20and%20the%20representer%20theorem.md)
