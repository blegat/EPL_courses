# Statistical properties of OLS

Under exogeneity, $E(\varepsilon\mid X)=0$, OLS is conditionally unbiased. Under homoskedastic uncorrelated errors, $\operatorname{Var}(\hat\beta\mid X)=\sigma^2(X^\top X)^{-1}$, and the Gauss–Markov theorem makes OLS the best linear unbiased estimator.

Consistency and asymptotic normality support large-sample inference under weaker conditions. The residual sum of squares yields an unbiased variance estimate after accounting for fitted degrees of freedom.

Source: `LM`, pp. 76–99.

## Related courses

- [Heteroskedasticity and autocorrelation](Heteroskedasticity%20and%20autocorrelation.md) studies failures of the classical covariance assumptions.
- [OLS estimation and projection geometry](OLS%20estimation%20and%20projection%20geometry.md) provides the algebraic estimator.
- [Maximum likelihood in Gaussian linear models](Maximum%20likelihood%20in%20Gaussian%20linear%20models.md) adds a full distributional assumption.
- [Parametric estimation](../LEPL1109/Parametric%20estimation.md) places unbiasedness, efficiency, and consistency in a general framework.
