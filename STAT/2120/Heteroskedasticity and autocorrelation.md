# Heteroskedasticity and autocorrelation

When error variance varies across observations, OLS may remain unbiased under exogeneity but its usual standard errors are wrong and it is no longer efficient. Residual plots and formal tests diagnose heteroskedasticity; robust covariance estimates or weighted least squares address it.

Serial correlation similarly changes the covariance matrix. Durbin–Watson-type diagnostics, generalized least squares, feasible GLS, and heteroskedasticity-and-autocorrelation-consistent standard errors target different assumptions about dependence.

Source: `LM`, pp. 259–302.

## Related courses

- [Statistical properties of OLS](Statistical%20properties%20of%20OLS.md) identifies which classical assumptions fail.
- [Panel-data fixed and random effects](Panel-data%20fixed%20and%20random%20effects.md) handles repeated observations with structured dependence.
- [Time series and autoregressive models](../../EPL/1109/Time%20series%20and%20autoregressive%20models.md) develops temporal dependence and forecasting.
