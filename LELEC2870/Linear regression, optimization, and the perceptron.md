# Linear regression, optimization, and the perceptron

## Topics and results

- Linear regression minimizes mean squared error for $y=w^Tx$, with a bias
  absorbed by augmenting the input (`LR`, pp. 4–7).
- The normal equations give the least-squares solution through the
  pseudoinverse; the slides contrast this batch solution with gradient descent
  and stochastic gradient descent (`LR`, pp. 8–15).
- Squared loss can be used for classification but does not directly encode the
  classification objective and behaves poorly for some nonseparable data
  (`LR`, pp. 16–18).
- The perceptron uses labels in $\{-1,+1\}$ and updates on misclassified
  examples. For linearly separable data, the perceptron convergence theorem
  proves termination after finitely many mistakes; the proof bounds progress
  toward a separating vector and growth of the weight norm (`LR`, pp. 20–26).
- A single perceptron cannot represent non-linearly-separable functions such as
  XOR (`LR`, p. 31).

## Related courses

- Prerequisite: [LEPL1109 — linear least squares and k-nearest neighbors](../LEPL1109/Linear%20least%20squares%20and%20k-nearest%20neighbors.md)
- Statistical formulation: [LEPL1109 — linear regression and ANOVA](../LEPL1109/Linear%20regression%20and%20ANOVA.md)

