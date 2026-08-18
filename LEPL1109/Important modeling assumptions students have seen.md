# 21. Important modeling assumptions students have seen

- Independence/i.i.d. sampling is required in many likelihood, CLT, bootstrap,
  and testing derivations.
- Normal-population assumptions underlie exact Student, chi-square, Fisher,
  regression, and ANOVA inference.
- The pooled two-sample Student procedure assumes equal variances.
- Linear regression's closed-form inverse assumes full column rank; LS remains
  usable without Gaussian errors, but its exact MLE and finite-sample inference
  interpretations change [STAT, pp. 172-174].
- Logistic likelihood assumes independent conditional Bernoulli outcomes
  [SL-2, p. 33].
- Random train/validation splitting presumes exchangeable observations; naive
  randomization is not automatically valid for dependent/time-ordered data even
  though the introductory CV discussion uses randomization [SL-2, p. 9].
- PCA is scale-sensitive and ordinarily needs centering; standardization is a
  modeling decision, not a universal requirement [UL, p. 23].
- k-NN and K-means depend critically on distance, feature scaling, dimension,
  and geometry [SL-1, p. 34; UL, pp. 62-63].
