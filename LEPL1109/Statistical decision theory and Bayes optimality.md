# 17. Statistical decision theory and Bayes optimality

- Inputs and outputs are jointly distributed, and the training set is an i.i.d.
  sample from this population [SL-3, pp. 3-4].
- Losses: squared error, 0/1 loss, and logistic/cross-entropy loss [SL-3, p. 5].
- Expected risk $R(f)=E[\ell(Y,f(X))]$, empirical risk, and convergence of
  empirical to expected risk under suitable conditions [SL-3, p. 6].
- Marginal and conditional distributions, Bayes' rule, and law of total
  expectation are reviewed to express risk as expected conditional risk
  [SL-3, pp. 7-11].
- Bayes predictor
  $f^*(x)=\arg\min_zE[\ell(Y,z)\mid X=x]$, Bayes risk $R^*=R(f^*)$,
  lower bound $R(f)\ge R^*$, and excess risk $R(f)-R^*$ [SL-3, p. 12].
- For squared loss, the Bayes regressor is the conditional mean
  $f^*(x)=E[Y\mid X=x]$; the deck proves this by completing the square
  [SL-3, pp. 13-14].
- k-NN approximates that conditional mean by sample averaging in a neighborhood.
  Consistency is stated for $N,k\to\infty$, $k/N\to0$, under regularity
  conditions [SL-3, p. 15].
- For 0/1 loss, the Bayes classifier selects the most probable conditional
  class: $f^*(x)=\arg\max_gP(Y=g\mid X=x)$, with
  $R^*=1-E_X\max_gP(Y=g\mid X)$ [SL-3, p. 16].
- A Gaussian-mixture example derives the Bayes decision by comparing
  class-conditional density times prior and compares it with k-NN [SL-3,
  pp. 17-19].

## Related courses

- Classification extension: [LINFO2262 — Bayesian decision theory, MAP, and ROC analysis](../LINFO2262/Bayesian%20decision%20theory,%20MAP,%20and%20ROC%20analysis.md)
- Related LDACS1110 topic: [Bayesian inference](../LDACS1110/Bayesian%20inference.md)
- Related LDACS1110 topic: [Fano's inequality and information-theoretic lower bounds](<../LDACS1110/Fano's%20inequality%20and%20information-theoretic%20lower%20bounds.md>)
- Related LDACS1110 topic: [Monte Carlo estimation](../LDACS1110/Monte%20Carlo%20estimation.md)
- Related LDACS1110 topic: [PAC learning and finite-class sample complexity](../LDACS1110/PAC%20learning%20and%20finite-class%20sample%20complexity.md)
- Related LDACS1110 topic: [Sample compression and description length](../LDACS1110/Sample%20compression%20and%20description%20length.md)
- Sequential extension: [LINMA2725 — Bellman equations, value iteration, and policy iteration](../LINMA2725/Bellman%20equations,%20value%20iteration,%20and%20policy%20iteration.md)
- Sequential extension: [LINMA2725 — stochastic optimal control and dynamic programming](../LINMA2725/Stochastic%20optimal%20control%20and%20dynamic%20programming.md)
