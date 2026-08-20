# Linear discriminants and perceptron learning

- A binary linear discriminant predicts by the sign of $w^Tx+w_0$; its decision
  boundary is a hyperplane (`LIN`, pp. 3–9).
- Gradient descent adjusts coefficients according to a differentiable empirical
  criterion (`LIN`, pp. 10–14).
- The perceptron updates only on mistakes and converges in finitely many updates
  when the sample is linearly separable (`LIN`, pp. 15–19).
- Minimum squared-error fitting provides a pseudoinverse/least-squares route,
  though squared loss is not classification-specific (`LIN`, pp. 20–23).
- Multiclass linear classification uses one discriminant per class and predicts
  the largest score (`LIN`, pp. 24–26).

## Related courses

- Algorithmic counterpart: [LELEC2870 — linear regression, optimization, and the perceptron](../LELEC2870/Linear%20regression,%20optimization,%20and%20the%20perceptron.md)
- Large-margin extension: [Support-vector machines and kernels](Support-vector%20machines%20and%20kernels.md)

