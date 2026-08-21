# Time series and autoregressive models

- Time series $(X_t)$, mean function, autocovariance
  $\gamma(t,s)$, autocorrelation $\rho(t,s)$, and interpretation of
  persistence [STAT, pp. 207-210; SUPP, pp. 2-5].
- AR($p$) model
  $X_t=\sum_{j=1}^p\beta_jX_{t-j}+\epsilon_t$, Gaussian innovations, and
  $E[X_t\mid X_{t-1:t-p}=x_{t-1:t-p}]=\sum_j\beta_jx_{t-j}$, with constant
  conditional variance [STAT, p. 211; SUPP, p. 6].
- Construction of lagged design matrix, OLS/MLE fit, innovation-variance
  estimate, and Statsmodels `AutoReg` implementation [STAT, pp. 212-214;
  SUPP, pp. 7-9].
- Recursive multi-step forecasting and degradation with forecast horizon
  because predictions become later inputs [STAT, pp. 215-216; SUPP,
  pp. 10-11].
- Lag-order/model selection: likelihood alone overfits; minimize
  $AIC=2p-2\log L$ or $BIC=(\log n)p-2\log L$. BIC penalizes complexity
  more strongly [STAT, pp. 217-220; SUPP, pp. 12-15].
- Partial autocorrelation is explicitly mentioned but not covered [STAT,
  p. 220; SUPP, p. 15].

## Related courses

- Regression diagnostics: [LSTAT2120 — heteroskedasticity and autocorrelation](../../STAT/2120/Heteroskedasticity%20and%20autocorrelation.md)
- Related LDACS1110 topic: [Markov-chain Monte Carlo](../../DACS/1110/Markov-chain%20Monte%20Carlo.md)
- Sequential extension: [LINMA2725 — Poisson equations and stochastic cost criteria](../../INMA/2725/Poisson%20equations%20and%20stochastic%20cost%20criteria.md)
