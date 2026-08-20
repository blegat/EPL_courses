# Multicollinearity and ridge regression

Exact collinearity makes coefficients unidentified; near collinearity leaves OLS defined but inflates variances and makes individual coefficients unstable. Condition numbers, variance inflation factors, and auxiliary regressions diagnose the geometry.

Ridge regression minimizes $\lVert y-X\beta\rVert^2+\lambda\lVert\beta\rVert^2$. Its shrinkage introduces bias to reduce variance and stabilize prediction, so scaling and tuning $\lambda$ are essential.

Source: `LM`, pp. 180–196.

## Related courses

- [OLS estimation and projection geometry](OLS%20estimation%20and%20projection%20geometry.md) explains why nearly dependent columns cause instability.
- [Variable and model selection](Variable%20and%20model%20selection.md) compares regularization with choosing a subset of regressors.

