# OLS estimation and projection geometry

Ordinary least squares minimizes $\lVert y-X\beta\rVert^2$. With full column rank, $\hat\beta=(X^\top X)^{-1}X^\top y$. Fitted values are the orthogonal projection $Hy$ onto the column space of $X$, with hat matrix $H=X(X^\top X)^{-1}X^\top$; residuals $(I-H)y$ are orthogonal to every regressor.

The geometry explains normal equations, decomposition of sums of squares, leverage, and the role of rank. Centering reveals the intercept and slope structure in simple regression.

Source: `LM`, pp. 36–75.

## Related courses

- [Multicollinearity and ridge regression](Multicollinearity%20and%20ridge%20regression.md) studies ill-conditioned design geometry.
- [Outliers, leverage, and influential observations](Outliers,%20leverage,%20and%20influential%20observations.md) uses the projection's hat matrix.
- [Regression specification and interpretation](Regression%20specification%20and%20interpretation.md) defines the model being fitted.
- [Statistical properties of OLS](Statistical%20properties%20of%20OLS.md) studies the estimator under stochastic assumptions.
- [Linear regression and ANOVA](../LEPL1109/Linear%20regression%20and%20ANOVA.md) develops the same projection and variance decomposition.
