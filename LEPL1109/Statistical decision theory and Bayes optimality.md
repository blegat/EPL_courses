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
