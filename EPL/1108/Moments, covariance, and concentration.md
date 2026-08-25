# Moments, covariance, and concentration

## Topics and results

- Raw and centered moments summarize a distribution, with
  $\operatorname{Var}(X)=E[X^2]-E[X]^2$ (`P3`, pp. 28–35; `P4`, pp. 3–9).
- Covariance gives
  $$\operatorname{Var}\!\left(\sum_iX_i\right)=
  \sum_i\operatorname{Var}(X_i)+2\sum_{i<j}\operatorname{Cov}(X_i,X_j).$$
- Independence implies zero covariance, but not conversely; correlation is
  normalized covariance (`P4`, pp. 10–18).
- Markov bounds a nonnegative variable by its mean, while Chebyshev gives
  $$P(|X-E[X]|\geq t)\leq\operatorname{Var}(X)/t^2.$$
- Applying Chebyshev to an empirical mean yields a weak law of large numbers
  (`P4`, pp. 19–25).

## Related courses

- Stronger finite-sample bounds: [LDACS1110 — concentration inequalities](../../DACS/1110/Concentration%20inequalities.md)

## Internal connections

- [Generating functions and linear recurrences](Generating%20functions%20and%20linear%20recurrences.md)
- [Random variables, distributions, and expectation](Random%20variables%2C%20distributions%2C%20and%20expectation.md)
- [Central limit theorem and normal approximation](Central%20limit%20theorem%20and%20normal%20approximation.md)
