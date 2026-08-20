# Maximum likelihood in Gaussian linear models

If $y\mid X\sim\mathcal N(X\beta,\sigma^2I)$, maximizing the likelihood over $\beta$ is equivalent to minimizing the residual sum of squares, so the MLE of $\beta$ equals OLS. The likelihood estimate of $\sigma^2$ uses divisor $n$, whereas the unbiased estimator uses $n-p$.

Normality provides exact finite-sample distributions and likelihood-ratio reasoning, while asymptotic likelihood theory motivates inference beyond the Gaussian case.

Source: `LM`, pp. 100–122.

## Related courses

- [Statistical properties of OLS](Statistical%20properties%20of%20OLS.md) states the moment-based guarantees that do not require normality.
- [Inference, confidence regions, and hypothesis tests](Inference,%20confidence%20regions,%20and%20hypothesis%20tests.md) uses the resulting sampling distributions.
- [Parametric estimation](../../EPL/1109/Parametric%20estimation.md) develops maximum likelihood generally.
